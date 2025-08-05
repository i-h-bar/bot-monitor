use crate::domain::register::RegisterEntry;
use crate::domain::events::status::{BotStates, StatusEvent};
use crate::ports::clients::discord::utils::user::user_from_id;
use async_trait::async_trait;
use serenity::all::{
    Context, CreateMessage, MessageBuilder, OnlineStatus, Presence, PresenceUser, UserId,
};
use std::str::FromStr;

pub struct DiscordStatusEvent {
    bot: PresenceUser,
    status: OnlineStatus,
    ctx: Context,
}

impl DiscordStatusEvent {
    pub fn new(ctx: Context, event: Presence) -> Self {
        Self {
            bot: event.user,
            status: event.status,
            ctx,
        }
    }
}

#[async_trait]
impl StatusEvent for DiscordStatusEvent {
    fn bot_id(&self) -> String {
        self.bot.id.to_string()
    }

    fn state(&self) -> BotStates {
        match self.status {
            OnlineStatus::Online => BotStates::Online,
            OnlineStatus::Offline | OnlineStatus::Invisible => BotStates::Offline,
            _ => BotStates::NA,
        }
    }

    async fn is_bot(&self) -> bool {
        if let Some(is_bot) = self.bot.bot {
            return is_bot;
        }

        if let Ok(user) = self.bot.id.to_user(&self.ctx).await {
            user.bot
        } else {
            false
        }
    }

    async fn send_offline_warning(&self, entries: Vec<RegisterEntry>) {
        for entry in entries {
            let user_id = UserId::new(u64::from_str(&entry.user_id).unwrap());
            let bot_id = UserId::new(u64::from_str(&entry.bot_id).unwrap());
            let bot_name = if let Some(bot) = user_from_id(&self.ctx, self.bot.id.into()).await {
                bot.name
            } else {
                String::from("Placeholder Name")
            };

            let message = MessageBuilder::new()
                .push("Hello, ")
                .mention(&user_id)
                .push(format!(" Your bot named '{bot_name}': "))
                .mention(&bot_id)
                .push(" has gone offline!")
                .build();

            if let Err(why) = user_id
                .direct_message(&self.ctx, CreateMessage::new().content(message))
                .await
            {
                log::warn!("Could not send message to Discord: {why}");
            }
        }
    }

    async fn send_online_message(&self, entries: Vec<RegisterEntry>) {
        for entry in entries {
            let user_id = UserId::new(u64::from_str(&entry.user_id).unwrap());
            let bot_id = UserId::new(u64::from_str(&entry.bot_id).unwrap());
            let bot_name = if let Some(bot) = user_from_id(&self.ctx, bot_id.into()).await {
                bot.name
            } else {
                String::from("Placeholder Name")
            };

            let message = MessageBuilder::new()
                .push("Hurray! ")
                .mention(&user_id)
                .push(", your bot '")
                .push(bot_name)
                .push("': ")
                .mention(&bot_id)
                .push(" is back online!")
                .build();

            if let Err(why) = user_id
                .direct_message(&self.ctx, CreateMessage::new().content(message))
                .await
            {
                log::warn!("Could not send message to Discord: {why}");
            }
        }
    }
}
