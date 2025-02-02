use axum::{http::StatusCode, response::IntoResponse};
use axum::extract::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    name: String,
    email: String,
    message: String,
    is_business: bool,
}

pub async fn message_post(Json(message): Json<Message>) -> impl IntoResponse {
    println!("{:?}", message);
    (StatusCode::OK, "Message recieved succesfully")
}
