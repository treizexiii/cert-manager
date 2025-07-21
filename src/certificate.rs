use std::fs::read_to_string;

use openssl::x509::X509;
use openssl::x509::extension::SubjectAlternativeName;
use openssl::{
    asn1::Asn1Time,
    hash::MessageDigest,
    pkey::PKey,
    rsa::Rsa,
    x509::{X509Builder, X509NameBuilder},
};

pub struct CertificateData {
    pub private_key: String,
    pub cert_pem: String,
}

impl CertificateData {
    pub fn generate_self_signed(
        domains: Vec<String>,
        validity_days: u32,
    ) -> Result<CertificateData, Box<dyn std::error::Error>> {
        let rsa = Rsa::generate(2048)?;
        let pkey = PKey::from_rsa(rsa)?;

        let mut name_builder = X509NameBuilder::new()?;
        let common_names = domains
            .get(0)
            .cloned()
            .unwrap_or_else(|| "localhost".to_string());
        name_builder.append_entry_by_text("CN", &common_names)?;
        let name = name_builder.build();

        let mut cert_builder = X509Builder::new()?;
        cert_builder.set_version(2)?; // Version 3
        cert_builder.set_subject_name(&name)?;
        cert_builder.set_issuer_name(&name)?;
        cert_builder.set_pubkey(&pkey)?;

        cert_builder.set_not_before(&Asn1Time::days_from_now(0).unwrap())?;
        cert_builder.set_not_after(&Asn1Time::days_from_now(validity_days).unwrap())?;

        let mut alt_names_builder = SubjectAlternativeName::new();
        for domain in domains {
            alt_names_builder.dns(&domain);
        }
        let alt_names_ext = alt_names_builder.build(&cert_builder.x509v3_context(None, None))?;
        cert_builder.append_extension(alt_names_ext)?;

        cert_builder.sign(&pkey, MessageDigest::sha256())?;
        let cert = cert_builder.build();

        let private_key_pem = String::from_utf8(pkey.private_key_to_pem_pkcs8()?)?;
        let cert_pem = String::from_utf8(cert.to_pem()?)?;

        return Ok(CertificateData {
            private_key: private_key_pem,
            cert_pem,
        });
    }

    pub fn from_pem(folder_path: &str) -> Result<CertificateData, Box<dyn std::error::Error>> {
        let cert_path = format!("{}/cert.pem", folder_path);
        let key_path = format!("{}/key.pem", folder_path);
        let cert_pem = read_to_string(cert_path)?;
        let private_key = read_to_string(key_path)?;
        Ok(CertificateData {
            private_key: private_key.to_string(),
            cert_pem: cert_pem.to_string(),
        })
    }

    pub fn renew(
        &mut self,
        validity_days: u32,
    ) -> Result<CertificateData, Box<dyn std::error::Error>> {
        let cert = X509::from_pem(self.cert_pem.as_bytes())?;
        let not_before = Asn1Time::days_from_now(0)?;
        let not_after = Asn1Time::days_from_now(validity_days)?;

        let rsa = Rsa::private_key_from_pem(self.private_key.as_bytes())?;
        let pkey = PKey::from_rsa(rsa)?;

        let mut cert_builder = X509Builder::new()?;
        cert_builder.set_version(2)?; // Version 3
        cert_builder.set_subject_name(cert.subject_name())?;
        cert_builder.set_issuer_name(cert.issuer_name())?;
        cert_builder.set_pubkey(&pkey)?;
        cert_builder.set_not_before(&not_before)?;
        cert_builder.set_not_after(&not_after)?;

        let mut alt_names_builder = SubjectAlternativeName::new();
        for name in cert.subject_alt_names().iter() {
            for entry in name.iter() {
                if let Some(dns_name) = entry.dnsname() {
                    alt_names_builder.dns(dns_name);
                }
            }
        }
        let alt_names_ext = alt_names_builder.build(&cert_builder.x509v3_context(None, None))?;
        cert_builder.append_extension(alt_names_ext)?;

        cert_builder.sign(&pkey, MessageDigest::sha256())?;
        let new_cert = cert_builder.build();
        let private_key_pem = String::from_utf8(pkey.private_key_to_pem_pkcs8()?)?;
        let cert_pem = String::from_utf8(new_cert.to_pem()?)?;
        Ok(CertificateData {
            private_key: private_key_pem,
            cert_pem,
        })
    }

    pub fn serialize(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::Write;

        let cert_path = format!("{}/cert.pem", path);
        let key_path = format!("{}/key.pem", path);

        let mut cert_file = File::create(cert_path)?;
        cert_file.write_all(self.cert_pem.as_bytes())?;

        let mut key_file = File::create(key_path)?;
        key_file.write_all(self.private_key.as_bytes())?;

        Ok(())
    }
}

// let extensions = cert
//     .issuer_alt_names()
//     .iter()
//     .map(|name| {
//         let mut ext_builder = extension::SubjectAlternativeName::new();
//         for entry in name.iter() {
//             if let Some(dns_name) = entry.dnsname() {
//                 ext_builder.dns(dns_name);
//             }
//         }
//         ext_builder.build(&cert_builder.x509v3_context(None, None))
//     })
//     .collect::<Result<Vec<_>, _>>()?;

// for ext in extensions {
//     cert_builder.append_extension(ext)?;
// }
