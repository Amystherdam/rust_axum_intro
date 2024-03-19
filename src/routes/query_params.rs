use axum::extract::Query;
use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
  message: String,
  id: String,
}

pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> { 
  Json(QueryParams {
    message: format!("Your message: {0}", query.message), 
    id: format!("Your id: {0}", query.id)
  })
}