use crate::domain::app::App;
use crate::domain::register::{Register, RegisterError};
use crate::ports::clients::discord::utils::messages;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedValue, User,
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
            match option.name {
                "bot" => match option.value {
                    ResolvedValue::User(user, ..) => bot = Some(user),
                    _ => {}
                },
                _ => {}
            }
        }

        let Some(bot) = bot else {
            messages::send_ephemeral(&ctx, &command, "Failed to remove bot from the register")
                .await;
            return;
        };

        if let Err(_) = self
            .remove_from_register(bot.id.into(), command.user.id.into())
            .await
        {
            messages::send_ephemeral(&ctx, &command, "Failed to remove bot from the register")
                .await;
            return;
        };

        let message = format!(
            "I have removed {} from the register. I will no longer DM you when the bot goes offline or comes online",
            bot.name
        );
        messages::send_ephemeral(&ctx, &command, &message).await;
    }
}
