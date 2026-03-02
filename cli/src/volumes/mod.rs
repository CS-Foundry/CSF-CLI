pub mod list;
pub mod get;
pub mod snapshots;
pub mod nodes;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum VolumeCommands {
    List,
    Get { id: String },
    Snapshots,
    Nodes,
}

pub async fn run(cmd: VolumeCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        VolumeCommands::List => list::run().await,
        VolumeCommands::Get { id } => get::run(&id).await,
        VolumeCommands::Snapshots => snapshots::run().await,
        VolumeCommands::Nodes => nodes::run().await,
    }
}
