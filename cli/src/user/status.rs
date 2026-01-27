use crate::config::load_config;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct UserInfo {
    user_id: String,
    username: String,
    email: Option<String>,
    two_factor_enabled: bool,
}

pub async fn status() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 CSF Status");
    println!();

    // Lade existierende Config
    let config = load_config();

    match config {
        Some(cfg) => {
            println!("Server: {}", cfg.server);

            match &cfg.token {
                Some(token) => {
                    println!("Status: ✅ Angemeldet");
                    println!();

                    // Versuche User-Informationen vom Server zu holen
                    println!("⏳ Hole Benutzerinformationen...");

                    let client = reqwest::Client::new();
                    let url = format!("{}/api/user/me", cfg.server.trim_end_matches('/'));

                    let response = client
                        .get(&url)
                        .header("accept", "application/json")
                        .header("Authorization", format!("Bearer {}", token))
                        .send()
                        .await?;

                    if response.status().is_success() {
                        let user_info: UserInfo = response.json().await?;
                        println!();
                        println!("Benutzerinformationen:");
                        println!("   User ID: {}", user_info.user_id);
                        println!("   Benutzername: {}", user_info.username);

                        if let Some(email) = user_info.email {
                            println!("   E-Mail: {}", email);
                        }

                        println!(
                            "   2FA: {}",
                            if user_info.two_factor_enabled {
                                "✅ Aktiviert"
                            } else {
                                "❌ Nicht aktiviert"
                            }
                        );
                    } else {
                        let status = response.status();
                        println!();
                        println!(
                            "⚠️  Token ist möglicherweise abgelaufen (Status: {})",
                            status
                        );
                        println!("   Bitte melde dich erneut an mit: csf login");
                    }
                }
                None => {
                    println!("Status: ❌ Nicht angemeldet");
                    println!();
                    println!("💡 Melde dich an mit: csf login");
                }
            }
        }
        None => {
            println!("Status: ❌ Keine Konfiguration gefunden");
            println!();
            println!("💡 Melde dich an mit: csf login");
        }
    }

    Ok(())
}
