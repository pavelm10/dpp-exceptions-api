mod endpoints;
mod models;
mod services;

#[macro_use]
extern crate rocket;

use crate::services::dynamodb::DbClient;
use aws_sdk_dynamodb::Client;
use endpoints::events::{get_active_events, get_event};

#[launch]
async fn rocket() -> _ {
    let shared_config = aws_config::load_from_env().await;

    let db_client = DbClient {
        dynamo_client: Client::new(&shared_config),
        table_name: "dpp-notifier-events".to_string(),
    };

    let rocket = rocket::build().manage(db_client);

    rocket.mount("/api", routes![get_event, get_active_events])
}
