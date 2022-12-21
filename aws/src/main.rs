use aws_sdk_dynamodb::{
    model::AttributeValue, output::ScanOutput, Client, Error,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrafficEvent {
    pub id: String,
    pub active: u8,
    pub url: String,
    pub lines: String,
    pub start_time: String,
}

fn parse(results: ScanOutput) -> Vec<TrafficEvent> {
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
    items
}

async fn get_active_events(client: &Client) -> ScanOutput {
    client
        .scan()
        .table_name("dpp-notifier-events")
        .filter_expression("active = :num")
        .expression_attribute_values(
            ":num",
            AttributeValue::N("1".to_string()),
        )
        .send()
        .await
        .unwrap()
}

async fn get_event(client: &Client, event_id: &str) -> ScanOutput {
    client
        .scan()
        .table_name("dpp-notifier-events")
        .filter_expression("event_id = :eid")
        .expression_attribute_values(
            ":eid",
            AttributeValue::S(event_id.to_string()),
        )
        .send()
        .await
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;

    let client = Client::new(&shared_config);
    let event_id = "1396-3";

    let results = get_active_events(&client).await;
    let event_results = get_event(&client, event_id).await;

    let items = parse(results);
    let event_items = parse(event_results);

    dbg!(items);
    dbg!(event_items);

    Ok(())
}
