mod database;
mod request;
mod visitor;
mod posts;
use crate::request::{message_post, Message};
use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tera::{Context, Tera};
use visitor::VisitorLog;

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
    static ref VISITORS: Arc<Mutex<VisitorLog>> = Arc::new(Mutex::new(VisitorLog::new()));
}

fn get_visitors() -> usize {
    match VISITORS.lock() {
        Ok(vis) => vis.get_all_visitors(),
        Err(_) => {
            println!("get visitors: thread panicked when holding lock");
            return 0;
        }
    }
}

fn increment_visitors() {
    match VISITORS.lock() {
        Ok(mut vis) => vis.add_visitor(),
        Err(_) => println!("get visitors: thread panicked when holding lock"),
    }
}

#[tokio::main]
async fn main() {
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
    increment_visitors();
    context.insert("visitors", &get_visitors());
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
