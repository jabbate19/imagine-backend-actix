use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct output {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub position: Vec<f64>,
    pub absolute_position: Vec<f64>,
    pub esps: HashMap<String, esp_data>,
    pub beacon_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct esp_data {
    timestamp: u64,
    rssi: i16,
    esp_position: Vec<f64>,
    esp_position_normal: Vec<f64>,
    distance: f64,
}
