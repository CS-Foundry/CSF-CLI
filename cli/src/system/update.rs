use crate::config::load_config;
use crate::display;
use crate::http::{auth, base_url, get_json};
use std::process::Stdio;
use tokio::process::Command;

pub async fn run(version: String) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config().ok_or("not configured, run: csfx login")?;

    let compose_dir = config
        .compose_dir
        .ok_or("compose_dir not set in ~/.csf/config.json")?;
    let ghcr_org = config
        .ghcr_org
        .ok_or("ghcr_org not set in ~/.csf/config.json")?;

    let compose_file = format!("{}/docker-compose.prod.yml", compose_dir.trim_end_matches('/'));

    if ghcr_org != "local" {
        display::info(&format!("pulling images for version {}", version));
        run_compose(&compose_file, &ghcr_org, &version, &["pull"]).await?;
    } else {
        display::info("local org detected, skipping pull");
    }

    display::info("restarting services");
    run_compose(&compose_file, &ghcr_org, &version, &["up", "-d"]).await?;

    display::success(&format!("updated to {}", version));
    Ok(())
}

async fn run_compose(
    compose_file: &str,
    ghcr_org: &str,
    version: &str,
    args: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd_args = vec!["compose", "-f", compose_file];
    cmd_args.extend_from_slice(args);

    let status = Command::new("docker")
        .args(&cmd_args)
        .env("GHCR_ORG", ghcr_org)
        .env("CSF_VERSION", version)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await?;

    if !status.success() {
        return Err(format!("docker compose {} failed: exit {}", args.join(" "), status).into());
    }
    Ok(())
}

pub async fn run_status() -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/system/update/status", base_url(&server));

    let resp = get_json(&client, &url, &token).await?;

    display::section("Control Plane Update Status");
    display::kv("current", resp["current_version"].as_str().unwrap_or("-"));
    display::kv("desired", resp["desired_version"].as_str().unwrap_or("-"));

    let result = resp["last_result"].as_str().unwrap_or("-");
    let color = match result {
        "success" => colored::Color::Green,
        "failed" => colored::Color::Red,
        "in_progress" => colored::Color::Yellow,
        _ => colored::Color::White,
    };
    display::kv_colored("last result", result, color);

    Ok(())
}
