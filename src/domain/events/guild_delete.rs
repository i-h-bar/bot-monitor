use crate::domain::app::App;
use crate::ports::inbound::client::events::guild_delete::GuildDeleteEvent;
use crate::ports::inbound::client::events::guild_resolution::GuildResolution;
use crate::ports::inbound::client::events::remove::RemoveEntry;
use crate::ports::outbound::register::Register;

impl<R> App<R>
where
    R: Register,
{
    pub async fn cleanup_guild<E: GuildDeleteEvent>(&self, event: E) {
        if !event.is_real_removal() {
            return;
        }

        let guild_id = event.guild_id();
        let entries = match self.register.fetch_by_guild(guild_id.clone()).await {
            Ok(entries) => entries,
            Err(why) => {
                log::warn!("Failed to fetch entries for departed guild {guild_id}: {why:?}");
                return;
            }
        };

        log::info!(
            "Bot Monitor left guild {guild_id}, re-checking {} register entries",
            entries.len()
        );

        for entry in entries {
            let bot_id = entry.bot_id.clone();
            let user_id = entry.user_id.clone();

            match event
                .resolve_shared_guild(bot_id.clone(), user_id.clone())
                .await
            {
                GuildResolution::Found(new_guild_id) => {
                    if let Err(why) = self
                        .register
                        .update_guild_id(bot_id.clone(), user_id.clone(), new_guild_id.clone())
                        .await
                    {
                        log::warn!(
                            "Failed to update guild_id after guild_delete for bot_id={bot_id} user_id={user_id}: {why:?}"
                        );
                    } else {
                        log::info!(
                            "Re-associated register entry bot_id={bot_id} user_id={user_id} with guild_id={new_guild_id} after leaving guild_id={guild_id}"
                        );
                    }
                }
                GuildResolution::NotFound => {
                    let remove_entry = RemoveEntry {
                        bot_id: bot_id.clone(),
                        user_id: user_id.clone(),
                    };
                    if let Err(why) = self.register.remove(remove_entry).await {
                        log::warn!(
                            "Failed to remove orphaned entry after guild_delete for bot_id={bot_id} user_id={user_id}: {why:?}"
                        );
                    } else {
                        log::info!(
                            "Removed orphaned register entry bot_id={bot_id} user_id={user_id} after leaving guild_id={guild_id} (no shared guild remains)"
                        );
                    }
                }
                GuildResolution::Unknown => {
                    log::warn!(
                        "Could not resolve guild after guild_delete for bot_id={bot_id} user_id={user_id}, skipping entry"
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::inbound::client::events::guild_delete::MockGuildDeleteEvent;
    use crate::ports::outbound::register::{MockRegister, RegisterEntry};
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_cleanup_guild_skips_unavailable_outage() {
        let mut register = MockRegister::new();
        register
            .expect_fetch_by_guild()
            .times(0)
            .return_const(Ok(Vec::new()));

        let mut event = MockGuildDeleteEvent::new();
        event.expect_is_real_removal().times(1).return_const(false);
        event.expect_guild_id().times(0).return_const(String::new());

        let app = App::new(register);
        app.cleanup_guild(event).await;
    }

    #[tokio::test]
    async fn test_cleanup_guild_found_updates_guild_id() {
        let bot_id = String::from("bot_id_12345");
        let user_id = String::from("user_id_12345");
        let old_guild_id = String::from("guild_id_old");
        let new_guild_id = String::from("guild_id_new");
        let entry = RegisterEntry {
            bot_id: bot_id.clone(),
            user_id: user_id.clone(),
            guild_id: Some(old_guild_id.clone()),
        };

        let mut register = MockRegister::new();
        register
            .expect_fetch_by_guild()
            .times(1)
            .with(eq(old_guild_id.clone()))
            .return_const(Ok(vec![entry]));
        register
            .expect_update_guild_id()
            .times(1)
            .with(
                eq(bot_id.clone()),
                eq(user_id.clone()),
                eq(new_guild_id.clone()),
            )
            .return_const(Ok(()));
        register.expect_remove().times(0).return_const(Ok(()));

        let mut event = MockGuildDeleteEvent::new();
        event.expect_is_real_removal().times(1).return_const(true);
        event
            .expect_guild_id()
            .times(1)
            .return_const(old_guild_id.clone());
        event
            .expect_resolve_shared_guild()
            .times(1)
            .with(eq(bot_id.clone()), eq(user_id.clone()))
            .return_const(GuildResolution::Found(new_guild_id.clone()));

        let app = App::new(register);
        app.cleanup_guild(event).await;
    }

    #[tokio::test]
    async fn test_cleanup_guild_not_found_removes_entry() {
        let bot_id = String::from("bot_id_12345");
        let user_id = String::from("user_id_12345");
        let guild_id = String::from("guild_id_old");
        let entry = RegisterEntry {
            bot_id: bot_id.clone(),
            user_id: user_id.clone(),
            guild_id: Some(guild_id.clone()),
        };

        let mut register = MockRegister::new();
        register
            .expect_fetch_by_guild()
            .times(1)
            .with(eq(guild_id.clone()))
            .return_const(Ok(vec![entry]));
        register
            .expect_update_guild_id()
            .times(0)
            .return_const(Ok(()));
        register
            .expect_remove()
            .times(1)
            .with(eq(RemoveEntry {
                bot_id: bot_id.clone(),
                user_id: user_id.clone(),
            }))
            .return_const(Ok(()));

        let mut event = MockGuildDeleteEvent::new();
        event.expect_is_real_removal().times(1).return_const(true);
        event
            .expect_guild_id()
            .times(1)
            .return_const(guild_id.clone());
        event
            .expect_resolve_shared_guild()
            .times(1)
            .with(eq(bot_id.clone()), eq(user_id.clone()))
            .return_const(GuildResolution::NotFound);

        let app = App::new(register);
        app.cleanup_guild(event).await;
    }
}
