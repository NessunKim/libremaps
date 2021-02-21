use actix_web::{get, web::Query, HttpResponse, Responder};
use jsonapi::api::*;
use jsonapi::jsonapi_model;
use jsonapi::model::*;
use rand::Rng;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Marker {
    id: String,
    name: String,
    latitude: f32,
    longitude: f32,
    zoom: i32,
}
jsonapi_model!(Marker; "markers");

#[derive(Deserialize)]
struct MarkersGetQuery {
    west: f32,
    east: f32,
    north: f32,
    south: f32,
    zoom: i32,
}

#[get("/markers")]
async fn get(query: Query<MarkersGetQuery>) -> impl Responder {
    let mut rng = rand::thread_rng();
    let mut markers = vec![];
    for i in 1..=30 {
        let marker = Marker {
            id: format!("{}", i),
            name: format!("Example{}", i),
            latitude: rng.gen_range(query.south..query.north),
            longitude: rng.gen_range(query.west..query.east),
            zoom: rng.gen_range(0..=query.zoom),
        };
        markers.push(marker)
    }

    let doc = vec_to_jsonapi_document(markers);

    HttpResponse::Ok().json(doc)
}
