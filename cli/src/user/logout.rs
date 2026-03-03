use crate::config::{load_config, save_config};
use crate::display;

pub async fn logout() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config();

    match config {
        Some(mut cfg) => {
            cfg.token = None;
            save_config(&cfg)?;
            display::success("logged out");
            display::kv("Server", &cfg.server);
        }
        None => {
            display::warn("not logged in");
        }
    }

    Ok(())
}
