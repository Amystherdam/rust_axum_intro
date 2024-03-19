mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;
mod mirror_user_agents;
mod mirror_custom_header;
mod aways_errors;
mod get_json;
mod validate_with_serde;

use axum::{Router, body::Body, routing::{get, post}, http::Method};
use tower_http::cors::{Any, CorsLayer};


use hello_world::hello_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::path_variables;
use query_params::query_params;
use mirror_user_agents::mirror_user_agents;
use mirror_custom_header::mirror_custom_header;
use aways_errors::aways_errors;
use get_json::get_json;
use validate_with_serde::validate_with_serde;

pub fn create_routes() -> Router<(), Body> {
  let layer = CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any);

  Router::new()
    .route("/", get(hello_world))
    .route("/mirror_body_string", post(mirror_body_string))
    .route("/mirror_body_json", post(mirror_body_json))
    .route("/path_variables/:id", get(path_variables))
    .route("/query_params", get(query_params))
    .route("/mirror_user_agents", get(mirror_user_agents))
    .route("/mirror_custom_header", get(mirror_custom_header))
    .layer(layer)
    .route("/aways_errors", get(aways_errors))
    .route("/get_json", get(get_json))
    .route("/validate_with_serde", post(validate_with_serde))
}