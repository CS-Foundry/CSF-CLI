use crate::display::{self, Table};
use crate::http::{auth, base_url, get_json};
use serde::Deserialize;

#[derive(Deserialize)]
struct Node {
    node_id: String,
    hostname: String,
    ip_address: String,
    status: String,
    role: String,
    last_heartbeat: String,
    volumes: Vec<String>,
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/volumes/nodes", base_url(&server));

    let pb = display::spinner("fetching cluster nodes...");
    let data = get_json(&client, &url, &token).await;
    pb.finish_and_clear();

    let data = data?;
    let nodes: Vec<Node> = serde_json::from_value(data)?;

    display::section("Cluster Nodes");

    if nodes.is_empty() {
        display::info("no nodes found");
        return Ok(());
    }

    let mut table =
        Table::new(vec!["NODE ID", "HOSTNAME", "IP", "ROLE", "STATUS", "VOLUMES", "HEARTBEAT"])
            .with_color(|col, val| {
                if col == 4 {
                    display::status_color(val)
                } else if col == 3 {
                    if val == "leader" || val == "Leader" {
                        colored::Color::Cyan
                    } else {
                        colored::Color::White
                    }
                } else {
                    colored::Color::White
                }
            });

    for n in &nodes {
        table.add_row(vec![
            n.node_id[..8.min(n.node_id.len())].to_string(),
            n.hostname.clone(),
            n.ip_address.clone(),
            n.role.clone(),
            n.status.clone(),
            n.volumes.len().to_string(),
            n.last_heartbeat.clone(),
        ]);
    }

    table.print();
    println!();
    display::info(&format!("{} node(s)", nodes.len()));

    Ok(())
}
