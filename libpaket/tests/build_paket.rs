#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::path::Path;

    use libpaket::build::*;

    #[test]
    fn build_configuration_paket() {
        let (_pathbuf, _file) =
            create_paket_from_toml(Path::new("./example_pakets/configuration_paket")).unwrap();
    }

    #[test]
    fn build_configuration_with_user_path_paket() {
        let (_pathbuf, _file) = create_paket_from_toml(Path::new(
            "./example_pakets/configuration_with_user_path_paket",
        ))
        .unwrap();
    }
}
