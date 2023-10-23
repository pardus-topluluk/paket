use std::{fs::File, io::Read, path::Path};

use crate::{PaketError, Result};

use serde::Deserialize;
use toml;

/// Example usage in **Paket.toml**:
/// ```toml
/// [package]
/// type = "application"
/// ```
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PackageType {
    /// an Application in binary compiled form
    ///
    /// Example: `vlc`
    Application,

    /// an Application in script form(e.g.: Python or Javascript scripts)
    ///
    /// Example: `pardus-image-writer`
    Script,

    /// a Library to applications depend on it.
    ///
    /// Example: `libgtk-4`
    Library,

    /// The development files of a library which programs depend on.
    ///
    /// It can be used to develop applications which depends on a library.
    ///
    /// Example: `libgtk-4-dev` is the package to develop apps depends on `libgtk-4`
    DevelopmentLibrary,

    /// Source code of an application.
    ///
    /// It can be used to compile the application from source code embedded in the package.
    ///
    /// Example: `gzip-src`
    ///
    ApplicationSourceCode,

    /// Source code of a library.
    ///
    /// It can be used to compile the library from source code embedded in the package.
    ///
    /// Example: `libXYZ-1-src`
    LibrarySourceCode,

    /// Configuration files
    ///
    /// It can be used to provide a theme, icon package, grub configs etc.
    ///
    /// Example: `fantastic-icons`, `my-wallpapers`, `my-custom-grub-theme`
    Configuration,
}

/// `[package]` table in Paket.toml file
///
/// Stores the information about the package like `name`, `description`, `architectures`.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Package {
    // ------------- Must Fields -------------
    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [package]
    /// name = "paket-ismi"
    /// ```
    pub name: String,

    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [package]
    /// type = "application"
    /// ```
    #[serde(rename = "type")]
    pub package_type: PackageType,

    /// Example:
    /// ```toml
    /// [package]
    /// version = "0.1.0"
    /// ```
    pub version: String,

    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [package]
    /// maintainers = ["Emin Fedar <eminfedar@gmail.com>"]
    /// ```
    pub maintainers: Vec<String>,

    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [package]
    /// description = "Here is a description of the application"
    /// ```
    /// Multi line example:
    /// ```toml
    /// [package]
    /// description = """
    /// Here is a description of the application.
    ///
    /// Multiline is supported via three quotation marks.
    /// """
    /// ```
    pub description: String,

    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [package]
    /// license = "MIT"
    /// # or a spesific license link:
    /// license = "mylicensesite.com/LICENSE"
    /// ```
    pub license: String,

    /// Architecture names list compatible with Debian architecture names.
    ///
    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [package]
    /// architectures = ["amd64", "i386", "riscv64", "arm64"]
    ///
    /// # If the package is architecture independent:
    /// architectures = ["any"]
    /// ```
    pub architectures: Vec<String>,

    // ------------- Optional Fields -------------
    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [package]
    /// homepage = "pardus.org.tr"
    /// ```
    pub homepage: Option<String>,

    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [package]
    /// source_repository = "github.com/pardus-topluluk/paket"
    /// ```
    pub source_repository: Option<String>,

    /// Example usage in **Paket.toml** for a game:
    /// ```toml
    /// [package]
    /// keywords = ["platformer", "2D", "shooter", "action"]
    /// ```
    pub keywords: Option<Vec<String>>,

    /// Categories compatible with freedesktop categories: https://specifications.freedesktop.org/menu-spec/menu-spec-1.0.html#category-registry
    ///
    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [package]
    /// categories = ["Game", "Education"]
    /// ```
    pub categories: Option<Vec<String>>,
}

/// `[dependencies]` table in Paket.toml file
///
/// Stores the information of dependent applications, libraries or development libraries of the package.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Dependencies {
    /// Application dependencies of the package
    ///
    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [dependencies]
    /// application = ["python3.11", "python3-gi"]
    /// ```
    pub application: Option<Vec<String>>,

    /// Library dependencies of the package
    ///
    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [dependencies]
    /// library = ["libgtk-3-0", "libglib2.0.0", "libpango-1.0-0"]
    /// ```
    pub library: Option<Vec<String>>,

    /// Development dependencies of the package.
    ///
    /// <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
    /// <b>This field is only needed if the paket type is 'Source Code'</b>
    /// </p>
    ///
    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [dependencies]
    /// development = ["libgtk-3-0-dev"]
    /// ```   
    pub development: Option<Vec<String>>,
}

/// `[application]` table in Paket.toml file
///
/// Stores the PackageType::Application specific properties like `executable` or `icon`.
#[derive(Debug, Deserialize, PartialEq)]
pub struct ApplicationInformation {
    /// Binary executable file of the application
    ///
    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [application]
    /// executable = "helloworld"
    /// ```
    pub executable: String,

    /// .svg Icon of the application
    ///
    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [application]
    /// icon = "myapp.svg"
    /// ```
    pub icon: String,

    /// Contains read-only assets required to run for the application. (images, videos, UI files, 3D models, json stored datas etc.)
    ///
    /// This folder will be copied to: /usr/share/<appname>/assets/
    ///
    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [script]
    /// assets_folder = "assets"
    /// ```
    pub assets_folder: Option<String>,

