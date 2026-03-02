pub mod agents;
pub mod pending;
pub mod stats;
pub mod tokens;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum RegistryCommands {
    Agents,
    #[command(name = "agents-get")]
    AgentsGet { id: String },
    Pending,
    Stats,
    Tokens,
}

pub async fn run(cmd: RegistryCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        RegistryCommands::Agents => agents::list().await,
        RegistryCommands::AgentsGet { id } => agents::get(&id).await,
        RegistryCommands::Pending => pending::run().await,
        RegistryCommands::Stats => stats::run().await,
        RegistryCommands::Tokens => tokens::run().await,
    }
}
