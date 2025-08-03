use async_trait::async_trait;
use aws_sdk_dynamodb::{Client as DynamoDBClient, Error};
use crate::domain::register::{Register, RegisterEntry, RegisterError};

struct DynamoDB(DynamoDBClient);


impl DynamoDB {
    pub async fn new() -> Self {
        let shared_config = aws_config::load_from_env().await;
        Self(DynamoDBClient::new(&shared_config))
    }
}

#[async_trait]
impl Register for DynamoDB {
    async fn fetch(&self, bot_id: u64) -> Option<RegisterEntry> {
        todo!()
    }

    async fn add(&self, entry: RegisterEntry) -> Result<(), RegisterError> {
        todo!()
    }

    async fn remove(&self, bot_id: u64, user_id: u64) -> Result<(), RegisterError> {
        todo!()
    }

    async fn list(&self, user_id: u64) -> Result<Vec<RegisterEntry>, RegisterError> {
        todo!()
    }
}