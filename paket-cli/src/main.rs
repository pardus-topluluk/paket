use paket_cli::*;

fn main() {
    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("build", sub_matches)) => {
            println!(
                "Paket.toml dosyasının olduğu path: {:?}",
                sub_matches.get_one::<String>("toml_path")
            );
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
}
