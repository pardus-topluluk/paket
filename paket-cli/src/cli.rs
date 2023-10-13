use clap::{Arg, ArgAction, Command};

pub fn cli() -> Command {
    Command::new("paket")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Blazing fast, memory safe and modern Linux package manager written in Rust.\nSource Repository: github.com/pardus-topluluk/paket")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("build")
                .about("Create a .paket package from Paket.toml.\n- Example usage: `paket build ./myapp_folder`")
                .arg(
                    Arg::new("toml_path")
                        .default_value("./")
                        .help("Path to where Paket.toml located."),
                ),
        )
        .subcommand(
            Command::new("install")
                .about("Install package(s) by package name or .paket file.\n- Example usage: `paket install vlc`")
                .arg(
                    Arg::new("packages")
                        .help("Package names or .paket file name")
                        .value_name("package-name or package_1.0.0.paket")
                        .value_parser(clap::builder::NonEmptyStringValueParser::new())
                        .action(ArgAction::Append)
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove package(s) by package name or .paket file.\n- Example usage: `paket remove vlc`")
                .alias("uninstall")
                .arg(
                    Arg::new("packages")
                        .help("Package names or .paket file name")
                        .value_name("package-name or package_1.0.0.paket")
                        .value_parser(clap::builder::NonEmptyStringValueParser::new())
                        .action(ArgAction::Append)
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("search")
                .about("Search a package\n- Example usage: `paket search vlc`")
                .arg(
                    Arg::new("keywords")
                        .help("Keywords to search on package name and descriptions")
                        .value_parser(clap::builder::NonEmptyStringValueParser::new())
                        .action(ArgAction::Append)
                        .required(true),
                ),
        )
}

use libpaket::color::*;
use libpaket::PaketError;
pub fn err(e: &PaketError) {
    eprintln!("❌ {RED}{BOLD}[Error]:{RESET} {e}");
}

pub fn success(s: impl AsRef<str>) {
    println!("✅ {GREEN}{BOLD}[Success]:{RESET} {}", s.as_ref());
}
