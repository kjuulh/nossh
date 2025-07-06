use skim::prelude::*;

use crate::{
    ssh_config::{SshItem, SshItems},
    state::State,
};

pub struct UserFuzzyFind {}

impl UserFuzzyFind {
    pub async fn get_ssh_item_from_user<'a>(
        &self,
        items: &'a SshItems,
    ) -> anyhow::Result<&'a SshItem> {
        let skim_options = SkimOptionsBuilder::default()
            .no_multi(true)
            .build()
            .expect("failed to build skim config");

        let (tx, rx): (SkimItemSender, SkimItemReceiver) = skim::prelude::unbounded();

        for item in items.to_vec().into_iter().cloned() {
            tx.send(Arc::new(item))
                .expect("we should never have enough items that we exceed unbounded");
        }

        let chosen_items = Skim::run_with(&skim_options, Some(rx))
            .and_then(|output| if output.is_abort { None } else { Some(output) })
            .map(|item| item.selected_items)
            .ok_or(anyhow::anyhow!("failed to find an ssh item"))?;

        let chosen_item = chosen_items
            .first()
            .expect("there should never be more than 1 skip item");

        let output = chosen_item.output();

        let chosen_ssh_item = items
            .get_choice(&output) // Cow, str, String
            .expect("always find an ssh item from a choice");
        tracing::debug!("the user chose item: {chosen_ssh_item:#?}");

        Ok(chosen_ssh_item)
    }
}

pub trait UserFuzzyFindState {
    fn user_fuzzy_find(&self) -> UserFuzzyFind;
}

impl UserFuzzyFindState for State {
    fn user_fuzzy_find(&self) -> UserFuzzyFind {
        UserFuzzyFind {}
    }
}

impl SkimItem for SshItem {
    fn text(&'_ self) -> std::borrow::Cow<'_, str> {
        format!("{self}").into()
    }
}
