use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct TechTweetStruct {
    pub name: String,
    pub tweets: String,
}

// Can unwrap this because I know the data source is always going to be the same, if it throws an error
// I should probably stop execution + know about it
pub fn fill_tweet_db() {
    let conn = Connection::open("src/db/techs/tweets.db").unwrap();

    conn.execute(
        "CREATE TABLE tweets (
        id   INTEGER PRIMARY KEY,
        name TEXT UNIQUE,
        tweets TEXT
    )",
        (), // empty list of parameters.
    )
    .unwrap();

    let dog = fs::read_dir("src/db/techs/json").unwrap();
    println!("{:#?}", dog);
    for file in dog {
        let file = file.unwrap();
        let file_path = file.path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let file_str = fs::read_to_string(&file_path).unwrap();
        let osha = TechTweetStruct {
            name: file_name.replace(".json", "").to_string(),
            tweets: file_str,
        };
        match conn.execute(
            "INSERT INTO tweets (name, tweets) values (?, ?);",
            params![osha.name, osha.tweets],
        ) {
            Ok(updated) => println!("something happened !!"),
            Err(err) => println!("something went wrong! you idiot!!! {}", err),
        }
    }
    conn.close().unwrap()
    // Ok(())
}
