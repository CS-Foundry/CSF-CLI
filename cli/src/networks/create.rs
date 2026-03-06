use crate::display::{self, kv, kv_colored, section, status_color};
use crate::http::{auth, base_url, post_json};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Network {
    id: String,
    name: String,
    cidr: String,
    overlay_type: String,
    status: String,
}

pub async fn run(
    name: String,
    cidr: String,
    overlay_type: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/networks", base_url(&server));

    let body = json!({
        "name": name,
        "cidr": cidr,
        "overlay_type": overlay_type,
    });

    let pb = display::spinner("creating network...");
    let data = post_json(&client, &url, &token, &body).await;
    pb.finish_and_clear();

    let data = data?;
    let n: Network = serde_json::from_value(data)?;

    section("Networks  /  Create");

    kv("ID", &n.id);
    kv("Name", &n.name);
    kv("CIDR", &n.cidr);
    kv("Overlay", &n.overlay_type);
    kv_colored("Status", &n.status, status_color(&n.status));

    println!();
    display::success("network created");

    Ok(())
}
