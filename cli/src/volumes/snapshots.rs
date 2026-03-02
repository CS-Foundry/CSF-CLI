use crate::display::{self, Table};
use crate::http::{auth, base_url, get_json};
use serde::Deserialize;

#[derive(Deserialize)]
struct Snapshot {
    id: String,
    volume_id: String,
    name: String,
    size_gb: u64,
    status: String,
    created_at: String,
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/volumes/snapshots", base_url(&server));

    let pb = display::spinner("fetching snapshots...");
    let data = get_json(&client, &url, &token).await;
    pb.finish_and_clear();

    let data = data?;
    let snapshots: Vec<Snapshot> = serde_json::from_value(data)?;

    display::section("Snapshots");

    if snapshots.is_empty() {
        display::info("no snapshots found");
        return Ok(());
    }

    let mut table = Table::new(vec!["ID", "NAME", "VOLUME", "SIZE", "STATUS", "CREATED"])
        .with_color(|col, val| {
            if col == 4 {
                display::status_color(val)
            } else {
                colored::Color::White
            }
        });

    for s in &snapshots {
        table.add_row(vec![
            s.id[..8].to_string(),
            s.name.clone(),
            s.volume_id[..8].to_string(),
            format!("{}G", s.size_gb),
            s.status.clone(),
            s.created_at.clone(),
        ]);
    }

    table.print();
    println!();
    display::info(&format!("{} snapshot(s)", snapshots.len()));

    Ok(())
}
