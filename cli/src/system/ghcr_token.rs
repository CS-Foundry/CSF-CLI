use crate::display;
use crate::http::{auth, base_url};
use serde_json::json;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;

    let username = dialoguer::Input::<String>::new()
        .with_prompt("GitHub username")
        .interact_text()?;

    let ghcr_token = dialoguer::Password::new()
        .with_prompt("GitHub personal access token (read:packages)")
        .interact()?;

    if ghcr_token.is_empty() {
        return Err("token must not be empty".into());
    }

    let url = format!("{}/system/ghcr-token", base_url(&server));
    let resp = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&json!({ "username": username, "token": ghcr_token }))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await?;
        crate::display::error(&format!("request failed: {} {}", status, body));
        std::process::exit(1);
    }

    display::success("ghcr token stored");
    Ok(())
}
