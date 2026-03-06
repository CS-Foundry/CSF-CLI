pub mod agent_metrics;
pub mod get;
pub mod list;
pub mod metrics;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum NodeCommands {
    List,
    Get { id: String },
    Metrics,
    AgentMetrics { id: String },
}

pub async fn run(cmd: NodeCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        NodeCommands::List => list::run().await,
        NodeCommands::Get { id } => get::run(&id).await,
        NodeCommands::Metrics => metrics::run().await,
        NodeCommands::AgentMetrics { id } => agent_metrics::run(&id).await,
    }
}
