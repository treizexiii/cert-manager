pub mod certificate;

use std::path;

use certificate::CertificateData;

const FOLDER_NAME: &str = "./certs";

fn main() {
    println!("Hello, world!");

    let subject_alt_names = vec!["localhost".to_string()];
    let path = path::Path::new(FOLDER_NAME);
    if !path.exists() {
        match std::fs::create_dir_all(path) {
            Ok(_) => println!("Created directory: {}", FOLDER_NAME),
            Err(e) => {
                eprintln!("Failed to create directory {}: {}", FOLDER_NAME, e);
                return;
            }
        }
    }
    
    match CertificateData::generate_self_signed(subject_alt_names, 365) {
        Ok(cert) => {
            println!("Private Key:\n{}", cert.private_key);
            println!("Certificate PEM:\n{}", cert.cert_pem);

            let _ = std::fs::write(
                format!("{}/cert.pem", path.to_str().unwrap()),
                cert.cert_pem,
            );

            let _ = std::fs::write(
                format!("{}/key.pem", path.to_str().unwrap()),
                cert.private_key,
            );

        }
        Err(e) => eprintln!("Error generating certificate: {}", e),
    }
}

