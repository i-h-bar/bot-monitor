use crate::adapters::inbound::clients::discord::utils::user::user_from_id;
use crate::ports::inbound::client::events::backfill::BackfillEvent;
use crate::ports::inbound::client::events::guild_delete::GuildDeleteEvent;
use crate::ports::inbound::client::events::guild_resolution::GuildResolution;
use crate::ports::inbound::client::events::status::{BotStates, StatusEvent};
use crate::ports::outbound::register::RegisterEntry;
use async_trait::async_trait;
use serenity::Error as SerenityError;
use serenity::all::{
    Context, CreateMessage, GuildId, MessageBuilder, OnlineStatus, Presence, PresenceUser,
    UnavailableGuild, UserId,
};
use serenity::http::StatusCode;
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

fn is_not_found(err: &SerenityError) -> bool {
    matches!(
        err,
        SerenityError::Http(http_err) if http_err.status_code() == Some(StatusCode::NOT_FOUND)
    )
}

async fn resolve_shared_guild_for(
    ctx: &Context,
    bot_id: String,
    user_id: String,
) -> GuildResolution {
    let (Ok(bot_id), Ok(user_id)) = (u64::from_str(&bot_id), u64::from_str(&user_id)) else {
        return GuildResolution::Unknown;
    };
    let bot_id = UserId::new(bot_id);
    let user_id = UserId::new(user_id);

    for guild_id in ctx.cache.guilds() {
        match (
            guild_id.member(ctx, user_id).await,
            guild_id.member(ctx, bot_id).await,
        ) {
            (Ok(_), Ok(_)) => return GuildResolution::Found(guild_id.to_string()),
            (Err(e), _) | (_, Err(e)) if is_not_found(&e) => {}
            _ => return GuildResolution::Unknown,
        }
    }

    GuildResolution::NotFound
}

pub struct DiscordBackfillEvent {
    ctx: Context,
}

impl DiscordBackfillEvent {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }
}

#[async_trait]
impl BackfillEvent for DiscordBackfillEvent {
    async fn resolve_shared_guild(&self, bot_id: String, user_id: String) -> GuildResolution {
        resolve_shared_guild_for(&self.ctx, bot_id, user_id).await
    }
}

pub struct DiscordGuildDeleteEvent {
    ctx: Context,
    guild_id: GuildId,
    unavailable: bool,
}

impl DiscordGuildDeleteEvent {
    pub fn new(ctx: Context, incomplete: UnavailableGuild) -> Self {
        Self {
            ctx,
            guild_id: incomplete.id,
            unavailable: incomplete.unavailable,
        }
    }
}

#[async_trait]
impl GuildDeleteEvent for DiscordGuildDeleteEvent {
    fn guild_id(&self) -> String {
        self.guild_id.to_string()
    }

    fn is_real_removal(&self) -> bool {
        !self.unavailable
    }

    async fn resolve_shared_guild(&self, bot_id: String, user_id: String) -> GuildResolution {
        resolve_shared_guild_for(&self.ctx, bot_id, user_id).await
    }
}
