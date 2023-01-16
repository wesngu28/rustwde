mod db;
mod api;
mod jobs;
use std::{
    net::SocketAddr
};
use api::app::create_api;
use db::{techs::tweets::fill_tweet_db, mongo::mongo_client::mongos};
use reqwest::StatusCode;

// Result<(), Box<dyn std::error::Error>>
#[tokio::main]
async fn main() {
    // fill_tweet_db()
    let api = create_api().await;
    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));

    axum::Server::bind(&addr)
        .serve(api.into_make_service())
        .await
        .unwrap();
    // let db = client.database(&"webdevevaluator");
    // let collection = db.collection::<Document>("techtweets");
    // let filter = doc! { "_id": "@code" };
    // let find_options = FindOptions::builder().build();
    // let mut cursor = collection.find(filter, find_options).await?;

    // // Iterate over the results of the cursor.
    // while let Some(tech) = cursor.try_next().await? {
    //     println!("title: {}", tech);
    // }
    // Ok(())
}

// pub async fn handler_404() -> impl IntoResponse {
//     (StatusCode::NOT_FOUND, "nothing to see here")
// }
