use crate::display::{self, kv, kv_colored, section, status_color};
use crate::http::{auth, base_url, get_json};
use serde::Deserialize;

#[derive(Deserialize)]
struct Volume {
    id: String,
    name: String,
    size_gb: u64,
    pool: String,
    status: String,
    encrypted: bool,
    node_id: Option<String>,
    created_at: String,
    updated_at: String,
    version: u64,
}

pub async fn run(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/volumes/{}", base_url(&server), id);

    let pb = display::spinner("fetching volume...");
    let data = get_json(&client, &url, &token).await;
    pb.finish_and_clear();

    let data = data?;
    let v: Volume = serde_json::from_value(data)?;

    section("Volume");

    kv("ID", &v.id);
    kv("Name", &v.name);
    kv("Size", &format!("{}G", v.size_gb));
    kv("Pool", &v.pool);
    kv_colored("Status", &v.status, status_color(&v.status));
    kv(
        "Encrypted",
        if v.encrypted { "yes" } else { "no" },
    );
    kv(
        "Node",
        &v.node_id.unwrap_or_else(|| "unattached".to_string()),
    );
    kv("Version", &v.version.to_string());
    kv("Created", &v.created_at);
    kv("Updated", &v.updated_at);

    println!();
    Ok(())
}
