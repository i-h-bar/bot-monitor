use crate::adapters::inbound::clients::discord::utils::messages;
use crate::ports::inbound::client::events::create::CreateEntry;
use crate::ports::inbound::client::events::create::CreateEntryEvent;
use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, GuildId,
    Permissions, ResolvedValue, User,
};

pub fn register() -> CreateCommand {
    CreateCommand::new("add")
        .description("Add a bot to a register")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "bot",
                "The bot you want to add to the register",
            )
            .required(true),
        )
        .default_member_permissions(Permissions::ADMINISTRATOR)
}

pub struct DiscordCreateEvent {
    ctx: Context,
    command: CommandInteraction,
    bot: User,
    guild_id: GuildId,
}

impl DiscordCreateEvent {
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

        let guild_id = command.guild_id?;

        Some(Self {
            ctx,
            command,
            bot: bot?,
            guild_id,
        })
    }

    fn user(&self) -> &User {
        &self.command.user
    }
}

#[async_trait]
impl CreateEntryEvent for DiscordCreateEvent {
    fn entry(&self) -> CreateEntry {
        CreateEntry {
            user_id: self.user().id.to_string(),
            bot_id: self.bot.id.to_string(),
            guild_id: self.guild_id.to_string(),
            version: 0,
        }
    }

    fn is_bot(&self) -> bool {
        self.bot.bot
    }

    async fn not_a_bot_message(&self) {
        messages::send_ephemeral(
            &self.ctx,
            &self.command,
            "This is to track bots not to spy on people",
        )
        .await;
    }

    async fn entry_added_message(&self) {
        let message = format!(
            "Added to {} to the register. I will now DM you when it goes offline and when it comes online.",
            self.bot.name
        );
        messages::send_ephemeral(&self.ctx, &self.command, &message).await;
    }

    async fn failed_message(&self) {
        messages::send_ephemeral(&self.ctx, &self.command, "Failed to add bot to register").await;
    }
}
