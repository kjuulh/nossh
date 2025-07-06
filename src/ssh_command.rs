use std::process::Stdio;

use anyhow::Context;

use crate::{ssh_config::SshItem, state::State};

pub struct SshCommand {}

impl SshCommand {
    pub async fn start_ssh_session(&self, ssh_item: &SshItem) -> anyhow::Result<()> {
        let host = ssh_item.to_host();

        tracing::info!("starting ssh session at: {}", ssh_item);

        // ssh something
        let mut cmd = tokio::process::Command::new("ssh");
        cmd.arg(host);

        let mut process = cmd.spawn()?;
        let res = process.wait().await.context("ssh call failed");

        tracing::debug!("ssh call finished to host: {}", ssh_item);
        res?;

        Ok(())
    }

    pub async fn start_ssh_session_from_raw(&self, raw: Vec<&str>) -> anyhow::Result<()> {
        let pretty_raw = raw.join(" ");
        tracing::info!("starting ssh session at: {}", pretty_raw);

        let mut cmd = tokio::process::Command::new("ssh");
        cmd.args(raw);

        let mut process = cmd.spawn()?;
        let res = process.wait().await.context("ssh call failed");

        tracing::debug!("ssh call finished to host: {}", pretty_raw);
        res?;

        Ok(())
    }
}

pub trait SshCommandState {
    fn ssh_command(&self) -> SshCommand;
}

impl SshCommandState for State {
    fn ssh_command(&self) -> SshCommand {
        SshCommand {}
    }
}
