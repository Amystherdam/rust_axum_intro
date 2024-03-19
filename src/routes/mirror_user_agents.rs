use axum::{headers::UserAgent, TypedHeader};

pub async fn mirror_user_agents(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
  user_agent.to_string()
}