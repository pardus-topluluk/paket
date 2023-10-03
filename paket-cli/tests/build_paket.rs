// use pretty_assertions::assert_eq;
use std::path::Path;

use paket_cli::paket::*;

#[test]
fn build_configuration_paket() {
    let (_pathbuf, _file) =
        build::create_paket_from_toml(Path::new("./examples/configuration_paket")).unwrap();
}

#[test]
fn build_configuration_with_user_path_paket() {
    let (_pathbuf, _file) =
        build::create_paket_from_toml(Path::new("./examples/configuration_with_user_path_paket"))
            .unwrap();
}