    /// If given, this will be the used .desktop file.
    ///
    /// If not given or empty, .desktop file automatically will be generated from Paket.toml
    ///
    /// This file will be copied to: /usr/share/applications/<desktop_file>
    ///
    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [script]
    /// desktop_file = "helloworld.desktop"
    /// ```
    pub desktop_file: Option<String>,
}

/// `[script]` table in Paket.toml file
///
/// Stores the PackageType::Script specific properties like `sources`,`executable` or `icon`.
#[derive(Debug, Deserialize, PartialEq)]
pub struct ScriptInformation {
    /// Main executable file in the directory in `sources` property.
    ///
    /// Like: `main.py` or `main.js`
    ///
    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [script]
    /// source_folder = "src"
    /// executable = "main.py" # this is stored in `src/main.py`
    /// ```
    pub executable: String,

    /// .svg Icon of the application
    ///
    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [script]
    /// icon = "myapp.svg"
    /// ```
    pub icon: String,

    /// Contains main script and other scripts.
    ///
    /// This folder will be copied to: /usr/share/<appname>/src/
    ///
    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [script]
    /// source_folder = "src"
    /// ```
    pub sources_folder: Option<String>,

    /// Contains read-only assets required to run for the script. (images, videos, UI files, 3D models, json stored datas etc.)
    ///
    /// This folder will be copied to: /usr/share/<appname>/assets/
    ///
    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [script]
    /// assets_folder = "assets"
    /// ```
    pub assets_folder: Option<String>,

    /// If given, this will be the used .desktop file.
    ///
    /// If not given or empty, .desktop file automatically will be generated from Paket.toml
    ///
    /// This file will be copied to: /usr/share/applications/<desktop_file>
    ///
    /// Example usage in **Paket.toml**:
    /// ```toml
    /// [script]
    /// desktop_file = "myapp.desktop"
    /// ```
    pub desktop_file: Option<String>,
}

/// Represents the whole Paket.toml file
#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    /// `[package]` table in Paket.toml file
    ///
    /// Stores the information about the package like `name`, `description`, `architectures`.
    pub package: Package,

    /// `[dependencies]` table in Paket.toml file
    ///
    /// Stores the information of dependent applications, libraries or development libraries of the package.
    pub dependencies: Option<Dependencies>,

    /// `[application]` table in Paket.toml file
    ///
    /// Stores the `PackageType::Application` specific properties like `executable` or `icon`.
    pub application: Option<ApplicationInformation>,

    /// `[script]` table in Paket.toml file
    ///
    /// Stores the `PackageType::Script` specific properties like `sources`,`executable` or `icon`.
    pub script: Option<ScriptInformation>,
}

fn is_toml_file_valid(toml_path: &Path) -> Result<()> {
    let toml_path_string = toml_path.to_string_lossy().to_string();
    // Pre checks
    if !toml_path.exists() {
        return Err(PaketError::FileNotFound(toml_path_string));
    }

    if !toml_path.is_file() {
        return Err(PaketError::NotAFile(toml_path_string));
    }

    match toml_path.extension() {
        Some(s) => {
            if s != "toml" {
                return Err(PaketError::NotATomlFile(toml_path_string));
            }
        }
        None => return Err(PaketError::NotATomlFile(toml_path_string)),
    }

    Ok(())
}

/// Read and parse a toml file to any toml struct.
///
/// Example:
/// ```rust,no_run
/// use std::path::Path;
/// use toml;
/// use libpaket::toml_structs::paket_toml;
///
/// let toml_table: toml::Table = paket_toml::read_toml_file(Path::new("./Paket.toml")).unwrap();
/// ```
pub fn read_toml_file<T: serde::de::DeserializeOwned>(toml_path: &Path) -> Result<T> {
    // Validness guard
    is_toml_file_valid(toml_path)?;

    // Read toml file
    let mut file = File::open(toml_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    toml::from_str(content.as_str())
        .map_err(|e| PaketError::TomlParseError(e.message().to_string()))
}

/// Get `Config` struct from a `Paket.toml` file
///
/// Example:
/// ```rust,no_run
/// use std::path::Path;
/// use libpaket::toml_structs::paket_toml;
///
/// let paket_config: paket_toml::Config = paket_toml::read_config_from_toml(Path::new("./Paket.toml")).unwrap();
/// ```
pub fn read_config_from_toml(toml_path: &Path) -> Result<Config> {
    let config: Config = read_toml_file(toml_path)?;

    // Check package types
    match config.package.package_type {
        PackageType::Application => {
            if config.application.is_none() {
                return Err(PaketError::TomlFieldNotFound(
                    r#"type="application" pakets must have [application] field."#.to_string(),
                ));
            }
        }
        PackageType::Script => {
            if config.script.is_none() {
                return Err(PaketError::TomlFieldNotFound(
                    r#"type="script" pakets must have [script] field."#.to_string(),
                ));
            }
        }
        PackageType::Configuration => (),
        _ => (),
    }

    Ok(config)
}
