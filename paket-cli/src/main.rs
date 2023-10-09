use std::path::Path;

use crate::paket::Result;

use paket_cli::{
    paket::{err, success},
    *,
};

fn main() -> Result<()> {
    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("build", sub_matches)) => {
            let toml_path = sub_matches
                .get_one::<String>("toml_path")
                .expect("Expecting a valid --toml-path.");

            match paket::build::create_paket_from_toml(Path::new(toml_path)) {
                Ok((filename, _file)) => {
                    success(format!("Paket Successfully Created at: {}", filename));
                }
                Err(e) => {
                    err(&e);
                }
            };
        }
        Some(("install", sub_matches)) => {
            let packages: Vec<String> =
                sub_matches.get_many("packages").unwrap().cloned().collect();

            println!("Kurulacak paketler: {:?}", packages);
        }
        Some(("remove", sub_matches)) => {
            let packages: Vec<String> =
                sub_matches.get_many("packages").unwrap().cloned().collect();

            println!("Kaldırılacak paketler: {:?}", packages);
        }
        Some(("search", sub_matches)) => {
            let keywords: Vec<String> =
                sub_matches.get_many("keywords").unwrap().cloned().collect();
            println!("Aranacak kelimeler: {:?}", keywords);
        }

        _ => (),
    }

    Ok(())
}
