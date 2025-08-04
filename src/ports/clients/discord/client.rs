use crate::domain::app::App;
use crate::domain::register::Register;
use crate::ports::clients::Client;
use crate::ports::clients::discord::commands::{add, list, remove};
use crate::ports::clients::discord::event::DiscordStatusEvent;
use async_trait::async_trait;
use serenity::Client as SerenityClient;
use serenity::all::{Command, Context, GatewayIntents, Interaction, Presence, Ready};
use serenity::client::EventHandler;
use std::env;

pub struct DiscordClient(SerenityClient);

impl DiscordClient {
    #[allow(clippy::missing_panics_doc)]
    pub async fn new<R: Register + Send + Sync + 'static>(app: App<R>) -> Self {
        let token = env::var("BOT_TOKEN").expect("Bot token wasn't in env vars");
        let intents = GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::GUILD_PRESENCES
            | GatewayIntents::DIRECT_MESSAGES;

        let client = SerenityClient::builder(&token, intents)
            .event_handler(app)
            .await
            .expect("Error creating client");

        Self(client)
    }
}

#[async_trait]
impl Client for DiscordClient {
    async fn run(&mut self) {
        if let Err(why) = self.0.start().await {
            println!("Failed to start DiscordClient - {why:?}");
        }
    }
}

#[async_trait]
impl<R> EventHandler for App<R>
where
    R: Register + Send + Sync,
{
    async fn presence_update(&self, ctx: Context, presence: Presence) {
        let event = DiscordStatusEvent::new(ctx, presence);
        self.resolve_event(event).await;
    }
    async fn ready(&self, ctx: Context, _: Ready) {
        if let Err(err) = Command::create_global_command(&ctx, add::register()).await {
            log::warn!("Could not create command {err:?}");
        } else {
            log::info!("Created add command");
        }

        if let Err(err) = Command::create_global_command(&ctx, remove::register()).await {
            log::warn!("Could not create command {err:?}");
        } else {
            log::info!("Created remove command");
        }

        if let Err(err) = Command::create_global_command(&ctx, list::register()).await {
            log::warn!("Could not create command {err:?}");
        } else {
            log::info!("Created list command");
        }

        log::info!("Bot is ready");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            if command.user.bot {
                return;
            }

            match command.data.name.as_str() {
                "add" => {
                    self.add_command(&ctx, command).await;
                }
                "remove" => self.remove_command(&ctx, command).await,
                "list" => self.list_command(&ctx, command).await,
                _ => {}
            }
        }
    }
}
