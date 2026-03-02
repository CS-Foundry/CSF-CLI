pub mod list;
pub mod metrics;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum NodeCommands {
    List,
    Metrics,
}

pub async fn run(cmd: NodeCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        NodeCommands::List => list::run().await,
        NodeCommands::Metrics => metrics::run().await,
    }
}
