use std::{fs, path::Path};

use crate::certificate::CertificateData;

pub(crate) fn generate_certificate(domains: Vec<String>, validity_days: u32, folder_path: &str) {
    match CertificateData::generate_self_signed(domains, validity_days) {
        Ok(cert) => {
            println!("Certificate generated successfully for {}.", cert.name);
            let path = build_path(folder_path, &cert.name);
            match cert.serialize(&path) {
                Ok(_) => println!("Certificate and key saved to {}", path),
                Err(e) => eprintln!("Error saving certificate: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Error generating certificate: {}", e);
        }
    }
}

pub(crate) fn renew_certificate(files_path: &str, validity_days: u32) {
    let mut cert = match CertificateData::from_pem(&files_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading certificate: {}", e);
            return;
        }
    };
    match cert.renew(validity_days) {
        Ok(_) => {
            println!("Certificate renewed successfully for {}.", cert.name);
            match cert.serialize(&files_path) {
                Ok(_) => println!("Renewed certificate and key saved to {}", files_path),
                Err(e) => eprintln!("Error saving renewed certificate: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Error renewing certificate: {}", e);
        }
    }
}

fn build_path(folder_path: &str, folder_name: &str) -> String {
    let path = format!("{}/{}", folder_path, folder_name);
    let exists = Path::new(&path).exists();
    if !exists {
        fs::create_dir_all(&path).unwrap_or_else(|_| {
            eprintln!("Failed to create directory: {}", path);
        });
    }
    path
}
