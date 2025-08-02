use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};

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
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "warnee",
                "The user you would like register bot to message if the bot is not responsive"
            )
                .required(true),
        )
}

