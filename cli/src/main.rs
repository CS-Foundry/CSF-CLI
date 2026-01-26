mod config;
mod user;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "csf")]
#[command(about = "CSF CLI Tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Login zum CSF Backend Service
    Login,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Login => {
            user::login::login().await?;
        }
    }

    Ok(())
}
