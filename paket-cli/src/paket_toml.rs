use std::{error::Error, fs, path::Path};

use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Package {
    // ------------- Must Fields -------------
    /// Example `Paket.toml` file content:
    /// ```toml
    /// [package]
    /// name = "paket-ismi"
    /// ```
    pub name: String,

    /// Example:
    /// ```toml
    /// [package]
    /// version = "0.1.0"
    /// ```
    pub version: String,

    /// Example `Paket.toml` file content:
    /// ```toml
    /// [package]
    /// maintainers = ["Emin Fedar <eminfedar@gmail.com>"]
    /// ```
    pub maintainers: Vec<String>,

    /// Example `Paket.toml` file content:
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

    /// Example `Paket.toml` file content:
    /// ```toml
    /// [package]
    /// license = "MIT"
    /// # or a spesific license link:
    /// license = "mylicensesite.com/LICENSE"
    /// ```
    pub license: String,

    /// Architecture names list compatible with Debian architecture names.
    ///
    /// Example `Paket.toml` file content:
    /// ```toml
    /// [package]
    /// architectures = ["amd64", "i386", "riscv64", "arm64"]
    ///
    /// # If the package is architecture independent:
    /// architectures = ["any"]
    /// ```
    pub architectures: Vec<String>,

    // ------------- Optional Fields -------------
    /// Example `Paket.toml` file content:
    /// ```toml
    /// [package]
    /// homepage = "x.org"
    /// ```
    pub homepage: Option<String>,

    /// Example `Paket.toml` file content for a game:
    /// ```toml
    /// [package]
    /// keywords = ["platformer", "2D", "shooter", "action"]
    /// ```
    pub keywords: Option<Vec<String>>,

    /// Categories compatible with freedesktop categories: https://specifications.freedesktop.org/menu-spec/menu-spec-1.0.html#category-registry
    ///
    /// Example `Paket.toml` file content:
    /// ```toml
    /// [package]
    /// categories = ["Game", "Education"]
    /// ```
    pub categories: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub package: Package,
}

pub fn read_config_from_toml(filepath: &Path) -> Result<Config, Box<dyn Error>> {
    let config: Config = toml::from_str(fs::read_to_string(filepath)?.as_str())?;

    Ok(config)
}
