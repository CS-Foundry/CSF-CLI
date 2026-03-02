use crate::display::{self, Table};
use crate::http::{auth, base_url, get_json};
use serde::Deserialize;

#[derive(Deserialize)]
struct Token {
    id: String,
    description: Option<String>,
    created_at: String,
    expires_at: String,
    used: bool,
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/registry/admin/tokens", base_url(&server));

    let pb = display::spinner("fetching registration tokens...");
    let data = get_json(&client, &url, &token).await;
    pb.finish_and_clear();

    let data = data?;
    let tokens: Vec<Token> = serde_json::from_value(data)?;

    display::section("Registry  /  Registration Tokens");

    if tokens.is_empty() {
        display::info("no tokens found");
        return Ok(());
    }

    let mut table =
        Table::new(vec!["ID", "DESCRIPTION", "CREATED", "EXPIRES", "USED"]).with_color(
            |col, val| {
                if col == 4 {
                    if val == "yes" {
                        colored::Color::Green
                    } else {
                        colored::Color::Yellow
                    }
                } else {
                    colored::Color::White
                }
            },
        );

    for t in &tokens {
        table.add_row(vec![
            t.id[..8].to_string(),
            t.description.clone().unwrap_or_else(|| "-".to_string()),
            t.created_at.clone(),
            t.expires_at.clone(),
            if t.used { "yes" } else { "no" }.to_string(),
        ]);
    }

    table.print();
    println!();
    display::info(&format!("{} token(s)", tokens.len()));

    Ok(())
}
