use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
  message: String,
  account: i32,
  username: String,
}

pub async fn get_json() -> Json<Data> {
  let data: Data = Data { 
    message: "Hello, World!".to_owned(),
    account: 12345,
    username: "user123".to_owned(),
  };

  Json(data)
}