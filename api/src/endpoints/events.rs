use crate::endpoints::exceptions::Error;
use crate::models::event_model::TrafficEvent;
use crate::services::dynamodb::DbClient;
use rocket::serde::json::Json;
use rocket::State;

#[get("/v1/events/<event_id>")]
pub async fn get_event(
    event_id: &str,
    dynamo_client: &State<DbClient>,
) -> Result<Json<Vec<TrafficEvent>>, Error> {
    let results =
        match dynamo_client.get_event(event_id.to_string()).await.parse() {
            Ok(val) => val,
            Err(e) => {
                return Err(Error::DBError(e.into_service_error().to_string()))
            }
        };

    if results.len() == 0 {
        return Err(Error::NotFound(format!(
            "Event with ID {event_id} not found"
        )));
    }

    Ok(Json(results))
}

#[get("/v1/events")]
pub async fn get_active_events(
    dynamo_client: &State<DbClient>,
) -> Result<Json<Vec<TrafficEvent>>, Error> {
    let results = match dynamo_client.get_active_events().await.parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(Error::DBError(e.into_service_error().to_string()))
        }
    };

    if results.len() == 0 {
        return Err(Error::NotFound("No active events found".to_string()));
    }

    Ok(Json(results))
}
