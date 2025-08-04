use crate::domain::register::events::create::CreateEntry;
use crate::domain::register::events::remove::RemoveEntry;
use crate::domain::register::{Register, RegisterEntry, RegisterError};
use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;
use std::env;

pub struct DynamoDB(Client, String);

impl DynamoDB {
    pub async fn new() -> Self {
        let shared_config = aws_config::load_from_env().await;
        Self(Client::new(&shared_config), env::var("TABLE_NAME").unwrap())
    }
}

#[async_trait]
impl Register for DynamoDB {
    async fn fetch(&self, bot_id: String) -> Option<Vec<RegisterEntry>> {
        let bot_id_attr_value = AttributeValue::S(bot_id.clone());
        let query_op = self
            .0
            .query()
            .table_name(&self.1)
            .key_condition_expression("bot_id = :value")
            .expression_attribute_values(":value", bot_id_attr_value)
            .send()
            .await;

        let results = match query_op {
            Err(e) => {
                log::error!("{e:?}");
                return None;
            }
            Ok(value) => value.items?,
        };

        Some(
            results
                .into_iter()
                .filter_map(|value| {
                    let AttributeValue::S(user_id) = value.get("user_id")? else {
                        return None;
                    };

                    Some(RegisterEntry {
                        user_id: user_id.clone(),
                        bot_id: bot_id.clone(),
                    })
                })
                .collect(),
        )
    }

    async fn add(&self, entry: CreateEntry) -> Result<(), RegisterError> {
        let user_id = AttributeValue::S(entry.user_id);
        let bot_id = AttributeValue::S(entry.bot_id);
        let entry_version = AttributeValue::S(entry.version.to_string());

        let request = self
            .0
            .put_item()
            .table_name(&self.1)
            .item("bot_id", bot_id)
            .item("user_id", user_id)
            .item("entry_version", entry_version);

        if let Err(why) = request.send().await {
            log::error!("failed to send add request: {why:?}");
            return Err(RegisterError::EntryCreationError);
        }

        Ok(())
    }

    async fn remove(&self, entry: RemoveEntry) -> Result<(), RegisterError> {
        let user_id_attr_value = AttributeValue::S(entry.user_id);
        let bot_id_attr_value = AttributeValue::S(entry.bot_id);

        let query_op = self
            .0
            .delete_item()
            .table_name(&self.1)
            .key("bot_id", bot_id_attr_value)
            .send()
            .await;

        if let Err(why) = query_op {
            log::error!("failed to send delete request: {why:?}");
            return Err(RegisterError::EntryRemoveError);
        }

        Ok(())
    }

    async fn list(&self, user_id: String) -> Result<Vec<RegisterEntry>, RegisterError> {
        let user_id_attr_value = AttributeValue::S(user_id.clone());
        let query_op = self
            .0
            .query()
            .table_name(&self.1)
            .index_name("user_id-index")
            .key_condition_expression("user_id = :value")
            .expression_attribute_values(":value", user_id_attr_value)
            .send()
            .await;

        let results = match query_op {
            Err(e) => {
                log::error!("{e:?}");
                return Err(RegisterError::EntryFetchError);
            }
            Ok(value) => value.items.unwrap(),
        };

        Ok(results
            .into_iter()
            .filter_map(|value| {
                let AttributeValue::S(bot_id) = value.get("bot_id")? else {
                    return None;
                };

                Some(RegisterEntry {
                    user_id: user_id.clone(),
                    bot_id: bot_id.clone(),
                })
            })
            .collect())
    }
}
