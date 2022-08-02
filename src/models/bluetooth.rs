use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct bluetooth_frame {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub sniffaddr: String,
    pub macaddr: String,
    pub rssi: i16,
    pub timestamp: u64,
}
