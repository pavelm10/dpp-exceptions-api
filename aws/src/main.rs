use aws_sdk_dynamodb::{model::AttributeValue, Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrafficEvent {
    pub id: String,
    pub active: u8,
    pub url: String,
    pub lines: String,
    pub start_time: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;

    let client = Client::new(&shared_config);
    let event_id = "1396-3";

    let results = client
        .query()
        .table_name("dpp-notifier-events")
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
        .await?;

    let mut items = vec![];
    for item in results.items().map(|slice| slice.to_vec()).unwrap() {
        let mut lines = vec![];
        for line in item["lines"].as_l().unwrap() {
            lines.push(line.as_s().unwrap().to_string());
        }
        let ev: TrafficEvent = TrafficEvent {
            id: item["event_id"].as_s().unwrap().to_owned(),
            active: item["active"].as_n().unwrap().to_owned().parse().unwrap(),
            url: item["url"].as_s().unwrap().to_owned(),
            lines: lines.join(","),
            start_time: item["start_date"].as_s().unwrap().to_owned(),
        };
        items.push(ev)
    }
    dbg!(items);
    Ok(())
}
