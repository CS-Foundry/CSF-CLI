use crate::display::{self, kv, section};
use crate::http::{auth, base_url, get_json};
use colored::Colorize;
use serde::Deserialize;

#[derive(Deserialize)]
struct AgentMetrics {
    cpu_usage_percent: Option<f32>,
    memory_total_bytes: Option<i64>,
    memory_used_bytes: Option<i64>,
    disk_total_bytes: Option<i64>,
    disk_used_bytes: Option<i64>,
    network_rx_bytes: Option<i64>,
    network_tx_bytes: Option<i64>,
    timestamp: Option<String>,
}

fn usage_bar(pct: f32, width: usize) -> String {
    let filled = ((pct / 100.0) * width as f32) as usize;
    let empty = width - filled.min(width);
    let color = if pct > 90.0 {
        colored::Color::Red
    } else if pct > 70.0 {
        colored::Color::Yellow
    } else {
        colored::Color::Green
    };
    format!(
        "[{}{}] {:.1}%",
        "#".repeat(filled).color(color),
        ".".repeat(empty).dimmed(),
        pct
    )
}

pub async fn run(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/agents/{}/metrics", base_url(&server), id);

    let pb = display::spinner("fetching agent metrics...");
    let data = get_json(&client, &url, &token).await;
    pb.finish_and_clear();

    let data = data?;
    let m: AgentMetrics = serde_json::from_value(data)?;

    section("Nodes  /  Agent Metrics");

    if let Some(ts) = &m.timestamp {
        kv("Timestamp", &ts[..16.min(ts.len())]);
        println!();
    }

    if let Some(pct) = m.cpu_usage_percent {
        println!("  {:<20} {}", "CPU Usage".dimmed(), usage_bar(pct, 30));
    }

    println!();

    if let (Some(total), Some(used)) = (m.memory_total_bytes, m.memory_used_bytes) {
        kv(
            "Memory",
            &format!(
                "{:.1}G / {:.1}G",
                used as f64 / 1_073_741_824.0,
                total as f64 / 1_073_741_824.0
            ),
        );
        let pct = used as f32 / total as f32 * 100.0;
        println!("  {:<20} {}", "Memory Usage".dimmed(), usage_bar(pct, 30));
    }

    println!();

    if let (Some(total), Some(used)) = (m.disk_total_bytes, m.disk_used_bytes) {
        kv(
            "Disk",
            &format!(
                "{:.1}G / {:.1}G",
                used as f64 / 1_073_741_824.0,
                total as f64 / 1_073_741_824.0
            ),
        );
        let pct = used as f32 / total as f32 * 100.0;
        println!("  {:<20} {}", "Disk Usage".dimmed(), usage_bar(pct, 30));
    }

    println!();

    if let (Some(rx), Some(tx)) = (m.network_rx_bytes, m.network_tx_bytes) {
        kv("Network RX", &format!("{:.2} MB", rx as f64 / 1_048_576.0));
        kv("Network TX", &format!("{:.2} MB", tx as f64 / 1_048_576.0));
    }

    println!();

    Ok(())
}
