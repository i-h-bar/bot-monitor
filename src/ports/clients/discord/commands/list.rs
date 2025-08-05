use crate::domain::register::RegisterEntry;
use crate::ports::clients::discord::utils::messages;
use crate::ports::clients::discord::utils::user::user_from_id;
use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateCommand, MessageBuilder, User};
use serenity::futures::future::join_all;
use std::str::FromStr;
use crate::domain::events::list::{ListEntriesPayload, ListEvent};

pub fn register() -> CreateCommand {
    CreateCommand::new("list").description("List all the warnings you currently have active")
}

pub struct DiscordListEvent {
    ctx: Context,
    command: CommandInteraction,
}

impl DiscordListEvent {
    pub fn new(ctx: Context, command: CommandInteraction) -> Self {
        DiscordListEvent { ctx, command }
    }

    fn user(&self) -> &User {
        &self.command.user
    }
}

#[async_trait]
impl ListEvent for DiscordListEvent {
    fn payload(&self) -> ListEntriesPayload {
        ListEntriesPayload {
            user_id: self.user().id.to_string(),
        }
    }

    async fn failed_message(&self) {
        messages::send_ephemeral(&self.ctx, &self.command, "Failed to list your entries :(").await;
    }

    async fn success_message(&self, entries: Vec<RegisterEntry>) {
        let mut message_builder = MessageBuilder::new();
        message_builder.push("Current Warnings:");

        let bots =
            join_all(entries.into_iter().filter_map(|entry| {
                Some(user_from_id(&self.ctx, u64::from_str(&entry.bot_id).ok()?))
            }))
            .await;

        for bot in bots {
            let Some(bot) = bot else {
                continue;
            };
            message_builder
                .push("\n")
                .push(bot.name)
                .push(": ")
                .mention(&bot.id);
        }

        let message = message_builder.build();
        messages::send_ephemeral(&self.ctx, &self.command, &message).await;
    }

    async fn empty_message(&self) {
        messages::send_ephemeral(
            &self.ctx,
            &self.command,
            "You have no current warnings set up",
        )
        .await;
    }
}
