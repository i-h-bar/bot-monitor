use crate::domain::events::create::CreateEntry;
use crate::domain::events::list::ListEntriesPayload;
use crate::domain::events::remove::RemoveEntry;
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

    async fn list(&self, entry: ListEntriesPayload) -> Result<Vec<RegisterEntry>, RegisterError> {
        let user_id_attr_value = AttributeValue::S(entry.user_id.clone());
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
                    user_id: entry.user_id.clone(),
                    bot_id: bot_id.clone(),
                })
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_sdk_dynamodb::Client;
    use aws_sdk_dynamodb::operation::put_item::PutItemOutput;
    use aws_smithy_mocks::{mock, mock_client};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_add() {
        let bot_id = String::from("bot_id_12345");
        let user_id = String::from("user_id_12345");
        let version = 0;

        let entry = CreateEntry {
            user_id: user_id.clone(),
            bot_id: bot_id.clone(),
            version: version.clone(),
        };

        let put_object = mock!(Client::put_item)
            .match_requests(move |req| {
                req.table_name == Some(String::from("test-register"))
                    && req.item
                        == Some(HashMap::from([
                            (String::from("bot_id"), AttributeValue::S(bot_id.clone())),
                            (String::from("user_id"), AttributeValue::S(user_id.clone())),
                            (
                                String::from("entry_version"),
                                AttributeValue::S(version.to_string()),
                            ),
                        ]))
            })
            .then_output(|| {
                PutItemOutput::builder()
                    .set_attributes(Some(HashMap::from([(
                        String::from("bot_id"),
                        AttributeValue::S(String::from("12345")),
                    )])))
                    .build()
            });

        let dynamodb_client = mock_client!(aws_sdk_dynamodb, [&put_object]);

        let dynamo_register = DynamoDB(dynamodb_client, String::from("test-register"));

        let return_value = dynamo_register.add(entry).await.unwrap();
        assert_eq!(put_object.num_calls(), 1);
        assert_eq!(return_value, ())
    }
}
