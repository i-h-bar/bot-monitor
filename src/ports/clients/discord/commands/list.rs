use crate::domain::app::App;
use crate::domain::register::Register;
use crate::ports::clients::discord::utils::messages;
use crate::ports::clients::discord::utils::user::user_from_id;
use serenity::all::{CommandInteraction, Context, CreateCommand, MessageBuilder};
use std::str::FromStr;

pub fn register() -> CreateCommand {
    CreateCommand::new("list").description("List all the warnings you currently have active")
}

impl<R> App<R>
where
    R: Register,
{
    pub async fn list_command(&self, ctx: &Context, command: CommandInteraction) {
        let Ok(entries) = self.list_user_entries(command.user.id.to_string()).await else {
            messages::send_ephemeral(ctx, &command, "Failed to list your entries :(").await;
            return;
        };

        if entries.is_empty() {
            messages::send_ephemeral(ctx, &command, "You have no current warnings set up").await;
            return;
        }

        let mut message_builder = MessageBuilder::new();
        message_builder.push("Current Warnings:");
        for entry in entries {
            let Some(bot) = user_from_id(ctx, u64::from_str(&entry.bot_id).unwrap()).await else {
                continue;
            };
            message_builder
                .push("\n")
                .push(bot.name)
                .push(": ")
                .mention(&bot.id);
        }

        let message = message_builder.build();
        messages::send_ephemeral(ctx, &command, &message).await;
    }
}
