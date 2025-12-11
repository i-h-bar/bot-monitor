use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};

pub async fn send_ephemeral(ctx: &Context, command: &CommandInteraction, message: &str) {
    let response = CreateInteractionResponseMessage::new()
        .content(message)
        .ephemeral(true);
    if let Err(why) = command
        .create_response(ctx, CreateInteractionResponse::Message(response))
        .await
    {
        log::warn!("Error sending message: {why:?}");
    }
}
