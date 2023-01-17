use axum::{
    body::{Body, HttpBody},
    extract::{Path, State},
    headers::{
        authorization::{Basic, Bearer},
        Authorization, UserAgent,
    },
    http::{HeaderMap, Method, Response},
    response::Redirect,
    routing::get,
    Json, Router, TypedHeader,
};

use reqwest::StatusCode;
use rusqlite::Connection;
// use mongodb::{bson::Document, bson::doc, Collection};
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};

use crate::db::techs::tweets::TechTweetStruct;
// use crate::{db::mongo::mongo_client::mongos, jobs::jobs::get_jobs};

// use super::structs::MongoObject;

pub async fn create_api() -> Router {
    // let client = mongos().await;
    // let db = client.database(&"webdevevaluator");
    // let collection = db.collection::<Document>("techtweets");
    let app = Router::new()
        .route(
            "/",
            get(|| async { Redirect::permanent("https://webdev-evaluator.vercel.app/") }),
        )
        .route("/tweets/:tech", get(get_all_tweets))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::POST]),
        );
    // let app = app.with_state(collection);
    app
}

// async fn get_all_tweets(State(collection): State<Collection<Document>>, Path(tech): Path<String>) -> Json<MongoObject> {
//     let filter = doc! { "_id": &tech };
//     let cursor = collection.find_one(filter, None).await.unwrap();
//     let mut json_docs = json!(cursor);
//     json_docs["linkedin_count"] = json!(get_jobs(tech).await.unwrap());
//     let mongo_object: MongoObject = serde_json::from_value(json_docs).unwrap();
//     Json(mongo_object)
// }

use serde_json::Value;
async fn get_all_tweets(Path(tech): Path<String>, headers: HeaderMap) -> Response<Body> {
    let api_key = match headers.get("API_KEY") {
        Some(api_key) => api_key,
        None => {
            return Response::builder()
                .status(401)
                .body(Body::from("Unauthorized, no API Key provided"))
                .unwrap()
        }
    };
    let api_key = api_key.to_str().unwrap();
    let env_api_key = std::env::var("API_KEY").unwrap();
    if api_key != env_api_key {
        return Response::builder()
            .status(401)
            .body(Body::from("Unauthorized, incorrect API Key"))
            .unwrap();
    }

    let conn = Connection::open("src/db/techs/tweets.db").expect("Connection failed");

    let mut stmt = conn
        .prepare("SELECT * FROM Tweets where name = ?;")
        .unwrap();
    let mut rows = stmt.query(rusqlite::params![tech]).unwrap();

    let mut items_list = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        items_list.push(TechTweetStruct {
            name: row.get(1).unwrap(),
            friendly_name: row.get(2).unwrap(),
            fireship: row.get(3).unwrap_or(Some(String::from(""))),
            docs: row.get(4).unwrap(),
            repo: row.get(5).unwrap_or(Some(String::from(""))),
            tweets: row.get(6).unwrap(),
            wordcount: row.get(7).unwrap(),
        });
    }

    let items_list = serde_json::to_string(&items_list).unwrap();

    Response::builder()
        .status(200)
        .body(Body::from(items_list))
        .unwrap()
}
