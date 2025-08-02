use std::env;
use async_trait::async_trait;
use serenity::all::{Context, GatewayIntents, Ready};
use crate::ports::clients::Client;
use serenity::Client as SerenityClient;
use serenity::client::EventHandler;
use crate::domain::app::App;

pub struct DiscordClient(SerenityClient);


impl DiscordClient {
    pub async fn new(app: App) -> Self {
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
impl EventHandler for App {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        println!("Connected as {}", data_about_bot.user.name);
    }
}