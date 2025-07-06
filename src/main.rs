use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use crate::{
    commands::interactive::InteractiveCommand, ssh_command::SshCommandState, state::State,
};

mod commands;

mod ssh_command;
mod ssh_config;
mod state;
mod user_fuzzy_find;

#[derive(Parser)]
#[command(author, version, about, allow_external_subcommands = true)]
pub struct Command {
    #[arg(long = "ssh-config-path", env = "SSH_CONFIG_PATH")]
    ssh_config_path: Option<PathBuf>,

    #[command(subcommand)]
    commands: Option<Subcommands>,
}

#[derive(clap::Subcommand)]
pub enum Subcommands {
    Interactive(InteractiveCommand),

    #[command(external_subcommand)]
    External(Vec<String>),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("nossh=debug".parse().unwrap()),
        )
        .init();

    let command = Command::parse();

    let state = State::new();

    tracing::debug!("welcome to nossh, your ssh manager");

    let ssh_config_path = match &command.ssh_config_path {
        Some(path) => path.to_path_buf(),
        None => {
            let home = std::env::var("HOME").context(
                "failed to find home, this is required if no SSH_CONFIG_PATH is provided",
            )?;

            PathBuf::from(home).join(".ssh").join("config")
        }
    };

    let Some(commands) = &command.commands else {
        InteractiveCommand::default()
            .execute(&state, &ssh_config_path)
            .await?;

        return Ok(());
    };

    match &commands {
        Subcommands::Interactive(cmd) => {
            tracing::debug!("running interactive");
            cmd.execute(&state, &ssh_config_path).await?;
        }
        Subcommands::External(items) => {
            // Send to ssh
            state
                .ssh_command()
                .start_ssh_session_from_raw(items.iter().map(|i| i.as_str()).collect())
                .await?;

            // Remember result
        }
    }

    Ok(())
}
