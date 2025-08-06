use crate::domain::events::help::HelpEvent;
use crate::ports::clients::discord::utils::messages;
use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateCommand};

const HELP_MESSAGE: &str =
    "
 ```ansi
🕵️ \u{001b}[1;10;4;31mI am Monitor Bot\u{001b}[0m

I am a Discord bot that monitors other bots and alerts you when they go offline. Perfect for server admins who rely on multiple bots and want to ensure maximum uptime.

I will send you a Direct Message when a monitored bot goes offline.

To function properly, I use the following privileged intents:
- `GUILD_PRESENCES`
- `DIRECT_MESSAGES`

\u{001b}[1;10;4;31mAll Commands:\u{001b}[0m
\u{001b}[1;34m/add\u{001b}[0m - Add the specified bot to a register for monitoring. (Requires you to be an administrator)
\u{001b}[1;34m/remove\u{001b}[0m - Removes the specified bot from the register. (Requires you to be an administrator)
\u{001b}[1;34m/list\u{001b}[0m - Lists all current warnings you have registered (Requires you to be an administrator)
\u{001b}[1;34m/help\u{001b}[0m - Show this message.


\u{001b}[1;10;4;31mHaving issues with me or have suggestions for how I can improve?\u{001b}[0m
Please raise a ticket here https://github.com/i-h-bar/bot-monitor/issues
```
";

pub fn register() -> CreateCommand {
    CreateCommand::new("help").description("Shows the help message")
}

pub struct DiscordHelpEvent {
    ctx: Context,
    command: CommandInteraction,
}

impl DiscordHelpEvent {
    pub fn new(ctx: Context, command: CommandInteraction) -> Self {
        DiscordHelpEvent { ctx, command }
    }
}

#[async_trait]
impl HelpEvent for DiscordHelpEvent {
    async fn send_message(&self) {
        messages::send_ephemeral(&self.ctx, &self.command, HELP_MESSAGE).await;
    }
}
