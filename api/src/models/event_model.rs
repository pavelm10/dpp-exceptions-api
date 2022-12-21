use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrafficEvent {
    pub id: String,
    pub active: u8,
    pub url: String,
    pub lines: String,
    pub start_time: String,
}
