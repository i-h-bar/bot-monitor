use crate::domain::app::App;
use crate::domain::register::{Register, RegisterEntry};
use crate::ports::clients::discord::commands::add;
use crate::ports::clients::Client;
use async_trait::async_trait;
use serenity::all::{Command, Context, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, GatewayIntents, GuildMemberUpdateEvent, Interaction, Member, MessageBuilder, OnlineStatus, Presence, Ready, ResolvedValue, User, UserId};
use serenity::client::EventHandler;
use serenity::Client as SerenityClient;
use std::env;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct DiscordClient(SerenityClient);

impl DiscordClient {
    pub async fn new<R: Register + Send + Sync + 'static>(app: App<R>) -> Self {
        let token = env::var("BOT_TOKEN").expect("Bot token wasn't in env vars");
        let intents = GatewayIntents::DIRECT_MESSAGES | GatewayIntents::GUILD_PRESENCES | GatewayIntents::DIRECT_MESSAGES;

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
                    let mut bot: Option<&User> = None;
                    let mut warnee: Option<&User> = None;

                    for option in options {
                        match option.name {
                            "bot" => match option.value {
                                ResolvedValue::User(user, ..) => bot = Some(user),
                                _ => {}
                            },
                            "warnee" => match option.value {
                                ResolvedValue::User(user, ..) => warnee = Some(user),
                                _ => {}
                            }
                            _ => {}
                        }
                    }

                    let Some(bot) = bot else {return};
                    let Some(warnee) = warnee else {return};

                    if !bot.bot {
                        let response = CreateInteractionResponseMessage::new().content("This is to track bots not to spy on people").ephemeral(true);
                        command.create_response(ctx, CreateInteractionResponse::Message(response)).await.unwrap();
                    }

                    let entry = RegisterEntry {
                        bot_id: bot.id.into(),
                        user_id: warnee.id.into()
                    };

                    self.add_to_register(entry).await;
                }
                _ => {}
            }
        }
    }

    async fn presence_update(&self, ctx: Context, presence: Presence) {
        log::info!("Presence update for {} status - {:?}", presence.user.id, presence.status);
        if let Some(name) = presence.user.name {
            log::info!("Presence update for {}", name);
        }

        if let Some(is_bot) = presence.user.bot && !is_bot {
            return;
        }

        if presence.status == OnlineStatus::Offline && let Some(entry) = self.fetch_from_register(presence.user.id.into()).await {
            log::info!("A bot went offline!");
            let user_id = UserId::new(entry.user_id);
            let bot_id = UserId::new(entry.bot_id);
            let bot_name = bot_id.to_user(&ctx.http).await.unwrap().name;

            let message = MessageBuilder::new()
                .push("Hello, ")
                .mention(&user_id)
                .push(format!(" Your bot named '{bot_name}': "))
                .mention(&bot_id)
                .push(" has gone offline!")
                .build();

            user_id.direct_message(&ctx, CreateMessage::new().content(message)).await.unwrap();
        }
    }
}
