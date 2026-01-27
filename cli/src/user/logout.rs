use crate::config::{load_config, save_config, Config};

pub async fn logout() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔓 CSF Logout");
    println!();

    // Lade existierende Config
    let config = load_config();

    match config {
        Some(mut cfg) => {
            if cfg.token.is_some() {
                // Lösche das Token, behalte aber den Server
                cfg.token = None;
                save_config(&cfg)?;
                println!("✅ Erfolgreich abgemeldet!");
                println!("   Server-URL bleibt gespeichert: {}", cfg.server);
            } else {
                println!("ℹ️  Du bist nicht angemeldet.");
            }
        }
        None => {
            println!("ℹ️  Keine Konfiguration gefunden. Du bist nicht angemeldet.");
        }
    }

    Ok(())
}
