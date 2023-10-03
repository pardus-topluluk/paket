use std::{
    io::{self, ErrorKind},
    path::Path,
};

use color_print::{cformat, cprintln, cstr};
use paket_cli::*;

fn main() -> io::Result<()> {
    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("build", sub_matches)) => {
            let toml_path = sub_matches
                .get_one::<String>("toml_path")
                .expect("Expecting a valid --toml-path.");

            match paket::build::create_paket_from_toml(Path::new(toml_path)) {
                Ok((pathbuf, _file)) => {
                    cprintln!(
                        "✅ <green,bold>Paket Successfully Created at:</> {}",
                        pathbuf.to_string_lossy()
                    );
                }
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => {
                        eprintln!(
                            "{}",
                            cformat!(
                                "❌ <bold><red>Error:</> Paket.toml wasn't found in path: '{}'</>",
                                toml_path
                            )
                        );
                    }
                    ErrorKind::InvalidInput => {
                        eprintln!(cstr!(
                            "❌ <bold><red>Error:</> Please provide a valid 'Paket.toml' file path.</>",
                        ));
                    }
                    _ => eprintln!(
                        "{}",
                        cformat!("❌ <red,bold>Error while creating a package:</> {e:?}")
                    ),
                },
            }
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
