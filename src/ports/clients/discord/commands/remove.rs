use crate::domain::app::App;
use crate::domain::register::Register;
use crate::ports::clients::discord::utils::messages;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    ResolvedValue, User,
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
}

impl<R> App<R>
where
    R: Register,
{
    pub async fn remove_command(&self, ctx: &Context, command: CommandInteraction) {
        let options = command.data.options();
        let mut bot: Option<&User> = None;

        for option in options {
            if option.name == "bot"
                && let ResolvedValue::User(user, ..) = option.value
            {
                bot = Some(user);
            }
        }

        let Some(bot) = bot else {
            messages::send_ephemeral(ctx, &command, "Failed to remove bot from the register").await;
            return;
        };

        if self
            .remove_from_register(bot.id.to_string(), command.user.id.to_string())
            .await
            .is_err()
        {
            messages::send_ephemeral(ctx, &command, "Failed to remove bot from the register").await;
            return;
        }

        let message = format!(
            "I have removed {} from the register. I will no longer DM you when the bot goes offline or comes online",
            bot.name
        );
        messages::send_ephemeral(ctx, &command, &message).await;
    }
}
