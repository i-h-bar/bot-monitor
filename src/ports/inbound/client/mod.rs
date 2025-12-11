pub mod events;

use async_trait::async_trait;
#[async_trait]
pub trait Client {
    async fn run(&mut self);
}
