use std::cmp::Ordering;
use std::fs::{self, DirEntry, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::toml_structs::paket_toml::{self, Config};
use crate::toml_structs::paket_toml::{read_config_from_toml, PackageType};
use flate2::write::GzEncoder;
use flate2::Compression;
use once_cell::sync::Lazy;
use tar::{Archive, Entries};
use toml;

use crate::{PaketError, Result};

use semver::Version;

#[derive(Debug)]
pub enum PaketExistance {
    NotExists,
    LowerVersionInstalled,
    HigherVersionInstalled,
    SameVersionInstalled,
}
pub enum DependencyStatus {
    Valid,
    NotValid(String),
}

pub struct InstallInformation {
    paket_existance_status: PaketExistance,
    dependency_status: DependencyStatus,
}

// === Static Paths ===
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

fn check_installed_version(config: &Config) -> Result<PaketExistance> {
    let installed_folder = match config.package.package_type {
        PackageType::Application => &INSTALLED_APPLICATIONS_FOLDER,
        PackageType::Script => &INSTALLED_SCRIPT_FOLDER,

        PackageType::Library => &INSTALLED_LIBRARY_FOLDER,
        PackageType::DevelopmentLibrary => &INSTALLED_DEVELOPMENT_LIBRARY_FOLDER,

        PackageType::ApplicationSourceCode => &INSTALLED_APPLICATION_SOURCE_CODES_FOLDER,
        PackageType::LibrarySourceCode => &INSTALLED_LIBRARY_SOURCE_CODES_FOLDER,

        PackageType::Configuration => &INSTALLED_CONFIGURATIONS_FOLDER,
    };

    let installed_filepath = installed_folder.join(config.get_paket_archive_name());
    if installed_filepath.exists() {
        // Same Paket is already installed!
        return Ok(PaketExistance::SameVersionInstalled);
    }

    // Check version difference with installed paket:
    let paket_basename = format!("{}_", &config.package.name);
    let mut paket_with_same_basename: Option<PathBuf> = None;
    for dir_entry in fs::read_dir(installed_folder.as_path())? {
        let e = dir_entry?;

        let filename = e.file_name().to_string_lossy().to_string();
        if let Some(0) = filename.find(&paket_basename) {
            paket_with_same_basename = Some(e.path());
            break;
        }
    }

    if let Some(paket_path) = paket_with_same_basename {
        // There is a package with the same name.
        let installed_paket = read_config_from_toml(&paket_path)?;

        let currently_installed_version = Version::parse(&installed_paket.package.version).unwrap();
        let new_version = Version::parse(&config.package.version).unwrap();

        let installed_version_diff = match currently_installed_version.cmp(&new_version) {
            Ordering::Less => PaketExistance::LowerVersionInstalled,
            Ordering::Equal => PaketExistance::SameVersionInstalled,
            Ordering::Greater => PaketExistance::HigherVersionInstalled,
        };

        Ok(installed_version_diff)
    } else {
        Ok(PaketExistance::NotExists)
    }
}

fn check_application_dependencies(dependency_list: &toml::Table) -> Result<DependencyStatus> {
    for (key, value) in dependency_list {
        println!("{key} => {value}");
    }

    Ok(DependencyStatus::Valid)
}

fn check_all_dependencies(config: &Config) -> Result<DependencyStatus> {
    let dependencies = match &config.dependencies {
        Some(d) => d,
        None => return Ok(DependencyStatus::Valid), // No dependencies, directly valid
    };

    // Check application dependencies
    if dependencies.application.is_some() {
        check_application_dependencies(dependencies.application.as_ref().unwrap())?;
    }

    Ok(DependencyStatus::Valid)
}

pub fn install_paket(paket_path: &PathBuf) -> Result<PaketExistance> {
    // Read base .tar archive:
    let file = File::open(paket_path)?;
    let mut ar = Archive::new(file);
    let entries = ar.entries()?;

    // Get the valid Paket.toml
    let config: Config = read_paket_toml_inside_tar(entries)?;

    let installed_status = check_installed_version(&config)?;

    match installed_status {
        PaketExistance::HigherVersionInstalled | PaketExistance::SameVersionInstalled => {
            // Do nothing.
            return Ok(installed_status);
        }
        _ => (),
    }

    check_all_dependencies(&config)?;

    Ok(PaketExistance::NotExists)
}

/// Paket installation steps:
/// 1. Read `Paket.toml` inside the archive and check if it's a valid Config.
/// 2. Check installed status of the package. If paket doesn't exist or have lower version, proceed.
/// 3. Check if dependency tree of the paket is valid.
pub fn install_paket_files(paket_path_list: &[PathBuf]) -> Result<()> {
    for paket_path in paket_path_list {
        match install_paket(paket_path) {
            Ok(s) => println!("{:?} => {s:?}", paket_path.file_name()),
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
