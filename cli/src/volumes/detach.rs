use crate::display::{self, kv, kv_colored, section, status_color};
use crate::http::{auth, base_url, post_json};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct Volume {
    id: String,
    name: String,
    status: String,
}

pub async fn run(volume_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/volumes/{}/detach", base_url(&server), volume_id);

    let pb = display::spinner("detaching volume...");
    let data = post_json(&client, &url, &token, &Value::Null).await;
    pb.finish_and_clear();

    let v: Volume = serde_json::from_value(data?)?;

    section("Volume Detached");
    kv("ID", &v.id);
    kv("Name", &v.name);
    kv_colored("Status", &v.status, status_color(&v.status));
    println!();
    display::success("volume detached");

    Ok(())
}
