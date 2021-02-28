use crate::db::DbPool;
use crate::models::Marker;
use actix_web::{
    get,
    web::{block, Data, Query},
    Error, HttpResponse,
};
use jsonapi::api::*;
use jsonapi::jsonapi_model;
use jsonapi::model::*;

jsonapi_model!(Marker; "marker");

#[derive(Deserialize)]
struct MarkersGetQuery {
    west: f32,
    east: f32,
    north: f32,
    south: f32,
    zoom: i8,
}

#[get("/markers")]
async fn get(pool: Data<DbPool>, query: Query<MarkersGetQuery>) -> Result<HttpResponse, Error> {
    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    let markers = block(move || {
        Marker::find(
            &conn,
            query.south,
            query.north,
            query.west,
            query.east,
            query.zoom,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let doc = vec_to_jsonapi_document(markers);

    Ok(HttpResponse::Ok().json(doc))
}
