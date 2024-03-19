use serde::{Deserialize, Serialize};
use validator::Validate;
use axum::{
    async_trait, 
    body::HttpBody, 
    extract::{rejection::JsonRejection, FromRequest}, 
    http::{Request, StatusCode}, BoxError, Json, RequestExt
};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct User {
  #[validate(email(message = "Invalid email"))]
    username: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    password: String,
    status: UserStatus,
}

#[derive(Debug, Deserialize, Serialize)]
enum UserStatus {
    Active,
    Inactive,
}

impl Default for UserStatus {
    fn default() -> Self {
        UserStatus::Inactive
    }
}

#[async_trait]
impl<S, B> FromRequest<S, B> for User
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync, // Exemplo de restrição para S
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = req
            .extract::<Json<User>, _>() // Provide both generic arguments
            .await
            .map_err(|error: JsonRejection| (StatusCode::BAD_REQUEST, format!("{}", error)))?;
    
        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }
    
        Ok(user)
    }
}


pub async fn validate_with_serde(user: User) -> Json<User> {
    Json(user)
}
