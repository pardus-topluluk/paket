use std::fs::{DirEntry, File};
use std::io;
use std::path::Path;

use flate2::write::GzEncoder;
use flate2::Compression;

use crate::paket;
use crate::paket::Result;

/// List only directories in a folder.
fn list_only_directories(folder_path: &Path) -> io::Result<Vec<DirEntry>> {
    Ok(std::fs::read_dir(folder_path)?
        .filter_map(|r| r.ok()) // Get rid of Err variants for Result<DirEntry>
        .filter(|r| r.path().is_dir()) // Filter out non-folders
        .collect())
}

/// Create a .tar.gz compressed archive from DirEntry list.
fn create_tar_gz_archive_from_directories(folders_list: &Vec<DirEntry>) -> io::Result<Vec<u8>> {
    let enc = GzEncoder::new(Vec::new(), Compression::default());
    let mut tar_builder = tar::Builder::new(enc);

    for p in folders_list {
        tar_builder.append_dir_all(p.file_name(), p.path())?;
    }

    tar_builder.into_inner()?.finish()
}

/// Create a .paket file.
fn create_paket_archive(
    archive_name: &str,
    toml_file_path: &Path,
    compressed_data: Vec<u8>,
) -> io::Result<File> {
    let file = File::create(archive_name)?;
    let mut tar_builder = tar::Builder::new(file);

    // Add Paket.toml
    tar_builder.append_path_with_name(toml_file_path, "Paket.toml")?;

    // Add SHA256SUM of data.tar.gz
    let mut sha256sum = paket::sha256::calculate_sha256(&compressed_data);
    sha256sum.push('\n');

    let mut header = tar::Header::new_gnu();
    header.set_size(sha256sum.len() as u64);
    header.set_entry_type(tar::EntryType::Regular);
    header.set_path("SHA256SUM")?;
    header.set_mtime(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );
    header.set_mode(0o755);
    header.set_cksum();
    tar_builder.append(&header, sha256sum.as_bytes())?;

    // Add data.tar.gz
    let mut header = tar::Header::new_gnu();
    header.set_size(compressed_data.len() as u64);
    header.set_entry_type(tar::EntryType::Regular);
    header.set_path("data.tar.gz")?;
    header.set_mtime(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );
    header.set_mode(0o755);
    header.set_cksum();
    tar_builder.append(&header, compressed_data.as_slice())?;

    // Create .paket file
    tar_builder.into_inner()
}

/// Create a .paket file from a Paket.toml config file path.
///
/// Example:
/// ```rust,no_run
/// use std::path::Path;
/// use paket_cli::paket;
/// // Files:
/// // ./
/// // ├── Paket.toml
/// // └── usr/
/// //     └── bin/
/// //         └── myapp
/// paket::build::create_paket_from_toml(Path::new("./")).unwrap();
///
/// // Then there should be a paket file created if everything is ok:
/// // Files:
/// // ./
/// // ├── myapp_1.0.0.paket
/// // ├── Paket.toml
/// // └── usr/
/// //     └── bin/
/// //         └── myapp
/// ```
pub fn create_paket_from_toml(toml_folder_path: &Path) -> Result<(String, File)> {
    // Read Config struct from toml file
    let toml_file_path = toml_folder_path.join("Paket.toml");
    let paket_config = paket::toml::read_config_from_toml(&toml_file_path)?;
    let archive_name = format!(
        "{}_{}.paket",
        paket_config.package.name.as_str(),
        paket_config.package.version.as_str()
    );

    let paket_folder_dir_list = paket::to_paket_error(list_only_directories(toml_folder_path))?;

    // Create data.tar.gz
    let compressed_data = paket::to_paket_error(create_tar_gz_archive_from_directories(
        &paket_folder_dir_list,
    ))?;

    // Create app_1.0.0.paket
    let paket_file = paket::to_paket_error(create_paket_archive(
        &archive_name,
        &toml_file_path,
        compressed_data,
    ))?;

    paket::to_paket_error(paket_file.sync_all())?;

    Ok((archive_name, paket_file))
}
