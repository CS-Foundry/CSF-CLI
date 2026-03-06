use crate::display::{self, section, status_color, Table};
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
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/networks", base_url(&server));

    let pb = display::spinner("fetching networks...");
    let data = get_json(&client, &url, &token).await;
    pb.finish_and_clear();

    let data = data?;
    let networks: Vec<Network> = serde_json::from_value(data)?;

    section("Networks");

    if networks.is_empty() {
        display::info("no networks");
        return Ok(());
    }

    let mut table = Table::new(vec!["ID", "NAME", "CIDR", "OVERLAY", "STATUS", "CREATED"])
        .with_color(|col, val| {
            if col == 4 {
                status_color(val)
            } else {
                colored::Color::White
            }
        });

    for n in &networks {
        table.add_row(vec![
            n.id[..8].to_string(),
            n.name.clone(),
            n.cidr.clone(),
            n.overlay_type.clone(),
            n.status.clone(),
            n.created_at[..16.min(n.created_at.len())].to_string(),
        ]);
    }

    table.print();
    println!();
    display::info(&format!("{} network(s)", networks.len()));

    Ok(())
}
