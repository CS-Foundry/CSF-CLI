use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use dialoguer::{Input, Password};
use reqwest;
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use serde::{Deserialize, Serialize};

use crate::config::{load_config, save_config, Config};

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    encrypted_password: String,
    two_factor_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginResponse {
    token: String,
    user_id: String,
    username: String,
    two_factor_enabled: bool,
    force_password_change: bool,
}

#[derive(Serialize, Deserialize)]
struct PublicKeyResponse {
    public_key: String,
}

#[derive(Serialize, Deserialize)]
struct ChangePasswordRequest {
    new_password: String,
}

/// Holt den öffentlichen RSA-Schlüssel vom Server
async fn get_public_key(server: &str) -> Result<RsaPublicKey, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/public-key", server.trim_end_matches('/'));

    println!("📡 Request: GET {}", url);

    let response = client
        .get(&url)
        .header("accept", "application/json")
        .send()
        .await?;

    println!("📥 Response Status: {}", response.status());

    if !response.status().is_success() {
        return Err(format!("Fehler beim Abrufen des Public Keys: {}", response.status()).into());
    }

    let response_text = response.text().await?;
    println!("📥 Response Body:\n{}", response_text);

    let public_key_response: PublicKeyResponse = serde_json::from_str(&response_text)?;
    println!("\n🔑 Public Key Format erkannt");
    println!("   Länge: {} Zeichen", public_key_response.public_key.len());
    println!(
        "   Start: {}...",
        &public_key_response
            .public_key
            .chars()
            .take(50)
            .collect::<String>()
    );

    // Parse PEM-formatted public key (PKCS#1 Format: RSA PUBLIC KEY)
    println!("\n🔍 Versuche PEM zu parsen...");
    let public_key =
        RsaPublicKey::from_pkcs1_pem(&public_key_response.public_key).map_err(|e| {
            eprintln!("❌ PEM Parse Fehler: {}", e);
            eprintln!(
                "📋 Vollständiger Public Key:\n{}",
                public_key_response.public_key
            );
            e
        })?;

    println!("✅ Public Key erfolgreich geladen!");
    Ok(public_key)
}

/// Verschlüsselt das Passwort mit dem öffentlichen RSA-Schlüssel
fn encrypt_password(
    password: &str,
    public_key: &RsaPublicKey,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let encrypted_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, password.as_bytes())?;
    Ok(BASE64.encode(&encrypted_data))
}

/// Fordert den Benutzer zur Eingabe eines neuen Passworts auf
fn prompt_new_password() -> Result<String, Box<dyn std::error::Error>> {
    loop {
        let new_password = Password::new().with_prompt("Neues Passwort").interact()?;

        let confirm_password = Password::new()
            .with_prompt("Passwort bestätigen")
            .interact()?;

        if new_password == confirm_password {
            return Ok(new_password);
        } else {
            eprintln!("❌ Passwörter stimmen nicht überein. Bitte erneut versuchen.\n");
        }
    }
}

/// Ändert das Passwort des Benutzers
async fn change_password(
    server: &str,
    token: &str,
    public_key: &RsaPublicKey,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔒 Passwort muss geändert werden");

    let new_password = prompt_new_password()?;
    let encrypted_password = encrypt_password(&new_password, public_key)?;

    let client = reqwest::Client::new();
    let url = format!("{}/api/change-password", server.trim_end_matches('/'));

    let response = client
        .post(&url)
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .json(&ChangePasswordRequest {
            new_password: encrypted_password,
        })
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ Passwort erfolgreich geändert!");
        Ok(())
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        Err(format!(
            "Fehler beim Ändern des Passworts: {} - {}",
            status, error_text
        )
        .into())
    }
}

pub async fn login() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 CSF Login");
    println!();

    // Lade existierende Config falls vorhanden
    let mut config = load_config().unwrap_or_else(|| Config {
        server: String::new(),
        token: None,
    });

    // Frage nach Server (mit default falls schon gespeichert)
    let server: String = if config.server.is_empty() {
        Input::new()
            .with_prompt("Server URL (z.B. http://192.168.1.36:8000)")
            .interact_text()?
    } else {
        Input::new()
            .with_prompt("Server URL")
            .default(config.server.clone())
            .interact_text()?
    };

    // Hole den öffentlichen Schlüssel vom Server
    println!("🔑 Hole Public Key vom Server...");
    let public_key = get_public_key(&server).await?;

    // Frage nach Benutzername
    let username: String = Input::new().with_prompt("Benutzername").interact_text()?;

    // Frage nach Passwort (wird nicht angezeigt)
    let password = Password::new().with_prompt("Passwort").interact()?;

    // Verschlüssele das Passwort
    let encrypted_password = encrypt_password(&password, &public_key)?;

    // Frage nach 2FA Code (optional)
    let two_factor_code: String = Input::new()
        .with_prompt("2FA Code (leer lassen wenn nicht benötigt)")
        .allow_empty(true)
        .interact_text()?;

    println!();
    println!("⏳ Authentifizierung läuft...");

    // Sende Login-Request an Backend
    let client = reqwest::Client::new();
    let login_url = format!("{}/api/login", server.trim_end_matches('/'));

    println!("📡 Request: POST {}", login_url);
    println!("📤 Payload:");
    println!("   - username: {}", username);
    println!(
        "   - encrypted_password: {}... ({} Zeichen)",
        &encrypted_password.chars().take(20).collect::<String>(),
        encrypted_password.len()
    );
    println!(
        "   - two_factor_code: {}",
        if two_factor_code.is_empty() {
            "(leer)"
        } else {
            "***"
        }
    );

    let response = client
        .post(&login_url)
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&LoginRequest {
            username: username.clone(),
            encrypted_password: encrypted_password.clone(),
            two_factor_code: two_factor_code.clone(),
        })
        .send()
        .await?;

    println!("📥 Response Status: {}", response.status());

    if response.status().is_success() {
        let response_text = response.text().await?;
        println!("📥 Response Body:\n{}", response_text);

        let login_response: LoginResponse = serde_json::from_str(&response_text)?;

        // Speichere Server und Token
        config.server = server.clone();
        config.token = Some(login_response.token.clone());
        save_config(&config)?;

        println!("✅ Login erfolgreich!");
        println!("   User: {}", login_response.username);
        println!("   User ID: {}", login_response.user_id);

        if login_response.two_factor_enabled {
            println!("   🔐 2FA ist aktiviert");
        }

        // Prüfe ob Passwort geändert werden muss
        if login_response.force_password_change {
            change_password(&server, &login_response.token, &public_key).await?;
        }

        println!("\n✅ Anmeldung abgeschlossen!");
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        eprintln!("❌ Login fehlgeschlagen: {} - {}", status, error_text);
        std::process::exit(1);
    }

    Ok(())
}
