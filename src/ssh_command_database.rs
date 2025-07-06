use std::{collections::BTreeMap, path::PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

use crate::{
    ssh_config::{SshItem, SshItems},
    state::State,
};

pub struct SshCommandDatabase {}

impl SshCommandDatabase {
    pub async fn get_items(&self) -> anyhow::Result<SshItems> {
        let database_path = self.ensure_get_database_file_path().await?;
        let database = if database_path.exists() {
            let content = tokio::fs::read(&database_path)
                .await
                .context("failed to read nossh database file")?;

            let database: Database = serde_json::from_slice(&content)?;

            database
        } else {
            Database::default()
        };

        let entries = database.get_entries();

        let ssh_items = entries
            .into_iter()
            .map(|e| {
                (
                    e.join(" "),
                    SshItem::Raw {
                        contents: e.clone(),
                    },
                )
            })
            .collect();

        Ok(SshItems { items: ssh_items })
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub async fn add_item(&self, raw_session: &[&str]) -> anyhow::Result<()> {
        tracing::debug!("adding item to database");
        let database_path = self.ensure_get_database_file_path().await?;

        let mut database = if database_path.exists() {
            let content = tokio::fs::read(&database_path)
                .await
                .context("failed to read nossh database file")?;

            let database: Database = serde_json::from_slice(&content)?;

            database
        } else {
            Database::default()
        };

        database.add_raw(raw_session);

        let mut database_file = tokio::fs::File::create(database_path)
            .await
            .context("failed to create nossh database file")?;

        let database_file_content = serde_json::to_vec_pretty(&database)?;

        database_file
            .write_all(&database_file_content)
            .await
            .context("failed to write data to database file")?;
        database_file
            .flush()
            .await
            .context("failed to flush nossh database file")?;

        Ok(())
    }

    fn get_database_file_path(&self) -> PathBuf {
        dirs::data_local_dir()
            .expect("requires having a data dir, if using nossh")
            .join("nossh")
            .join("nossh.database.json")
    }

    async fn ensure_get_database_file_path(&self) -> anyhow::Result<PathBuf> {
        let file_dir = self.get_database_file_path();

        if let Some(parent) = file_dir.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .context("failed to create data dir for nossh")?;
        }

        Ok(file_dir)
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
struct Database {
    entries: BTreeMap<String, DatabaseEntry>,
}
impl Database {
    fn add_raw(&mut self, raw_session: &[&str]) {
        self.entries.insert(
            raw_session.join(" "),
            DatabaseEntry::Raw {
                contents: raw_session.iter().map(|r| r.to_string()).collect(),
            },
        );
    }

    fn get_entries(&self) -> Vec<&Vec<String>> {
        let mut items = Vec::new();

        for v in self.entries.values() {
            match v {
                DatabaseEntry::Raw { contents } => {
                    items.push(contents);
                }
            }
        }

        items
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "entry_type")]
enum DatabaseEntry {
    Raw { contents: Vec<String> },
}

pub trait SshCommandDatabaseState {
    fn ssh_command_database(&self) -> SshCommandDatabase;
}

impl SshCommandDatabaseState for State {
    fn ssh_command_database(&self) -> SshCommandDatabase {
        SshCommandDatabase {}
    }
}
