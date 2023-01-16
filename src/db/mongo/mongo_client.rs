use mongodb::{Client, options::ClientOptions};
use dotenvy::dotenv;
use super::connect;

pub async fn mongos() -> Client {
  dotenv().ok();

  // unwrap, I know what these values are + won't work wihtout htem anyways
  let connection_parameters = connect::ConnectionString {
      username: String::from(std::env::var("mongo_username").unwrap()),
      password: String::from(std::env::var("mongo_password").unwrap()),
      cluster: String::from(std::env::var("cluster").unwrap()),
  };

  let url: String =
      connect::ConnectionString::build_connection_string(&connection_parameters);

  // I unwrap here because unless Mongo changes their strings/sunsets the old non srv version, should never panic
  let options = ClientOptions::parse(&url).await.unwrap();

  // Since failing this is dependent on options, I can just unwrap this as well
  return Client::with_options(options).unwrap();
}