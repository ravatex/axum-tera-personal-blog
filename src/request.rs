use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    name: String,
    email: String,
    message: String,
}

pub async fn message_post(message: axum::extract::Json<Message>) -> impl IntoResponse {
    println!("{:?}", message);
    (StatusCode::OK, "Message recieved succesfully")
}
