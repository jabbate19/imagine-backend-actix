mod api;
mod models;
mod repository;

//modify imports below
use actix_web::{
    get,
    http::StatusCode,
    web::{Data, Json},
    App, HttpResponse, HttpServer,
};
use api::beacons::locations;
use futures::TryStreamExt;
use futures::{io::Cursor, StreamExt};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use repository::mongodb_repo::MongoRepo;

#[get("/beacons")]
pub async fn beacons(client: Data<MongoRepo>) -> HttpResponse {
    let collection: &Collection<models::beacon::beacon> = &client.beacon;
    match collection.find(None, None).await {
        Ok(mut cursor) => {
            let mut out: Vec<models::beacon::beacon> = Vec::new();
            while let Some(beacon) = cursor.try_next().await.unwrap() {
                out.push(beacon);
            }
            HttpResponse::Ok().json(out)
        }
        Err(e) => HttpResponse::build(StatusCode::from_u16(500).unwrap()).body(format!("{}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    // let client = Client::with_uri_str(uri).await.expect("failed to connect");
    let client = MongoRepo::init().await;
    let db = Data::new(client);

    HttpServer::new(move || App::new().app_data(db.clone())
        .service(locations))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
