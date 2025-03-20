use axum::extract::Json;
use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::database::inquiries::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub name: String,
    pub email: String,
    pub message: String,
    pub is_business: bool,
}

pub async fn message_post(Json(message): Json<Message>) -> impl IntoResponse {
    println!("{:?}", message);

    println!("{:?}", get_business_inquiries_by_company());
    match insert_inquiry(message) {
        Ok(()) => (StatusCode::OK, "Message recieved succesfully".into()),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error Inserting into Database: {error:?}"),
        ),
    }
}
