mod certificate;
mod process;

use clap::{Command, arg};

const FOLDER_NAME: &str = "./certs";

fn main() {
    let mut matches = build_args();
    let args = matches.clone().get_matches();
    let subcommand = args.subcommand_name().unwrap_or("help");

    match subcommand {
        "generate" => {
            let domain_args = args.get_one::<String>("domain").unwrap();
            let domains: Vec<String> = domain_args
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            let validity_days: u32 = *args.get_one::<u32>("validity").unwrap();
            process::generate_certificate(domains, validity_days);
            println!("Certificate generation process completed.");
        }
        "renew" => {
            let file_path = args.get_one::<String>("file").unwrap();
            let validity_days: u32 = *args.get_one::<u32>("validity").unwrap_or(&365);
            process::renew_certificate(file_path, validity_days);
            println!("Certificate renewal process completed.");
        }
        "help" => {
            matches.print_long_help().unwrap();
        }
        _ => {
            println!("\nInvalid subcommand: '{}'", subcommand);
            matches.print_long_help().unwrap();
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
                .arg(
                    arg!(-d --domain <DOMAIN> "The domain for which the certificate is generated")
                        .required(true)
                        .value_name("DOMAIN")
                        .default_value("localhost"),
                )
                .arg(
                    arg!(-v --validity <DAYS> "Validity period in days, default is 365")
                        .value_name("DAYS")
                        .default_value("365")
                        .value_parser(clap::value_parser!(u32)),
                ),
        )
        .subcommand(
            Command::new("renew")
                .about("Renew an existing certificate")
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
