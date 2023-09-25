use std::path::Path;

use paket_cli::paket_toml::*;

const TEST_VALUES: &[(&str, &str)] = &[
    (
        "basic.toml",
        r#"Config { package: Package { name: "hello-world", package_type: Application, version: "0.1.0", maintainers: ["Emin Fedar <eminfedar@gmail.com>"], description: "Simple hello world program", license: "MIT", architectures: ["amd64"], homepage_url: None, source_repository_url: None, keywords: None, categories: None }, dependencies: None }"#,
    ),
    (
        "application_full.toml",
        r#"Config { package: Package { name: "hello-world", package_type: Application, version: "0.1.0", maintainers: ["Emin Fedar <eminfedar@gmail.com>"], description: "Multiline description of what this package is about.\n", license: "MIT", architectures: ["any"], homepage_url: None, source_repository_url: Some("github.com/repo-here/if-exists"), keywords: Some(["package", "tags", "here"]), categories: Some(["Game", "Education"]) }, dependencies: Some(Dependencies { application: Some(["python3.11", "python3-gi"]), library: Some(["libgtk-3-0", "libglib2.0.0", "libpango-1.0-0"]), development: None }) }"#,
    ),
];

#[test]
fn deserialize_example_tomls() {
    for (filename, content) in TEST_VALUES {
        let file_content =
            read_config_from_toml(Path::new(format!("./examples/{filename}").as_str())).unwrap();

        println!("Testing: {filename}");
        assert_eq!(*content, format!("{file_content:?}").as_str());
    }
}

#[test]
fn error_on_insufficient_toml() {
    assert!(read_config_from_toml(Path::new("./examples/insufficient.toml")).is_err());
}
