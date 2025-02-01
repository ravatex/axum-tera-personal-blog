use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    name: String,
    email: String,
    message: String,
}

impl Message {
    pub fn new() -> Message {
        Message {
            name: "hello".to_string(),
            email: "bye".to_string(),
            message: " some".to_string(),
        }
    }
}

pub async fn message_post(message: axum::extract::Json<Message>) -> impl IntoResponse {
    println!("{:?}", message);
    (StatusCode::OK, "Message recieved succesfully")
}
