use std::fs::{DirEntry, File};
use std::io;
use std::path::Path;

use flate2::write::GzEncoder;
use flate2::Compression;

use crate::sha256;
use crate::toml_structs::paket_toml::{self, PackageType};
use crate::PaketError;
use crate::Result;

/// List only directories in a path.
fn list_directories(folder_path: &Path) -> io::Result<Vec<DirEntry>> {
    Ok(std::fs::read_dir(folder_path)?
        .filter_map(|r| r.ok()) // Get rid of Err variants for Result<DirEntry>
        .filter(|r| r.path().is_dir()) // Filter out non-folders
        .collect())
}

/// List only files in a path
fn list_files(folder_path: &Path) -> io::Result<Vec<DirEntry>> {
    Ok(std::fs::read_dir(folder_path)?
        .filter_map(|r| r.ok()) // Get rid of Err variants for Result<DirEntry>
        .filter(|r| r.path().is_file()) // Filter out non-folders
        .collect())
}

/// Convert from "hello-world" to "Hello World" to generate app name
fn kebab_case_to_separate_words(s: &str) -> String {
    s.split('-')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(c).collect(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

/// Generate .desktop file content from Paket.toml
fn generate_desktop_file_content(paket_toml: &paket_toml::Config) -> String {
    let pack = &paket_toml.package;

    let executable = match pack.package_type {
        PackageType::Application => &paket_toml.application.as_ref().unwrap().executable,
        PackageType::Script => &paket_toml.script.as_ref().unwrap().executable,
        _ => unreachable!(),
    };

    let icon = match pack.package_type {
        PackageType::Application => paket_toml.application.as_ref().unwrap().icon.as_ref(),
        PackageType::Script => paket_toml.script.as_ref().unwrap().icon.as_ref(),
        _ => None,
    };

    let categories = match pack.categories.as_ref() {
        Some(v) => v.join(";"),
        None => String::new(),
    };

    let mut formatted_desktop_file_content = format!(
        r#"[Desktop Entry]
Name={}
Comment={}
Exec={}
Type=Application"#,
        kebab_case_to_separate_words(pack.name.as_str()),
        pack.description,
        executable
    );

    if let Some(i) = icon {
        formatted_desktop_file_content.push_str(format!("Icon={}", i).as_str());
    }

    if !categories.is_empty() {
        formatted_desktop_file_content.push_str(format!("Categories={}", categories).as_str());
    }

    formatted_desktop_file_content
}

/// Create a .tar.gz compressed archive from DirEntry list.
fn create_data_tar_gz(
    paket_config: &paket_toml::Config,
    folders_list: &Vec<DirEntry>,
    files_list: &Vec<DirEntry>,
) -> Result<Vec<u8>> {
    let enc = GzEncoder::new(Vec::new(), Compression::default());
    let mut tar_builder = tar::Builder::new(enc);

    match paket_config.package.package_type {
        PackageType::Application => {
            // We can unwrap it because `paket_toml::read_config_from_toml`` checks this field must exists.
            let application = paket_config.application.as_ref().unwrap();

            // Add folders
            for d in folders_list {
                // `assets_folder = "assets"` property check
                if Some(d.file_name().to_string_lossy().to_string()) == application.assets_folder {
                    tar_builder.append_dir_all(
                        format!("/usr/share/{}/assets", paket_config.package.name),
                        d.path(),
                    )?;
                } else {
                    tar_builder.append_dir_all(d.file_name(), d.path())?;
                }
            }

            // Add files
            for f in files_list {
                let filename = f.file_name();
                let filename = filename.to_str().unwrap_or("");

                // Add `executable``
                if filename == application.executable.as_str() {
                    tar_builder.append_path_with_name(
                        f.path(),
                        Path::new("/usr/bin/").join(f.file_name()),
                    )?
                } else if filename
                    == application
                        .desktop_file
                        .as_ref()
                        .unwrap_or(&"".to_string())
                        .as_str()
                {
                    // Add `desktop_file` if exists
                    tar_builder.append_path_with_name(
                        f.path(),
                        Path::new("/usr/share/applications/").join(f.file_name()),
                    )?;
                } else if filename
                    == application
                        .icon
                        .as_ref()
                        .unwrap_or(&"".to_string())
                        .as_str()
                {
                    if !application.icon.as_ref().unwrap().ends_with(".svg") {
                        return Err(PaketError::TomlParseError(
                            "'application.icon' property must be a .svg file".to_string(),
                        ));
                    }
                    // Add `icon`
                    tar_builder.append_path_with_name(
                        f.path(),
                        Path::new("/usr/share/icons/hicolor/scalable/apps").join(f.file_name()),
                    )?;
                } else {
                    tar_builder.append_path_with_name(f.path(), f.file_name())?;
                }

                // Generate desktop file if doesn't exist
                if application.desktop_file.is_none() {
                    let desktop_file_content = generate_desktop_file_content(paket_config);

                    let mut header = tar::Header::new_gnu();
                    header.set_size(desktop_file_content.len() as u64);
                    header.set_entry_type(tar::EntryType::Regular);
                    header.set_path(format!(
                        "/usr/share/applications/{}.desktop",
                        paket_config.package.name
                    ))?;
                    header.set_mtime(
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    );
                    header.set_mode(0o755);
                    header.set_cksum();

                    tar_builder.append(&header, desktop_file_content.as_bytes())?;
                }
            }
        }

        PackageType::Script => {
            // We can unwrap it because `paket_toml::read_config_from_toml`` checks this field must exists.
            let script = paket_config.script.as_ref().unwrap();

            // Add folders
            for d in folders_list {
                // `assets_folder = "assets"` property check
                if d.file_name().to_str().unwrap() == script.sources_folder.as_str() {
                    tar_builder.append_dir_all(
                        format!("/usr/share/{}/src", paket_config.package.name),
                        d.path(),
                    )?;
                } else if Some(d.file_name().to_string_lossy().to_string()) == script.assets_folder
                {
                    tar_builder.append_dir_all(
                        format!("/usr/share/{}/assets", paket_config.package.name),
                        d.path(),
                    )?;
                } else {
                    tar_builder.append_dir_all(d.file_name(), d.path())?;
                }
            }

            // Add files
            for f in files_list {
                let filename = f.file_name();
                let filename = filename.to_str().unwrap_or("");

                // Add `executable``
                if filename == script.executable.as_str() {
                    tar_builder.append_path_with_name(
                        f.path(),
                        Path::new("/usr/bin/").join(f.file_name()),
                    )?
                } else if filename
                    == script
                        .desktop_file
                        .as_ref()
                        .unwrap_or(&"".to_string())
                        .as_str()
                {
                    // Add `desktop_file` if exists
                    tar_builder.append_path_with_name(
                        f.path(),
                        Path::new("/usr/share/applications/").join(f.file_name()),
                    )?;
                } else if filename == script.icon.as_ref().unwrap_or(&"".to_string()).as_str() {
                    if !script.icon.as_ref().unwrap().ends_with(".svg") {
                        return Err(PaketError::TomlParseError(
                            "'script.icon' property must be a .svg file".to_string(),
                        ));
                    }
                    // Add `icon`
                    tar_builder.append_path_with_name(
                        f.path(),
                        Path::new("/usr/share/icons/hicolor/scalable/apps").join(f.file_name()),
                    )?;
                } else {
                    tar_builder.append_path_with_name(f.path(), f.file_name())?;
                }
            }
        }

        PackageType::Configuration => {
            for p in folders_list {
                tar_builder.append_dir_all(p.file_name(), p.path())?;
            }
        }

        // TODO: Library and other types
        _ => {
            for p in folders_list {
                tar_builder.append_dir_all(p.file_name(), p.path())?;
            }
        }
    }

    Ok(tar_builder.into_inner()?.finish()?)
}

fn append_bytes_to_tar(
    tar_builder: &mut tar::Builder<File>,
    data: &[u8],
    path: impl AsRef<Path>,
) -> io::Result<()> {
    let mut header = tar::Header::new_gnu();
    header.set_size(data.len() as u64);
    header.set_entry_type(tar::EntryType::Regular);
    header.set_path(path)?;
    header.set_mtime(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );
    header.set_mode(0o755);
    header.set_cksum();

    tar_builder.append(&header, data)?;

    Ok(())
}

fn add_sha256sum_of_data_to_tar(
    tar_builder: &mut tar::Builder<File>,
    compressed_data: &[u8],
) -> io::Result<()> {
    // Add SHA256SUM of data.tar.gz
    let mut sha256sum = sha256::calculate_sha256(compressed_data);
    sha256sum.push('\n');

    append_bytes_to_tar(tar_builder, compressed_data, "SHA256SUM")?;

    Ok(())
}

/// Create a .paket file.
fn create_paket_archive(
    archive_name: &str,
    toml_file_path: &Path,
    compressed_data: Vec<u8>,
) -> io::Result<File> {
    let file = File::create(archive_name)?;
    let mut tar_builder = tar::Builder::new(file);

    // Add Paket.toml to tar
    tar_builder.append_path_with_name(toml_file_path, "Paket.toml")?;

    // Add SHA256SUM of data.tar.gz to paket archive:
    add_sha256sum_of_data_to_tar(&mut tar_builder, &compressed_data)?;

    // Add data.tar.gz
    append_bytes_to_tar(&mut tar_builder, &compressed_data, "data.tar.gz")?;

    // Create .paket file
    tar_builder.into_inner()
}

/// Create a .paket file from a Paket.toml config file path.
///
/// Example:
/// ```rust,no_run
/// use std::path::Path;
/// // Files:
/// // ./
/// // ├── Paket.toml
/// // └── usr/
/// //     └── bin/
/// //         └── myapp
/// libpaket::build::create_paket_from_toml(Path::new("./")).unwrap();
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
    let paket_config = paket_toml::read_config_from_toml(&toml_file_path)?;
    let archive_name = paket_config.get_paket_archive_name();

    let paket_folder_dir_list = list_directories(toml_folder_path)?;
    let paket_folder_file_list = list_files(toml_folder_path)?;
    println!("Dir List:{:#?}", paket_folder_dir_list);
    println!("File List:{:#?}", paket_folder_file_list);

    // Create data.tar.gz
    let compressed_data = create_data_tar_gz(
        &paket_config,
        &paket_folder_dir_list,
        &paket_folder_file_list,
    )?;

    // Create app_1.0.0.paket
    let paket_file = create_paket_archive(&archive_name, &toml_file_path, compressed_data)?;

    paket_file.sync_all()?;

    Ok((archive_name, paket_file))
}
