pub mod build;
pub mod install;
pub mod sha256;
pub mod toml_structs;

use std::fmt;

#[allow(dead_code)]
pub mod color {
    pub const RESET: &str = "\x1b[0m"; // Sıfırla (rengi sıfırlar ve özellikleri kapatır)
    pub const BOLD: &str = "\x1b[1m"; // Kalın
    pub const DIM: &str = "\x1b[2m"; // İnce
    pub const UNDERLINE: &str = "\x1b[4m"; // Alt Çizgi
    pub const INVERSE: &str = "\x1b[7m"; // Ters (renklerin tersine çevrilmesi)
    pub const STRIKETHROUGH: &str = "\x1b[9m"; // Üstü Çizili
    pub const RED: &str = "\x1b[31m"; // Kırmızı
    pub const GREEN: &str = "\x1b[32m"; // Yeşil
    pub const YELLOW: &str = "\x1b[33m"; // Sarı
    pub const BLUE: &str = "\x1b[34m"; // Mavi
    pub const MAGENTA: &str = "\x1b[35m"; // Magenta (Pembe)
    pub const CYAN: &str = "\x1b[36m"; // Cyan (Camgöbeği)
    pub const WHITE: &str = "\x1b[37m"; // Beyaz
}
use color::*;

#[derive(Debug)]
pub enum PaketError {
    FileNotFound(String),
    NotAFile(String),
    NotATomlFile(String),
    TomlFieldNotFound(String),
    TomlParseError(String),
    IOError(String),
}

pub type Result<T> = std::result::Result<T, PaketError>;

impl fmt::Display for PaketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PaketError::FileNotFound(s) => write!(f, "File not found: {BOLD}'{s}'{RESET}"),
            PaketError::NotAFile(s) => write!(f, "{BOLD}'{s}'{RESET} is not a file."),
            PaketError::NotATomlFile(s) => write!(f, "{BOLD}'{s}{RESET}' is not a .toml file."),
            PaketError::TomlFieldNotFound(s) => {
                write!(f, "Field not found in Paket.toml: {BOLD}{s}{RESET}")
            }
            PaketError::TomlParseError(s) => {
                write!(f, "Paket.toml Parse Error: {BOLD}'{s}'{RESET}")
            }
            PaketError::IOError(s) => write!(f, "IO Error -> {BOLD}'{s}'{RESET}"),
        }
    }
}

impl From<std::io::Error> for PaketError {
    fn from(value: std::io::Error) -> Self {
        PaketError::IOError(value.to_string())
    }
}
