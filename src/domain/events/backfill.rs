use crate::domain::app::App;
use crate::ports::inbound::client::events::backfill::BackfillEvent;
use crate::ports::inbound::client::events::guild_resolution::GuildResolution;
use crate::ports::inbound::client::events::remove::RemoveEntry;
use crate::ports::outbound::register::Register;

impl<R> App<R>
where
    R: Register,
{
    pub async fn backfill_guild_ids<E: BackfillEvent>(&self, event: E) {
        let entries = match self.register.fetch_missing_guild_id().await {
            Ok(entries) => entries,
            Err(why) => {
                log::warn!("Failed to fetch entries missing guild_id: {why:?}");
                return;
            }
        };

        log::info!(
            "Backfilling guild_id for {} legacy register entries",
            entries.len()
        );

        for entry in entries {
            let bot_id = entry.bot_id.clone();
            let user_id = entry.user_id.clone();

            match event
                .resolve_shared_guild(bot_id.clone(), user_id.clone())
                .await
            {
                GuildResolution::Found(guild_id) => {
                    if let Err(why) = self
                        .register
                        .update_guild_id(bot_id.clone(), user_id.clone(), guild_id.clone())
                        .await
                    {
                        log::warn!(
                            "Failed to backfill guild_id for bot_id={bot_id} user_id={user_id}: {why:?}"
                        );
                    } else {
                        log::info!(
                            "Backfilled guild_id={guild_id} for bot_id={bot_id} user_id={user_id}"
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
                            "Failed to remove orphaned entry during backfill for bot_id={bot_id} user_id={user_id}: {why:?}"
                        );
                    } else {
                        log::info!(
                            "Removed orphaned register entry bot_id={bot_id} user_id={user_id} during backfill (no shared guild found)"
                        );
                    }
                }
                GuildResolution::Unknown => {
                    log::warn!(
                        "Could not resolve guild during backfill for bot_id={bot_id} user_id={user_id}, skipping entry"
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::inbound::client::events::backfill::MockBackfillEvent;
    use crate::ports::outbound::register::{MockRegister, RegisterEntry};
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_backfill_found_updates_guild_id() {
        let bot_id = String::from("bot_id_12345");
        let user_id = String::from("user_id_12345");
        let entry = RegisterEntry {
            bot_id: bot_id.clone(),
            user_id: user_id.clone(),
            guild_id: None,
        };

        let mut register = MockRegister::new();
        register
            .expect_fetch_missing_guild_id()
            .times(1)
            .return_const(Ok(vec![entry]));
        register
            .expect_update_guild_id()
            .times(1)
            .with(
                eq(bot_id.clone()),
                eq(user_id.clone()),
                eq(String::from("guild_id_1")),
            )
            .return_const(Ok(()));
        register.expect_remove().times(0).return_const(Ok(()));

        let mut event = MockBackfillEvent::new();
        event
            .expect_resolve_shared_guild()
            .times(1)
            .with(eq(bot_id.clone()), eq(user_id.clone()))
            .return_const(GuildResolution::Found(String::from("guild_id_1")));

        let app = App::new(register);
        app.backfill_guild_ids(event).await;
    }

    #[tokio::test]
    async fn test_backfill_not_found_removes_entry() {
        let bot_id = String::from("bot_id_12345");
        let user_id = String::from("user_id_12345");
        let entry = RegisterEntry {
            bot_id: bot_id.clone(),
            user_id: user_id.clone(),
            guild_id: None,
        };

        let mut register = MockRegister::new();
        register
            .expect_fetch_missing_guild_id()
            .times(1)
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

        let mut event = MockBackfillEvent::new();
        event
            .expect_resolve_shared_guild()
            .times(1)
            .with(eq(bot_id.clone()), eq(user_id.clone()))
            .return_const(GuildResolution::NotFound);

        let app = App::new(register);
        app.backfill_guild_ids(event).await;
    }

    #[tokio::test]
    async fn test_backfill_unknown_leaves_entry_untouched() {
        let bot_id = String::from("bot_id_12345");
        let user_id = String::from("user_id_12345");
        let entry = RegisterEntry {
            bot_id: bot_id.clone(),
            user_id: user_id.clone(),
            guild_id: None,
        };

        let mut register = MockRegister::new();
        register
            .expect_fetch_missing_guild_id()
            .times(1)
            .return_const(Ok(vec![entry]));
        register
            .expect_update_guild_id()
            .times(0)
            .return_const(Ok(()));
        register.expect_remove().times(0).return_const(Ok(()));

        let mut event = MockBackfillEvent::new();
        event
            .expect_resolve_shared_guild()
            .times(1)
            .with(eq(bot_id.clone()), eq(user_id.clone()))
            .return_const(GuildResolution::Unknown);

        let app = App::new(register);
        app.backfill_guild_ids(event).await;
    }
}
