use std::path::{Path, PathBuf};

use libpaket::Result;
use paket_cli::cli;
use paket_cli::repo;

fn main() -> Result<()> {
    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("build", sub_matches)) => {
            let toml_path = sub_matches
                .get_one::<String>("toml_path")
                .expect("Expecting a valid --toml-path.");

            match libpaket::build::create_paket_from_toml(Path::new(toml_path)) {
                Ok((filename, _file)) => {
                    cli::success(format!("Paket Successfully Created at: {}", filename));
                }
                Err(e) => {
                    cli::err(&e);
                }
            };
        }
        Some(("install", sub_matches)) => {
            let args: Vec<String> = sub_matches.get_many("packages").unwrap().cloned().collect();

            let (paket_files, package_names): (Vec<String>, Vec<String>) =
                args.into_iter().partition(|e| e.ends_with(".paket"));

            let paket_files: Vec<PathBuf> = paket_files.into_iter().map(PathBuf::from).collect();

            println!(
                "Packages will be installed from repository: {:?}",
                package_names
            );
            println!(".paket files will be installed: {:?}", paket_files);

            // TODO: Download & Install paket files
            //match repo::download(&package_names) {
            //    Ok(s) => {
            //        cli::success(format!(".paket files downloaded: {s:?}"));
            //    }
            //    Err(e) => {
            //        cli::err(&e);
            //    }
            //};

            // Install paket files:
            match libpaket::install::install_paket_files(&paket_files) {
                Ok(s) => {
                    cli::success(format!("Paket Installed: {s:?}"));
                }
                Err(e) => {
                    cli::err(&e);
                }
            };
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
