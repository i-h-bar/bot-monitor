use serenity::all::{Context, User, UserId};

pub async fn user_from_id(ctx: &Context, id: u64) -> Option<User> {
    UserId::new(id).to_user(&ctx.http).await.ok()
}
