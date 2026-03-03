use crate::display::{self, kv};
use crate::http::{auth, base_url, post_json};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct PreRegisterResponse {
    agent_id: String,
    registration_token: String,
    token_expires_at: String,
}

pub async fn run(
    name: String,
    hostname: String,
    os_type: Option<String>,
    architecture: Option<String>,
    ttl_hours: Option<i64>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/registry/admin/agents/pre-register", base_url(&server));

    let body = json!({
        "name": name,
        "hostname": hostname,
        "expected_os_type": os_type,
        "expected_architecture": architecture,
        "ttl_hours": ttl_hours.unwrap_or(24),
    });

    let pb = display::spinner("pre-registering agent...");
    let data = post_json(&client, &url, &token, &body).await;
    pb.finish_and_clear();

    let data = data?;
    let resp: PreRegisterResponse = serde_json::from_value(data)?;

    display::section("Registry  /  Agent Pre-Registered");

    kv("Agent ID", &resp.agent_id);
    kv("Token Expires", &resp.token_expires_at);
    println!();
    println!("  {}", "Registration Token".bold());
    println!("  {}", resp.registration_token.cyan().bold());
    println!();
    display::info("embed this token in the NixOS configuration for this node");
    display::info("the token is single-use and expires after the configured TTL");

    Ok(())
}

use colored::Colorize;
