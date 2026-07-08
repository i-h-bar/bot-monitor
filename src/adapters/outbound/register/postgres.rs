use crate::ports::inbound::client::events::create::CreateEntry;
use crate::ports::inbound::client::events::list::ListEntriesPayload;
use crate::ports::inbound::client::events::remove::RemoveEntry;
use crate::ports::outbound::register::{Register, RegisterEntry, RegisterError};
use async_trait::async_trait;
use sqlx::PgPool;
use std::env;

pub struct Postgres(PgPool);

impl Postgres {
    pub async fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to Postgres");
        Self(pool)
    }
}

#[async_trait]
impl Register for Postgres {
    async fn fetch(&self, bot_id: String) -> Option<Vec<RegisterEntry>> {
        let rows: Vec<(String, String, Option<String>)> = match sqlx::query_as(
            "SELECT bot_id, user_id, guild_id FROM register WHERE bot_id = $1",
        )
        .bind(&bot_id)
        .fetch_all(&self.0)
        .await
        {
            Ok(r) => r,
            Err(e) => {
                log::error!("{e:?}");
                return None;
            }
        };

        Some(
            rows.into_iter()
                .map(|(_, user_id, guild_id)| RegisterEntry {
                    bot_id: bot_id.clone(),
                    user_id,
                    guild_id,
                })
                .collect(),
        )
    }

    async fn add(&self, entry: CreateEntry) -> Result<(), RegisterError> {
        sqlx::query(
            "INSERT INTO register (bot_id, user_id, entry_version, guild_id) VALUES ($1, $2, $3, $4)",
        )
        .bind(&entry.bot_id)
        .bind(&entry.user_id)
        .bind(entry.version.to_string())
        .bind(&entry.guild_id)
        .execute(&self.0)
        .await
        .map_err(|e| {
            log::error!("failed to add entry: {e:?}");
            RegisterError::EntryCreationError
        })?;
        Ok(())
    }

    async fn remove(&self, entry: RemoveEntry) -> Result<(), RegisterError> {
        sqlx::query("DELETE FROM register WHERE bot_id = $1 AND user_id = $2")
            .bind(&entry.bot_id)
            .bind(&entry.user_id)
            .execute(&self.0)
            .await
            .map_err(|e| {
                log::error!("failed to remove entry: {e:?}");
                RegisterError::EntryRemoveError
            })?;
        Ok(())
    }

    async fn list(&self, entry: ListEntriesPayload) -> Result<Vec<RegisterEntry>, RegisterError> {
        let rows: Vec<(String, String, Option<String>)> =
            sqlx::query_as("SELECT bot_id, user_id, guild_id FROM register WHERE user_id = $1")
                .bind(&entry.user_id)
                .fetch_all(&self.0)
                .await
                .map_err(|e| {
                    log::error!("{e:?}");
                    RegisterError::EntryFetchError
                })?;

        Ok(rows
            .into_iter()
            .map(|(bot_id, _, guild_id)| RegisterEntry {
                bot_id,
                user_id: entry.user_id.clone(),
                guild_id,
            })
            .collect())
    }

    async fn fetch_by_guild(&self, guild_id: String) -> Result<Vec<RegisterEntry>, RegisterError> {
        let rows: Vec<(String, String)> =
            sqlx::query_as("SELECT bot_id, user_id FROM register WHERE guild_id = $1")
                .bind(&guild_id)
                .fetch_all(&self.0)
                .await
                .map_err(|e| {
                    log::error!("{e:?}");
                    RegisterError::EntryFetchError
                })?;

        Ok(rows
            .into_iter()
            .map(|(bot_id, user_id)| RegisterEntry {
                bot_id,
                user_id,
                guild_id: Some(guild_id.clone()),
            })
            .collect())
    }

    async fn fetch_missing_guild_id(&self) -> Result<Vec<RegisterEntry>, RegisterError> {
        let rows: Vec<(String, String)> =
            sqlx::query_as("SELECT bot_id, user_id FROM register WHERE guild_id IS NULL")
                .fetch_all(&self.0)
                .await
                .map_err(|e| {
                    log::error!("{e:?}");
                    RegisterError::EntryFetchError
                })?;

        Ok(rows
            .into_iter()
            .map(|(bot_id, user_id)| RegisterEntry {
                bot_id,
                user_id,
                guild_id: None,
            })
            .collect())
    }

    async fn update_guild_id(
        &self,
        bot_id: String,
        user_id: String,
        guild_id: String,
    ) -> Result<(), RegisterError> {
        sqlx::query("UPDATE register SET guild_id = $1 WHERE bot_id = $2 AND user_id = $3")
            .bind(&guild_id)
            .bind(&bot_id)
            .bind(&user_id)
            .execute(&self.0)
            .await
            .map_err(|e| {
                log::error!("failed to update guild_id: {e:?}");
                RegisterError::EntryUpdateError
            })?;
        Ok(())
    }
}
