use crate::models::event_model::TrafficEvent;
use aws_sdk_dynamodb::{
    error::QueryError, model::AttributeValue, output::QueryOutput,
    types::SdkError, Client,
};

pub struct DbClient {
    pub dynamo_client: Client,
    pub table_name: String,
}

pub struct DbParser {
    query_output: Result<QueryOutput, SdkError<QueryError>>,
}

impl DbClient {
    pub async fn get_event(&self, event_id: String) -> DbParser {
        let results = self
            .dynamo_client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("event_type = :pk and event_id = :sk")
            .expression_attribute_values(
                ":pk",
                AttributeValue::S("dpp".to_string()),
            )
            .expression_attribute_values(
                ":sk",
                AttributeValue::S(event_id.to_string()),
            )
            .send()
            .await;
        DbParser {
            query_output: results,
        }
    }

    pub async fn get_active_events(&self) -> DbParser {
        let results = self
            .dynamo_client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("event_type = :pk")
            .expression_attribute_values(
                ":pk",
                AttributeValue::S("dpp".to_string()),
            )
            .filter_expression("active = :num")
            .expression_attribute_values(
                ":num",
                AttributeValue::N("1".to_string()),
            )
            .send()
            .await;
        DbParser {
            query_output: results,
        }
    }
}

impl DbParser {
    pub fn parse(self) -> Result<Vec<TrafficEvent>, SdkError<QueryError>> {
        let mut items = vec![];
        for item in self
            .query_output?
            .items()
            .map(|slice| slice.to_vec())
            .unwrap()
        {
            let mut lines = vec![];
            for line in item["lines"].as_l().unwrap() {
                lines.push(line.as_s().unwrap().to_string());
            }
            let ev: TrafficEvent = TrafficEvent {
                id: item["event_id"].as_s().unwrap().to_owned(),
                active: item["active"]
                    .as_n()
                    .unwrap()
                    .to_owned()
                    .parse()
                    .unwrap(),
                url: item["url"].as_s().unwrap().to_owned(),
                lines: lines.join(","),
                start_time: item["start_date"].as_s().unwrap().to_owned(),
            };
            items.push(ev)
        }
        Ok(items)
    }
}
