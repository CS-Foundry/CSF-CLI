mod config;
mod display;
mod http;
mod nodes;
mod registry;
mod user;
mod volumes;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "csf")]
#[command(about = "Cloud Service Foundry CLI")]
#[command(version)]
#[command(disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Login,
    Logout,
    Status,
    #[command(subcommand, about = "Manage storage volumes")]
    Volumes(volumes::VolumeCommands),
    #[command(subcommand, about = "View registry and agents")]
    Registry(registry::RegistryCommands),
    #[command(subcommand, about = "View cluster nodes and metrics")]
    Nodes(nodes::NodeCommands),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    display::banner();

    let cli = Cli::parse();

    match cli.command {
        Commands::Login => user::login::login().await?,
        Commands::Logout => user::logout::logout().await?,
        Commands::Status => user::status::status().await?,
        Commands::Volumes(cmd) => volumes::run(cmd).await?,
        Commands::Registry(cmd) => registry::run(cmd).await?,
        Commands::Nodes(cmd) => nodes::run(cmd).await?,
    }

    Ok(())
}
