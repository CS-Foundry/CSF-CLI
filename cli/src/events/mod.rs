pub mod list;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum EventCommands {
    List,
}

pub async fn run(cmd: EventCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        EventCommands::List => list::run().await,
    }
}
