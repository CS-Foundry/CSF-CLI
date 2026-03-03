use crate::display::{self, kv, section};
use crate::http::{auth, base_url, get_json};
use colored::Colorize;
use serde::Deserialize;

#[derive(Deserialize)]
struct Metrics {
    hostname: Option<String>,
    os_name: Option<String>,
    os_version: Option<String>,
    kernel_version: Option<String>,
    uptime_seconds: Option<u64>,
    cpu_model: Option<String>,
    cpu_cores: Option<u32>,
    cpu_threads: Option<u32>,
    cpu_usage_percent: Option<f32>,
    memory_total_bytes: Option<u64>,
    memory_used_bytes: Option<u64>,
    memory_usage_percent: Option<f32>,
    disk_total_bytes: Option<u64>,
    disk_used_bytes: Option<u64>,
    disk_usage_percent: Option<f32>,
    network_rx_bytes: Option<u64>,
    network_tx_bytes: Option<u64>,
}

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1_073_741_824.0
}

fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    format!("{}d {}h {}m", days, hours, minutes)
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

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/system/metrics", base_url(&server));

    let pb = display::spinner("fetching system metrics...");
    let data = get_json(&client, &url, &token).await;
    pb.finish_and_clear();

    let data = data?;

    let metrics: Metrics = if let Some(inner) = data.get("metrics") {
        serde_json::from_value(inner.clone())?
    } else {
        serde_json::from_value(data)?
    };

    section("System Metrics");

    if let Some(ref h) = metrics.hostname {
        kv("Hostname", h);
    }
    if let Some(ref os) = metrics.os_name {
        kv(
            "OS",
            &format!(
                "{} {}",
                os,
                metrics.os_version.as_deref().unwrap_or("")
            ),
        );
    }
    if let Some(ref k) = metrics.kernel_version {
        kv("Kernel", k);
    }
    if let Some(uptime) = metrics.uptime_seconds {
        kv("Uptime", &format_uptime(uptime));
    }

    println!();

    if let Some(ref model) = metrics.cpu_model {
        kv("CPU Model", model);
    }
    if let (Some(cores), Some(threads)) = (metrics.cpu_cores, metrics.cpu_threads) {
        kv("CPU", &format!("{} cores / {} threads", cores, threads));
    }
    if let Some(pct) = metrics.cpu_usage_percent {
        println!("  {:<20} {}", "CPU Usage".dimmed(), usage_bar(pct, 30));
    }

    println!();

    if let (Some(total), Some(used)) = (metrics.memory_total_bytes, metrics.memory_used_bytes) {
        kv(
            "Memory",
            &format!(
                "{:.1}G / {:.1}G",
                bytes_to_gb(used),
                bytes_to_gb(total)
            ),
        );
    }
    if let Some(pct) = metrics.memory_usage_percent {
        println!("  {:<20} {}", "Memory Usage".dimmed(), usage_bar(pct, 30));
    }

    println!();

    if let (Some(total), Some(used)) = (metrics.disk_total_bytes, metrics.disk_used_bytes) {
        kv(
            "Disk",
            &format!(
                "{:.1}G / {:.1}G",
                bytes_to_gb(used),
                bytes_to_gb(total)
            ),
        );
    }
    if let Some(pct) = metrics.disk_usage_percent {
        println!("  {:<20} {}", "Disk Usage".dimmed(), usage_bar(pct, 30));
    }

    println!();

    if let (Some(rx), Some(tx)) = (metrics.network_rx_bytes, metrics.network_tx_bytes) {
        kv(
            "Network RX",
            &format!("{:.2} MB", rx as f64 / 1_048_576.0),
        );
        kv(
            "Network TX",
            &format!("{:.2} MB", tx as f64 / 1_048_576.0),
        );
    }

    println!();
    Ok(())
}
