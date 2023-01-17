use axum::{
    extract::{Path, State},
    http::{Method},
    response::Redirect,
    routing::get,
    Router, Json
};

use mongodb::{bson::Document, bson::doc, Collection};
use serde_json::{json, Value};
use tower_http::cors::{Any, CorsLayer};

use crate::{db::mongo_client::mongos, jobs::jobs::get_jobs};

pub async fn create_api() -> Router {
    let client = mongos().await;
    let db = client.database(&"webdevevaluator");
    let collection = db.collection::<Document>("techtweet");
    let app = Router::new()
        .route(
            "/",
            get(|| async { Redirect::permanent("https://webdev-evaluator.vercel.app/") }),
        )
        .route("/linkedin/:tech", get(get_linkedin_jobs))
        .route("/tweets/:tech", get(get_all_tweets))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::POST]),
        );
    let app = app.with_state(collection);
    app
}

async fn get_all_tweets(State(collection): State<Collection<Document>>, Path(tech): Path<String>) -> Json<Value> {
    let filter = doc! { "name": &tech };
    let cursor = collection.find_one(filter, None).await.unwrap();
    let json_docs = json!(cursor);
    Json(json_docs)
}

async fn get_linkedin_jobs(Path(tech): Path<String>) -> Json<Value> {
    let linkedin = json!(get_jobs(tech).await.unwrap());
    Json(linkedin)
}