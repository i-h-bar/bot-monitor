use serenity::all::{Context, CreateMessage, MessageBuilder, OnlineStatus, Presence, PresenceUser, UserId};
use crate::domain::app::App;
use crate::domain::register::Register;

async fn is_bot(ctx: &Context, user: &PresenceUser) -> bool {
    if let Some(is_bot) =  user.bot {
        return is_bot;
    }

    if let Ok(user) = user.id.to_user(ctx).await {
        user.bot
    } else {
        false
    }
}

impl<R> App<R> where R: Register {
    pub async fn resolve_update(&self, ctx: Context, presence: Presence) {
        if !is_bot(&ctx, &presence.user).await {
            return;
        }

        match presence.status {
            OnlineStatus::Offline | OnlineStatus::Invisible  => self.resolve_offline(&ctx, presence).await,
            OnlineStatus::Online => self.resolve_online(&ctx, presence).await,
            _ => {}
        }


    }

    async fn resolve_online(&self, ctx: &Context, presence: Presence) {
        if let Some(entry) = self.fetch_from_register(presence.user.id.into()).await {
            log::info!("A bot came back online!");
            let user_id = UserId::new(entry.user_id);
            let bot_id = UserId::new(entry.bot_id);
            let bot_name = bot_id.to_user(&ctx.http).await.unwrap().name;

            let message = MessageBuilder::new()
                .mention(&user_id)
                .push(" hurray your bot '")
                .push(bot_name)
                .push("': ")
                .mention(&bot_id)
                .push(" cam back online!")
                .build();

            if let Err(why) = user_id.direct_message(&ctx, CreateMessage::new().content(message)).await {
                log::warn!("Could not send message to Discord: {}", why);
            }
        }
    }

    async fn resolve_offline(&self, ctx: &Context, presence: Presence) {
        if let Some(entry) = self.fetch_from_register(presence.user.id.into()).await {
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

            if let Err(why) = user_id.direct_message(&ctx, CreateMessage::new().content(message)).await {
                log::warn!("Could not send message to Discord: {}", why);
            }
        }
    }
}


