use std::{collections::BTreeMap, fmt::Display, path::Path};

use anyhow::Context;

use crate::state::State;

pub struct SshConfigService {}

impl SshConfigService {
    // Get list of hostnames
    #[tracing::instrument(skip(self), level = "trace")]
    pub async fn get_ssh_items(&self, ssh_config_path: &Path) -> anyhow::Result<SshItems> {
        if !ssh_config_path.exists() {
            anyhow::bail!(
                "was unable to find ssh config file at the given path: {}",
                ssh_config_path.display()
            )
        }

        tracing::trace!("reading ssh config");

        // 1. get ssh config
        let ssh_config_content = tokio::fs::read_to_string(ssh_config_path)
            .await
            .context("failed to read ssh config file, check that it a normal ssh config file")?;

        // 2. parse what we care about
        let ssh_config_lines = ssh_config_content.lines();

        let ssh_config_hosts = ssh_config_lines
            .into_iter()
            .filter(|item| item.starts_with("Host "))
            .map(|item| item.trim_start_matches("Host ").trim_start().trim_end())
            .collect::<Vec<_>>();

        // 3. model into our own definition
        let ssh_items: BTreeMap<String, SshItem> = ssh_config_hosts
            .into_iter()
            .map(|s| (s.to_string(), s.into()))
            .collect();

        Ok(SshItems { items: ssh_items })
    }
}

#[derive(Debug, Clone)]
pub enum SshItem {
    Own(String),
    Raw { contents: Vec<String> },
}

impl SshItem {
    pub fn to_host(&self) -> Vec<&str> {
        match self {
            SshItem::Own(own) => vec![own],
            SshItem::Raw { contents } => contents.iter().map(|c| c.as_str()).collect(),
        }
    }
}

impl Display for SshItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let host = match self {
            SshItem::Own(o) => o.to_string(),
            SshItem::Raw { contents } => contents.join(" "),
        };

        f.write_str(&host)
    }
}

#[derive(Debug)]
pub struct SshItems {
    pub items: BTreeMap<String, SshItem>,
}

impl SshItems {
    pub fn to_vec(&self) -> Vec<&SshItem> {
        self.items.values().collect()
    }

    pub fn get_choice(&self, choice: &str) -> Option<&SshItem> {
        self.items.get(choice)
    }

    pub fn append(&mut self, other: &mut SshItems) {
        self.items.append(&mut other.items);
    }
}

impl From<&str> for SshItem {
    fn from(value: &str) -> Self {
        Self::Own(value.to_string())
    }
}

pub trait SshConfigServiceState {
    fn ssh_config_service(&self) -> SshConfigService;
}

impl SshConfigServiceState for State {
    fn ssh_config_service(&self) -> SshConfigService {
        SshConfigService {}
    }
}
