use axum::{headers::HeaderValue, http::HeaderMap};

pub async fn mirror_custom_header(headers: HeaderMap) -> String {
  let message_value: &HeaderValue = headers.get("x-message").unwrap();
  let message = message_value.to_str().unwrap().to_owned();

  let user_agent_value: &HeaderValue = headers.get("User-Agent").unwrap();
  let user_agent = user_agent_value.to_str().unwrap().to_owned();
  
  format!("Your headers: {message} and {user_agent}")
}