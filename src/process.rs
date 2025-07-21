use crate::{certificate::CertificateData, FOLDER_NAME};


pub(crate) fn generate_certificate(domains: Vec<String>, validity_days: u32) {
    match CertificateData::generate_self_signed(domains, validity_days) {
        Ok(cert) => {
            println!("Certificate generated successfully.");
            match cert.serialize(FOLDER_NAME) {
                Ok(_) => println!("Certificate and key saved to {}", FOLDER_NAME),
                Err(e) => eprintln!("Error saving certificate: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Error generating certificate: {}", e);
        }
    }
}

pub(crate) fn renew_certificate(files_path: &str, validity_days: u32) {
    let mut cert_data = match CertificateData::from_pem(&files_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading certificate: {}", e);
            return;
        }
    };
    match cert_data.renew(validity_days) {
        Ok(_) => {
            println!("Certificate renewed successfully.");
            match cert_data.serialize(FOLDER_NAME) {
                Ok(_) => println!("Renewed certificate and key saved to {}", FOLDER_NAME),
                Err(e) => eprintln!("Error saving renewed certificate: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Error renewing certificate: {}", e);
        }
    }
}