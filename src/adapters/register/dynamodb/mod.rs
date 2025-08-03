use std::env;
use std::str::FromStr;
use async_trait::async_trait;
use aws_sdk_dynamodb::{Client as DynamoDBClient, Error};
use aws_sdk_dynamodb::types::AttributeValue;
use serenity::json::to_string;
use crate::domain::register::{Register, RegisterEntry, RegisterError};

pub struct DynamoDB(DynamoDBClient, String);


impl DynamoDB {
    pub async fn new() -> Self {
        let shared_config = aws_config::load_from_env().await;
        Self(DynamoDBClient::new(&shared_config), env::var("TABLE_NAME").unwrap())
    }
}

#[async_trait]
impl Register for DynamoDB {
    async fn fetch(&self, bot_id: u64) -> Option<Vec<RegisterEntry>> {
        let bot_id_attr_value = AttributeValue::S(bot_id.to_string());
        let query_op = self.0
            .query()
            .table_name(&self.1)
            .key_condition_expression("bot_id = :value")
            .expression_attribute_values(":value", bot_id_attr_value)
            .send()
            .await;

        let results = match query_op {
            Err(e) => { log::error!("{:?}", e); return None; },
            Ok(value)  => value.items?
        };

        Some(results.into_iter().filter_map(| value | {
            let AttributeValue::S(user_id) = value.get("user_id")? else { return None };
            
            Some(RegisterEntry {
                user_id: u64::from_str(&user_id).ok()?,
                bot_id
            })
        }).collect())
    }

    async fn add(&self, entry: RegisterEntry) -> Result<(), RegisterError> {
        let user_id = AttributeValue::S(entry.user_id.to_string());
        let bot_id = AttributeValue::S(entry.bot_id.to_string());

        let request = self.0.put_item().table_name(&self.1).item("bot_id", bot_id).item("user_id", user_id);

        if let Err(why) = request.send().await {
            log::error!("failed to send to request: {:?}", why);
            return Err(RegisterError::EntryCreationError);
        }

        Ok(())
    }

    async fn remove(&self, bot_id: u64, user_id: u64) -> Result<(), RegisterError> {
        todo!()
    }

    async fn list(&self, user_id: u64) -> Result<Vec<RegisterEntry>, RegisterError> {
        todo!()
    }
}