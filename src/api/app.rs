use axum::{
  routing::get,
  response::Redirect,
  http::{Method},
  Json,
  Router, extract::{Path, State}
};

use rusqlite::Connection;
// use mongodb::{bson::Document, bson::doc, Collection};
use tower_http::cors::{Any, CorsLayer};
use serde_json::json;

use crate::db::techs::tweets::TechTweetStruct;
// use crate::{db::mongo::mongo_client::mongos, jobs::jobs::get_jobs};

// use super::structs::MongoObject;

pub async fn create_api() -> Router {
    // let client = mongos().await;
    // let db = client.database(&"webdevevaluator");
    // let collection = db.collection::<Document>("techtweets");
    let app = Router::new()
      .route("/", get(|| async { Redirect::permanent("https://webdev-evaluator.vercel.app/") }))
      .route("/tweets/:tech", get(get_all_tweets))
      .layer(CorsLayer::new()
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

async fn get_all_tweets(Path(tech): Path<String>) -> String {
  let conn = Connection::open("src/db/techs/tweets.db").expect("Connection failed");

  let mut stmt = conn.prepare("SELECT * FROM Tweets where name = ?;").unwrap();
  let mut rows = stmt.query(rusqlite::params![tech]).unwrap();

  let mut items_list = Vec::new();

  while let Some(row) = rows.next().unwrap() {
      items_list.push(TechTweetStruct {
          name: row.get(1).unwrap(),
          tweets: row.get(2).unwrap(),
      });
  }

  let items_list = serde_json::to_string(&items_list).unwrap();

  items_list
}