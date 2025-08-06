use crate::domain::events::remove::{RemoveEntry, RemoveEvent};
use crate::ports::clients::discord::utils::messages;
use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    Permissions, ResolvedValue, User,
};

pub fn register() -> CreateCommand {
    CreateCommand::new("remove")
        .description("Remove a bot from the register")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "bot",
                "The bot you want to remove from the register",
            )
            .required(true),
        )
        .default_member_permissions(Permissions::ADMINISTRATOR)
}

pub struct RemoveDiscordEvent {
    ctx: Context,
    command: CommandInteraction,
    bot: User,
}

impl RemoveDiscordEvent {
    pub fn new(ctx: Context, command: CommandInteraction) -> Option<Self> {
        let options = command.data.options();
        let mut bot: Option<User> = None;

        for option in options {
            if option.name == "bot"
                && let ResolvedValue::User(user, ..) = option.value
            {
                bot = Some(user.clone());
            }
        }

        Some(Self {
            ctx,
            command,
            bot: bot?,
        })
    }

    fn user(&self) -> &User {
        &self.command.user
    }
}

#[async_trait]
impl RemoveEvent for RemoveDiscordEvent {
    fn entry(&self) -> RemoveEntry {
        RemoveEntry {
            user_id: self.user().id.to_string(),
            bot_id: self.bot.id.to_string(),
        }
    }

    async fn failed_message(&self) {
        messages::send_ephemeral(
            &self.ctx,
            &self.command,
            "Failed to remove bot from the register",
        )
        .await;
    }

    async fn success_message(&self) {
        let message = format!(
            "I have removed {} from the register. I will no longer DM you when the bot goes offline or comes online",
            self.bot.name
        );
        messages::send_ephemeral(&self.ctx, &self.command, &message).await;
    }
}
