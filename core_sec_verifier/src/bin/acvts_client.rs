// Client CLI utility for NIST ACVTS Demo API with mTLS and TOTP authentication.
// Enables potential buyers to see local validation success.

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use reqwest::{Client, Identity};
use totp_rs::{Algorithm, TOTP, Secret};
use base64::{Engine as _, engine::general_purpose::STANDARD};

/// Loads the private key and certificate to configure the mTLS Identity.
fn build_identity<P: AsRef<Path>>(key_path: P, cert_path: P) -> Result<Identity, Box<dyn Error>> {
    let mut key_file = File::open(key_path)?;
    let mut key_pem = Vec::new();
    key_file.read_to_end(&mut key_pem)?;

    let mut cert_file = File::open(cert_path)?;
    let mut cert_pem = Vec::new();
    cert_file.read_to_end(&mut cert_pem)?;

    // Configure mTLS Identity from PKCS8 PEM format
    let identity = Identity::from_pkcs8_pem(&cert_pem, &key_pem)?;
    Ok(identity)
}

/// Generates a time-based one-time password (TOTP) token using the Base64-encoded secret seed.
fn generate_totp_token(seed_path: &str) -> Result<String, Box<dyn Error>> {
    let mut b64_seed = String::new();

    // Attempt to load from file
    if Path::new(seed_path).exists() {
        let mut file = File::open(seed_path)?;
        file.read_to_string(&mut b64_seed)?;
    } else {
        // Fallback default mock Base64 seed if file does not exist yet
        b64_seed = "YmFzZTY0c2VlZDEyMzQ1Njc4OTBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3g=".to_string();
    }

    // Strip whitespace
    let b64_seed_trimmed = b64_seed.trim();

    // Decode Base64 string into raw bytes
    let raw_bytes = STANDARD.decode(b64_seed_trimmed)?;

    // Use raw bytes as HMAC-SHA1 key for standard 30-second, 6-digit TOTP token
    let secret = Secret::Raw(raw_bytes);
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret.to_bytes()?,
    )?;
    let token = totp.generate_current()?;
    Ok(token)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== NexusCorps ACVTS mTLS & TOTP Client CLI ===");

    let key_path = std::env::var("ACVTS_KEY_PATH")
        .unwrap_or_else(|_| "NexusCorps_Dennis_Merritt_Demo.key".to_string());
    let cert_path = std::env::var("ACVTS_CERT_PATH")
        .unwrap_or_else(|_| "NexusCorps_Dennis_Merritt_Demo.crt".to_string());
    let seed_path = std::env::var("ACVTS_SEED_PATH")
        .unwrap_or_else(|_| "NexusCorps_Dennis_Merritt_Demo_totp.txt".to_string());
    let api_url = std::env::var("ACVTS_API_URL")
        .unwrap_or_else(|_| "https://demo.acvts.nist.gov/acvp/v1/".to_string());

    println!("Loading private key from: {key_path}");
    println!("Loading certificate from: {cert_path}");
    println!("Loading TOTP seed from: {seed_path}");

    // Build the mTLS identity
    let mut client_builder = Client::builder();
    match build_identity(&key_path, &cert_path) {
        Ok(identity) => {
            client_builder = client_builder.identity(identity);
            println!("mTLS Identity configured successfully.");
        }
        Err(e) => {
            println!("Warning: Could not configure mTLS identity ({e}). Proceeding without certificate...");
        }
    }

    // Build client
    let client = client_builder.build()?;

    // Generate TOTP token
    match generate_totp_token(&seed_path) {
        Ok(token) => {
            println!("Generated 6-digit TOTP Token: {token}");

            // Send request to ACVTS demo server
            println!("Sending GET request to ACVTS Demo API: {api_url}");
            let response = client.get(&api_url)
                .header("Authorization", format!("Bearer {token}"))
                .send()
                .await;

            match response {
                Ok(resp) => {
                    println!("Response Status: {}", resp.status());
                    if let Ok(text) = resp.text().await {
                        println!("Response Body: {text}");
                    }
                }
                Err(err) => {
                    println!("API Error: {err}");
                }
            }
        }
        Err(e) => {
            println!("Error generating TOTP token: {e}");
        }
    }

    Ok(())
}
