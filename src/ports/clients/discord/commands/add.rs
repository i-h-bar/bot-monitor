use crate::domain::app::App;
use crate::domain::register::{Register, RegisterEntry};
use crate::ports::clients::discord::utils::messages;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    ResolvedValue, User,
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
}

impl<R> App<R>
where
    R: Register,
{
    pub async fn add_command(&self, ctx: &Context, command: CommandInteraction) {
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
            messages::send_ephemeral(ctx, &command, "Failed to add bot to register").await;
            return;
        };

        // if !bot.bot {
        //     messages::send_ephemeral(&ctx, &command, "This is to track bots not to spy on people")
        //         .await;
        // }

        let entry = RegisterEntry {
            bot_id: bot.id.into(),
            user_id: command.user.id.into(),
        };

        if self.add_to_register(entry).await.is_err() {
            messages::send_ephemeral(ctx, &command, "Failed to add bot to register").await;
        }

        let message = format!(
            "Added to {} to the register. I will now DM you when it goes offline and when it comes online.",
            bot.name
        );
        messages::send_ephemeral(ctx, &command, &message).await;
    }
}
