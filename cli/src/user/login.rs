use dialoguer::{Input, Password};
use reqwest;
use serde::{Deserialize, Serialize};

use crate::config::{load_config, save_config, Config};

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    encrypted_password: String,
    two_factor_code: String,
}

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    token: String,
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

    // Frage nach Benutzername
    let username: String = Input::new().with_prompt("Benutzername").interact_text()?;

    // Frage nach Passwort (wird nicht angezeigt)
    let password = Password::new().with_prompt("Passwort").interact()?;

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

    let response = client
        .post(&login_url)
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&LoginRequest {
            username,
            encrypted_password: password,
            two_factor_code,
        })
        .send()
        .await?;

    if response.status().is_success() {
        let login_response: LoginResponse = response.json().await?;

        // Speichere Server und Token
        config.server = server;
        config.token = Some(login_response.token);
        save_config(&config)?;

        println!("✅ Login erfolgreich!");
        println!("Token wurde gespeichert.");
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        eprintln!("❌ Login fehlgeschlagen: {} - {}", status, error_text);
        std::process::exit(1);
    }

    Ok(())
}
