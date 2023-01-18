use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, Method, Response},
    response::Redirect,
    routing::get,
    Router,
};

use mongodb::{bson::doc, bson::Document, Collection};
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

async fn get_all_tweets(
    State(collection): State<Collection<Document>>,
    Path(tech): Path<String>,
    headers: HeaderMap,
) -> Response<Body> {
    let api_key_checker = check_api_key(headers);
    if api_key_checker.contains("Unauthorized") {
        return Response::builder()
            .status(401)
            .body(Body::from(api_key_checker))
            .unwrap();
    }
    let filter = doc! { "name": &tech };
    let cursor = match collection.find_one(filter, None).await {
        Ok(cursor) => cursor,
        Err(e) => {
            return Response::builder()
                .status(404)
                .body(Body::from(format!("Could not find {} on server.", &tech)))
                .unwrap()
        }
    };

    // If something is found, it should always be valid
    match serde_json::to_string(&cursor) {
        Ok(body) => {
            return Response::builder()
                .status(200)
                .body(Body::from(body))
                .unwrap()
        }
        Err(e) => {
            return Response::builder()
                .status(400)
                .body(Body::from(format!(
                    "Something went wrong with your request for {}.",
                    &tech
                )))
                .unwrap()
        }
    };
}

async fn get_linkedin_jobs(Path(tech): Path<String>, headers: HeaderMap) -> Response<Body> {
    let api_key_checker = check_api_key(headers);
    if api_key_checker.contains("Unauthorized") {
        return Response::builder()
            .status(401)
            .body(Body::from(api_key_checker))
            .unwrap();
    }
    let linkedin = get_jobs(tech).await.unwrap();
    Response::builder()
        .status(200)
        .body(Body::from(linkedin))
        .unwrap()
}

fn check_api_key(headers: HeaderMap) -> String {
    let api_key = match headers.get("API_KEY") {
        Some(api_key) => api_key,
        None => {
            return "Unauthorized, no API Key provided".to_string();
        }
    };
    let api_key = api_key.to_str().unwrap();
    let env_api_key = std::env::var("API_KEY").unwrap();
    if api_key != env_api_key {
        return "Unauthorized, incorrect API Key".to_string();
    }
    return "Proceed".to_string();
}
