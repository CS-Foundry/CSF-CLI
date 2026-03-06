use crate::display::{self, kv, kv_colored, section, status_color};
use crate::http::{auth, base_url, get_json};
use serde::Deserialize;

#[derive(Deserialize)]
struct Network {
    id: String,
    name: String,
    cidr: String,
    overlay_type: String,
    status: String,
    created_at: String,
    updated_at: Option<String>,
}

pub async fn run(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/networks/{}", base_url(&server), id);

    let pb = display::spinner("fetching network...");
    let data = get_json(&client, &url, &token).await;
    pb.finish_and_clear();

    let data = data?;
    let n: Network = serde_json::from_value(data)?;

    section("Networks  /  Details");

    kv("ID", &n.id);
    kv("Name", &n.name);
    kv("CIDR", &n.cidr);
    kv("Overlay", &n.overlay_type);
    kv_colored("Status", &n.status, status_color(&n.status));
    kv("Created", &n.created_at);
    kv("Updated", n.updated_at.as_deref().unwrap_or("-"));

    println!();

    Ok(())
}
