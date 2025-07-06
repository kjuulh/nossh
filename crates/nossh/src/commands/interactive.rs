use std::path::Path;

use anyhow::Context;

use crate::{
    ssh_command::SshCommandState, ssh_command_database::SshCommandDatabaseState,
    ssh_config::SshConfigServiceState, state::State, user_fuzzy_find::UserFuzzyFindState,
};

#[derive(clap::Parser, Default)]
pub struct InteractiveCommand {}

impl InteractiveCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn execute(&self, state: &State, ssh_config_path: &Path) -> anyhow::Result<()> {
        // 1. Get a list of items in ~/.ssh/config
        let mut items = state
            .ssh_config_service()
            .get_ssh_items(ssh_config_path)
            .await
            .context("failed to get ssh items")?;

        let mut database_items = state.ssh_command_database().get_items().await?;
        items.append(&mut database_items);

        tracing::trace!("found ssh items: {:#?}", items);

        // 2. Present the list, and allow the user to choose an item
        let item = state
            .user_fuzzy_find()
            .get_ssh_item_from_user(&items)
            .await?;

        tracing::debug!("found ssh item: '{}'", item);

        // 3. Perform ssh
        // call the cmdline parse in all pipes, with the hostname as the destination
        // ssh something
        state.ssh_command().start_ssh_session(item).await?;

        Ok(())
    }
}
