use serde::{Deserialize, Serialize};
use ruma_macros::EventContent;

#[derive(Clone, Debug, Default, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
#[ruma_event(type = "dev.jorgemartin.recent_emojis", kind = GlobalAccountData)]
pub struct RecentEmojisEventContent {
    pub emojis: Vec<String>,
}

impl RecentEmojisEventContent {
    pub fn new(emojis: Vec<String>) -> Self {
        Self { emojis }
    }
}