use std::env;
use async_trait::async_trait;
use serenity::all::{Command, Context, GatewayIntents, Interaction, Ready};
use crate::ports::clients::Client;
use serenity::Client as SerenityClient;
use serenity::client::EventHandler;
use crate::domain::app::App;
use crate::domain::register::Register;
use crate::ports::clients::discord::commands::add;

pub struct DiscordClient(SerenityClient);


impl DiscordClient {
    pub async fn new<R: Register + Send + Sync + 'static>(app: App<R>) -> Self {
        let token = env::var("BOT_TOKEN").expect("Bot token wasn't in env vars");
        let intents = GatewayIntents::DIRECT_MESSAGES;;

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
impl<R> EventHandler for App<R> where R: Register + Send + Sync {
    async fn ready(&self, ctx: Context, _: Ready) {
        if let Err(err) = Command::create_global_command(&ctx, add::register()).await {
            log::warn!("Could not create command {:?}", err);
        } else {
            log::info!("Created add command");
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
                    let options = command.data.options();
                    let x = 0;
                }
                _ => {}
            }
        }
    }
}