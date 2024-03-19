use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJson {
  message: String
}

pub async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJson> {
    Json(MirrorJson {message: body.message + "oi"} )
}