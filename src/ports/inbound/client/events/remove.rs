use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct RemoveEntry {
    pub user_id: String,
    pub bot_id: String,
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait RemoveEvent {
    fn entry(&self) -> RemoveEntry;
    async fn failed_message(&self);
    async fn success_message(&self);
}
