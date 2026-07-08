use crate::ports::inbound::client::events::guild_resolution::GuildResolution;
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait BackfillEvent {
    async fn resolve_shared_guild(&self, bot_id: String, user_id: String) -> GuildResolution;
}
