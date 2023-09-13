use std::path::Path;

use paket_cli::paket_toml::*;

#[test]
fn deserialize_basic_binary_toml() {
    let config = read_config_from_toml(Path::new("./examples/basic-toml/Paket.toml")).unwrap();

    assert_eq!(
        r#"Config { package: Package { name: "hello-world", version: "0.1.0", maintainers: ["Emin Fedar <eminfedar@gmail.com>"], description: "Simple hello world program", license: "MIT", architectures: ["amd64"], homepage: None, keywords: None, categories: None } }"#,
        format!("{config:?}").as_str()
    );
}

#[test]
fn deserialize_full_binary_toml() {
    let config = read_config_from_toml(Path::new("./examples/full-toml/Paket.toml")).unwrap();

    assert_eq!(
        r#"Config { package: Package { name: "hello-world", version: "0.1.0", maintainers: ["Emin Fedar <eminfedar@gmail.com>"], description: "Multiline description of what this package is about.\n", license: "MIT", architectures: ["any"], homepage: Some("pardus.org.tr"), keywords: Some(["package", "tags", "here"]), categories: Some(["Game", "Education"]) } }"#,
        format!("{config:?}").as_str()
    );
}

#[test]
fn error_on_insufficient_toml() {
    assert!(read_config_from_toml(Path::new("./examples/Insufficient_Paket.toml")).is_err());
}
