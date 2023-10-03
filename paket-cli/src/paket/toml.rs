use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use serde::Deserialize;
use toml;

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
}

pub fn read_config_from_toml(toml_path: &Path) -> io::Result<Config> {
    // Pre checks
    if !toml_path.exists() || !toml_path.is_file() {
        return Err(io::Error::from(io::ErrorKind::NotFound));
    }

    match toml_path.extension() {
        Some(s) => {
            if s != "toml" {
                return Err(io::Error::from(io::ErrorKind::InvalidInput));
            }
        }
        None => return Err(io::Error::from(io::ErrorKind::InvalidInput)),
    }

    // Read toml file
    let mut file = File::open(toml_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Convert it to toml
    let config: Config = toml::from_str(content.as_str())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.message()))?;

    Ok(config)
}
