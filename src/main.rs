mod request;
mod database;
use crate::request::{message_post, Message};
use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use lazy_static::lazy_static;
use std::sync::atomic::AtomicU64;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

lazy_static! {
    static ref VISITOR_COUNT: AtomicU64 = AtomicU64::new(0);
}

fn get_visitors() -> u64 {
    VISITOR_COUNT.load(std::sync::atomic::Ordering::SeqCst)
}

fn increment_visitors() -> u64 {
    VISITOR_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1
}

#[tokio::main]
async fn main() {
    println!("{}", serde_json::to_string(&Message::new()).unwrap());
    println!("Starting the server");
    let app = Router::new()
        .route("/", get(index_page))
        .route("/message", get(contact_form).post(message_post));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index_page() -> Html<String> {
    let mut context = Context::new();
    context.insert("visitors", &increment_visitors());
    let finished = TEMPLATES
        .render("index.html", &context)
        .unwrap_or_else(error_to_page);

    Html(finished)
}

async fn contact_form() -> Html<String> {
    Html(
        TEMPLATES
            .render("contact.html", &Context::new())
            .unwrap_or_else(error_to_page),
    )
}

fn error_to_page<T: std::error::Error>(error: T) -> String {
    let mut context = Context::new();
    context.insert("error", &error.to_string());

    TEMPLATES.render("error.html", &context).unwrap()
}
