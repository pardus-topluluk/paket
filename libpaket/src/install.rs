use std::cmp::Ordering;
use std::fs::{self, DirEntry, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::toml_structs::paket_toml::PackageType;
use crate::toml_structs::paket_toml::{self, Config};
use flate2::write::GzEncoder;
use flate2::Compression;
use once_cell::sync::Lazy;
use tar::{Archive, Entries};
use toml;

use crate::{PaketError, Result};

use semver::Version;

pub enum PaketInstalledStatus {
    No,
    LowerVersion,
    HigherVersion,
    SameVersion,
}

// === Paths ===
static BASE_PAKET_FOLDER: Lazy<PathBuf> = Lazy::new(|| Path::new("/var/lib/paket").to_path_buf());
static INSTALLED_PAKETS_FOLDER: Lazy<PathBuf> = Lazy::new(|| BASE_PAKET_FOLDER.join("installed"));

static INSTALLED_APPLICATIONS_FOLDER: Lazy<PathBuf> =
    Lazy::new(|| INSTALLED_PAKETS_FOLDER.join("application"));
static INSTALLED_LIBRARY_FOLDER: Lazy<PathBuf> =
    Lazy::new(|| INSTALLED_PAKETS_FOLDER.join("library"));
static INSTALLED_DEVELOPMENT_LIBRARY_FOLDER: Lazy<PathBuf> =
    Lazy::new(|| INSTALLED_PAKETS_FOLDER.join("development_library"));
static INSTALLED_SCRIPT_FOLDER: Lazy<PathBuf> =
    Lazy::new(|| INSTALLED_PAKETS_FOLDER.join("script"));
static INSTALLED_CONFIGURATIONS_FOLDER: Lazy<PathBuf> =
    Lazy::new(|| INSTALLED_PAKETS_FOLDER.join("configuration"));
static INSTALLED_APPLICATION_SOURCE_CODES_FOLDER: Lazy<PathBuf> =
    Lazy::new(|| INSTALLED_PAKETS_FOLDER.join("application_source_code"));
static INSTALLED_LIBRARY_SOURCE_CODES_FOLDER: Lazy<PathBuf> =
    Lazy::new(|| INSTALLED_PAKETS_FOLDER.join("library_source_code"));

// === Functions ===
pub fn unlock_paket_operations() -> Result<()> {
    Ok(fs::remove_file(BASE_PAKET_FOLDER.join("lock"))?)
}

pub fn lock_paket_operations() -> Result<()> {
    let _created_file = File::create(BASE_PAKET_FOLDER.join("lock"))?;

    Ok(())
}

fn read_paket_toml_inside_tar(entries: Entries<'_, File>) -> Result<Config> {
    let paket_toml = entries.filter_map(|entry| entry.ok()).find(|entry| {
        entry
            .path()
            .is_ok_and(|f| f.file_name().unwrap() == "Paket.toml")
    });

    let mut paket_toml = match paket_toml {
        Some(p) => p,
        None => return Err(PaketError::FileNotFound(String::from("Paket.toml"))),
    };

    let mut toml_content = String::new();

    paket_toml.read_to_string(&mut toml_content)?;

    toml::from_str(&toml_content).map_err(|e| PaketError::TomlParseError(e.message().to_string()))
}

fn check_installed_version(config: &Config) -> PaketInstalledStatus {
    let installed_folder = match config.package.package_type {
        PackageType::Application => INSTALLED_APPLICATIONS_FOLDER,
        PackageType::Script => INSTALLED_SCRIPT_FOLDER,

        PackageType::Library => INSTALLED_LIBRARY_FOLDER,
        PackageType::DevelopmentLibrary => INSTALLED_DEVELOPMENT_LIBRARY_FOLDER,

        PackageType::ApplicationSourceCode => INSTALLED_APPLICATION_SOURCE_CODES_FOLDER,
        PackageType::LibrarySourceCode => INSTALLED_LIBRARY_SOURCE_CODES_FOLDER,

        PackageType::Configuration => INSTALLED_CONFIGURATIONS_FOLDER,
    };

    let installed_filepath = installed_folder.join(config.get_paket_archive_name());
    if installed_filepath.exists() {
        // Paket is already installed!
        return PaketInstalledStatus::SameVersion;
    }

    // Check version difference with installed paket:
    let paket_basename = format!("{}_", &config.package.name);
    let mut pakets_with_same_basename = fs::read_dir(INSTALLED_CONFIGURATIONS_FOLDER.as_path())
        .unwrap()
        .filter_map(|f| match f {
            Ok(e) => {
                let filename = e.file_name().to_string_lossy().to_string();
                if filename.contains(&paket_basename) {
                    Some(filename)
                } else {
                    None
                }
            }
            Err(e) => None,
        });

    if let Some(p) = pakets_with_same_basename.next() {
        // There is a package with the same name.
        let version = p.split('_').last().unwrap(); // abc_1.0.0.paket -> 1.0.0.paket
        let version = &version[..version.len() - 6]; // 1.0.0.paket -> 1.0.0

        let currently_installed_version = Version::parse(version).unwrap();
        let new_version = Version::parse(&config.package.version).unwrap();

        match currently_installed_version.cmp(&new_version) {
            Ordering::Less => PaketInstalledStatus::LowerVersion,
            Ordering::Equal => PaketInstalledStatus::SameVersion,
            Ordering::Greater => PaketInstalledStatus::HigherVersion,
        }
    } else {
        PaketInstalledStatus::No
    }
}

pub fn install_paket(paket_path: &Path) -> Result<()> {
    // Read base .tar archive:
    let file = File::open(paket_path)?;
    let mut ar = Archive::new(file);
    let entries = ar.entries()?;

    // Get the valid Paket.toml
    let config: Config = read_paket_toml_inside_tar(entries)?;

    let installed_status = check_installed_version(&config);

    match installed_status {
        PaketInstalledStatus::No | PaketInstalledStatus::LowerVersion => {
            // Install
        }
        PaketInstalledStatus::HigherVersion | PaketInstalledStatus::SameVersion => {
            // Do nothing.
        }
    }

    Ok(())
}
