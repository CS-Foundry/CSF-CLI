pub mod ghcr_token;
pub mod releases;
pub mod stats;
pub mod update;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum SystemCommands {
    Stats {
        #[arg(long, short)]
        watch: bool,
    },
    Update {
        version: String,
    },
    UpdateStatus,
    UpdatePause,
    UpdateResume,
    CheckUpdate {
        #[arg(long, help = "include pre-release versions")]
        pre: bool,
    },
    SetGhcrToken,
}

pub async fn run(cmd: SystemCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        SystemCommands::Stats { watch } => stats::run(watch).await,
        SystemCommands::Update { version } => update::run(version).await,
        SystemCommands::UpdateStatus => update::run_status().await,
        SystemCommands::UpdatePause => update::run_pause().await,
        SystemCommands::UpdateResume => update::run_resume().await,
        SystemCommands::CheckUpdate { pre } => releases::run(pre).await,
        SystemCommands::SetGhcrToken => ghcr_token::run().await,
    }
}
