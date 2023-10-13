#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::path::Path;

    use libpaket::toml::*;

    #[test]
    fn deserialize_basic_toml() {
        let read_config: Config =
            read_config_from_toml(Path::new(format!("./example_pakets/basic.toml").as_str()))
                .unwrap();

        let expected_config = Config {
            package: Package {
                name: String::from("hello-world"),
                package_type: PackageType::Application,
                version: String::from("0.1.0"),
                maintainers: vec![String::from("Emin Fedar <eminfedar@gmail.com>")],
                description: String::from("Simple hello world program"),
                license: String::from("MIT"),
                architectures: vec![String::from("amd64")],
                homepage: None,
                source_repository: None,
                keywords: None,
                categories: None,
            },
            dependencies: None,
            application: Some(ApplicationInformation {
                executable: String::from("hello-world"),
                icon: String::from("hello-world.svg"),
                assets_folder: None,
            }),
            script: None,
        };

        assert_eq!(read_config, expected_config);
    }

    #[test]
    fn deserialize_application_full_toml() {
        let read_config: Config = read_config_from_toml(Path::new(
            format!("./example_pakets/application_full.toml").as_str(),
        ))
        .unwrap();

        let expected_config = Config {
            package: Package {
                name: String::from("hello-world"),
                package_type: PackageType::Application,
                version: String::from("0.1.0"),
                maintainers: vec![String::from("Emin Fedar <eminfedar@gmail.com>")],
                description: String::from("Multiline description of what this package is about.\n"),
                license: String::from("MIT"),
                architectures: vec![String::from("any")],
                homepage: Some(String::from("https://pardus.org.tr")),
                source_repository: Some(String::from("https://github.com/repo-here/if-exists")),
                keywords: Some(vec![
                    String::from("package"),
                    String::from("tags"),
                    String::from("here"),
                ]),
                categories: Some(vec![String::from("Game"), String::from("Education")]),
            },
            dependencies: Some(Dependencies {
                application: Some(vec![String::from("python3.11"), String::from("python3-gi")]),
                library: Some(vec![
                    String::from("libgtk-3-0"),
                    String::from("libglib2.0.0"),
                    String::from("libpango-1.0-0"),
                ]),
                development: None,
            }),
            application: Some(ApplicationInformation {
                executable: String::from("myapp"),
                icon: String::from("myapp.svg"),
                assets_folder: None,
            }),
            script: None,
        };

        assert_eq!(read_config, expected_config);
    }

    #[test]
    fn error_on_insufficient_toml() {
        assert!(read_config_from_toml(Path::new("./example_pakets/insufficient.toml")).is_err());
    }
}
