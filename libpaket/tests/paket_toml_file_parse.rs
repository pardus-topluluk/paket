#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::path::Path;
    use toml::map::Map;

    use libpaket::toml_structs::paket_toml::*;

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
                icon: Some(String::from("hello-world.svg")),
                assets_folder: None,
                desktop_file: None,
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

        // python3 = "3.11"
        // python3-gi = "3.42"
        let mut application_dependency_list = Map::new();
        application_dependency_list.insert(
            "python3".to_string(),
            toml::Value::String("3.11".to_string()),
        );
        application_dependency_list.insert(
            "python3-gi".to_string(),
            toml::Value::String("3.42".to_string()),
        );

        // libgtk4 = "4.8"
        let mut library_dependency_list = Map::new();
        library_dependency_list.insert(
            "libgtk4".to_string(),
            toml::Value::String("4.8".to_string()),
        );

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
                application: Some(application_dependency_list),
                library: Some(library_dependency_list),
                development: None,
            }),
            application: Some(ApplicationInformation {
                executable: String::from("myapp"),
                icon: Some(String::from("myapp.svg")),
                assets_folder: None,
                desktop_file: None,
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
