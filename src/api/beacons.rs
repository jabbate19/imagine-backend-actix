use std::collections::HashMap;

use actix_web::{get, web::{Data,Json}, App,http::StatusCode, HttpServer, HttpResponse};
use mongodb::bson::doc;
use crate::models::output::output;
use crate::repository::mongodb_repo::MongoRepo;
use futures::{io::Cursor, StreamExt, TryStreamExt};

#[get("/beacons/locations")]
pub async fn locations(db: Data<MongoRepo>) -> HttpResponse {
    let mut all_output = &mut db.output.find(None,None).await.unwrap();
    let mut out: HashMap<String, output> = HashMap::new();
    while let Some(output) = all_output.try_next().await.unwrap() {
        let beacon_id = &output.beacon_id;
        let mut beacon_find = &mut db.beacon.find(doc!{"id": beacon_id}, None).await.unwrap();
        let mut beacon_hidden = true;
        while let Some(beacon) = beacon_find.try_next().await.unwrap() {
            if !beacon.hidden {
                beacon_hidden = false;
                break;
            }
        }
        if !beacon_hidden {
            out.insert(beacon_id.to_string(), output);
        }
    }
    HttpResponse::Ok().json(out)
}