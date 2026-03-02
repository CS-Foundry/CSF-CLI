use crate::display::{self, Table};
use crate::http::{auth, base_url, get_json};
use serde::Deserialize;

#[derive(Deserialize)]
struct PendingAgent {
    id: String,
    name: String,
    hostname: String,
    expected_os_type: Option<String>,
    expected_architecture: Option<String>,
    created_by: String,
    token_expires_at: String,
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/registry/admin/agents/pending", base_url(&server));

    let pb = display::spinner("fetching pending agents...");
    let data = get_json(&client, &url, &token).await;
    pb.finish_and_clear();

    let data = data?;
    let agents: Vec<PendingAgent> = serde_json::from_value(data)?;

    display::section("Registry  /  Pending Agents");

    if agents.is_empty() {
        display::info("no pending agents");
        return Ok(());
    }

    let mut table = Table::new(vec![
        "ID",
        "NAME",
        "HOSTNAME",
        "EXPECTED OS",
        "ARCH",
        "CREATED BY",
        "TOKEN EXPIRES",
    ]);

    for a in &agents {
        table.add_row(vec![
            a.id[..8].to_string(),
            a.name.clone(),
            a.hostname.clone(),
            a.expected_os_type.clone().unwrap_or_else(|| "-".to_string()),
            a.expected_architecture
                .clone()
                .unwrap_or_else(|| "-".to_string()),
            a.created_by.clone(),
            a.token_expires_at.clone(),
        ]);
    }

    table.print();
    println!();
    display::info(&format!("{} pending agent(s)", agents.len()));

    Ok(())
}
