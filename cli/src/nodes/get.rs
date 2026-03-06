use crate::display::{self, kv, kv_colored, section, status_color};
use crate::http::{auth, base_url, get_json};
use serde::Deserialize;

#[derive(Deserialize)]
struct Agent {
    id: String,
    name: String,
    hostname: String,
    ip_address: Option<String>,
    os_type: String,
    os_version: String,
    architecture: String,
    agent_version: String,
    status: String,
    last_heartbeat: Option<String>,
    registered_at: String,
}

pub async fn run(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/agents/{}", base_url(&server), id);

    let pb = display::spinner("fetching node...");
    let data = get_json(&client, &url, &token).await;
    pb.finish_and_clear();

    let data = data?;
    let a: Agent = serde_json::from_value(data)?;

    section("Nodes  /  Get");

    kv("ID", &a.id);
    kv("Name", &a.name);
    kv("Hostname", &a.hostname);
    kv("IP", a.ip_address.as_deref().unwrap_or("-"));
    kv("OS", &format!("{} {}", a.os_type, a.os_version));
    kv("Architecture", &a.architecture);
    kv("Agent Version", &a.agent_version);
    kv_colored("Status", &a.status, status_color(&a.status));
    kv(
        "Last Heartbeat",
        a.last_heartbeat
            .as_deref()
            .map(|t| &t[..16.min(t.len())])
            .unwrap_or("never"),
    );
    kv(
        "Registered",
        &a.registered_at[..16.min(a.registered_at.len())],
    );

    println!();

    Ok(())
}
