use crate::ports::inbound::client::events::guild_resolution::GuildResolution;
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait GuildDeleteEvent {
    fn guild_id(&self) -> String;
    fn is_real_removal(&self) -> bool;
    async fn resolve_shared_guild(&self, bot_id: String, user_id: String) -> GuildResolution;
}
