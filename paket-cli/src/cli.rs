use clap::{Arg, ArgAction, Command};

pub fn cli() -> Command {
    Command::new("paket")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Blazing fast, memory safe and modern Linux package manager.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("build")
                .about("Create a .paket package from a Paket.toml")
                .arg(
                    Arg::new("toml_path")
                        .default_value("./")
                        .help("Optional path to where Paket.toml located"),
                ),
        )
        .subcommand(
            Command::new("install")
                .about("Install package(s) by package name or .paket file")
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
                .about("Remove package(s) by package name or .paket file")
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
            Command::new("search").about("Search a package").arg(
                Arg::new("keywords")
                    .help("Keywords to search on package name and descriptions")
                    .value_parser(clap::builder::NonEmptyStringValueParser::new())
                    .action(ArgAction::Append)
                    .required(true),
            ),
        )
}
