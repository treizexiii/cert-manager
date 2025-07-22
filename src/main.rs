mod certificate;
mod process;

use clap::{Arg, Command, arg};

fn main() {
    let mut command = build_args();
    let matches = command.clone().get_matches();
    let subcommand = matches.subcommand_name().unwrap_or("help");

    match subcommand {
        "generate" => {
            let args = matches.subcommand_matches(subcommand).unwrap_or(&matches);
            let domains: Vec<String> = args
                .get_many::<String>("domain")
                .map(|v| v.map(String::from).collect())
                .unwrap_or_else(|| vec!["localhost".to_string()]);
            let validity_days: u32 = *args.get_one::<u32>("validity").unwrap_or(&365);
            let folder_path: &String = args.get_one::<String>("path").unwrap();
            process::generate_certificate(domains, validity_days, folder_path);
            println!("Certificate generation process completed.");
        }
        "renew" => {
            let file_path = matches.get_one::<String>("file").unwrap();
            let validity_days: u32 = *matches.get_one::<u32>("validity").unwrap_or(&365);
            let folder_path = matches.get_one::<String>("path").unwrap();
            process::renew_certificate(file_path, validity_days, folder_path);
            println!("Certificate renewal process completed.");
        }
        "help" => {
            command.print_long_help().unwrap();
        }
        _ => {
            println!("\nInvalid subcommand: '{}'", subcommand);
            command.print_long_help().unwrap();
        }
    }
}

fn build_args() -> Command {
    let command = clap::Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            Command::new("generate")
                .about("Generate a self-signed certificate")
                .arg(Arg::new("domain")
                        .short('d')
                        .long("domain")
                        .help("Domain names for the certificate, comma-separated")
                        .value_name("DOMAIN")
                        .default_value("localhost")
                        .value_parser(clap::value_parser!(String))
                        .required(false)
                        .default_missing_value("localhost"))
                .arg(Arg::new("validity")
                        .short('v')
                        .long("validity")
                        .help("Validity period in days, default is 365")
                        .value_name("DAYS")
                        .default_value("365")
                        .value_parser(clap::value_parser!(u32)))
                .arg(Arg::new("path")
                        .short('p')
                        .long("path")
                        .help("The path to the folder where the certificate will be saved")
                        .value_name("FOLDER")
                        .required(true)
                        .value_parser(clap::value_parser!(String)))
                )
        .subcommand(
            Command::new("renew")
                .about("Renew an existing certificate")
                .arg(
                    arg!([path] "The path to the folder where the certificate will be saved")
                        .required(true)
                        // last argument is the folder path
                        .value_name("FOLDER")
                        .index(1),
                )
                .arg(
                    arg!(-f --file <FILE> "The path to the certificate file")
                        .required(true)
                        .value_name("FILE"),
                )
                .arg(
                    arg!(-v --validity <DAYS> "Validity period in days, default is 365")
                        .value_name("DAYS")
                        .default_value("365")
                        .value_parser(clap::value_parser!(u32)),
                ),
        );
    command
}
