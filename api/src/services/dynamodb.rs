use crate::models::event_model::TrafficEvent;
use aws_sdk_dynamodb::{
    error::ScanError, model::AttributeValue, output::ScanOutput,
    types::SdkError, Client,
};

pub struct DbClient {
    pub dynamo_client: Client,
    pub table_name: String,
}

pub struct DbParser {
    query_output: Result<ScanOutput, SdkError<ScanError>>,
}

impl DbClient {
    pub async fn get_event(&self, event_id: String) -> DbParser {
        let results = self
            .dynamo_client
            .scan()
            .table_name(&self.table_name)
            .filter_expression("event_id = :eid")
            .expression_attribute_values(
                ":eid",
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
            .scan()
            .table_name(&self.table_name)
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
    pub fn parse(self) -> Result<Vec<TrafficEvent>, SdkError<ScanError>> {
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
