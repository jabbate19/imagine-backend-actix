use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{bson::extjson::de::Error, results::InsertOneResult, Client, Collection};

use crate::models::*;

pub struct MongoRepo {
    pub bluetooth: Collection<bluetooth::bluetooth_frame>,
    pub heartbeat: Collection<heartbeat::heartbeat_frame>,
    pub beacon: Collection<beacon::beacon>,
    pub esp: Collection<esp::esp>,
    pub output: Collection<output::output>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGODB_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("imagine2022");
        MongoRepo {
            bluetooth: db.collection("bluetooth_frames"),
            heartbeat: db.collection("heartbeat_frames"),
            beacon: db.collection("prod_beacons"),
            esp: db.collection("prod_esps"),
            output: db.collection("prod_output"),
        }
    }
}
