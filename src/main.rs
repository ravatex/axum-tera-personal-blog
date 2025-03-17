mod database;
mod posts;
mod request;
mod visitor;
use request::message_post;

use axum::{
    body::Body,
    extract::Path,
    http::{HeaderValue, Request},
    middleware::{self, Next},
    response::{Html, Response},
    routing::{get, post},
    Router,
};
use lazy_static::lazy_static;
use posts::{blog_refresher, get_blogs, BlogError};
use tera::{Context, Tera};
use tower_http::services::ServeDir;
use visitor::{get_visitors, increment_visitors};

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

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]

async fn main() {
    database::cool_stuff();
    println!("Starting the server");

    tokio::spawn(blog_refresher(tokio::time::Duration::new(5, 0)));

    let app = Router::new()
        .route("/", get(index_page))
        .route("/message", get(contact_form).post(message_post))
        .route("/blogs", get(blogs_page))
        .route("/blogs/{path}", get(get_blog_from_path))
        .route("/notfound", get(not_found_page))
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/images", ServeDir::new("images"))
        .layer(middleware::from_fn(no_cache_middleware))
        .layer(middleware::from_fn(not_found_middleware));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn no_cache_middleware(request: Request<Body>, next: Next) -> Response {
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        "Cache-Control",
        HeaderValue::from_static("no-cache, no-store, must-revalidate"),
    );
    response
}

async fn not_found_middleware(request: Request<Body>, next: Next) -> Response {
    let mut response = next.run(request).await;
    if response.status() == 404 {
        *response.body_mut() = Body::from(not_found_page().await);
    }
    response
}

async fn not_found_page() -> String {
    let context = get_base_context();
    TEMPLATES
        .render("notfound.html", &context)
        .unwrap_or_else(error_to_page)
}

async fn index_page() -> Html<String> {
    let mut context = get_base_context();
    increment_visitors();
    context.insert("posts", &*get_blogs().await);
    let finished = TEMPLATES
        .render("index.html", &context)
        .unwrap_or_else(error_to_page);

    Html(finished)
}

async fn blogs_page() -> Html<String> {
    let mut context = get_base_context();

    context.insert("posts", &*get_blogs().await);

    let finished = TEMPLATES
        .render("blogs.html", &context)
        .unwrap_or_else(error_to_page);

    Html(finished)
}

async fn contact_form() -> Html<String> {
    Html(
        TEMPLATES
            .render("contact.html", &get_base_context())
            .unwrap_or_else(error_to_page),
    )
}

fn error_to_page<T: std::error::Error>(error: T) -> String {
    let mut context = Context::new();

    context.insert("error", &error.to_string());
    println!("error: {:?}", error);

    TEMPLATES.render("error.html", &context).unwrap()
}

async fn get_blog_from_path(Path(path): Path<String>) -> Html<String> {
    let blogs = &*get_blogs().await;
    let post = blogs.iter().find(|x| x.path == path);

    let page = match post {
        Some(post) => {
            let mut context = get_base_context();
            context.insert("post", post);
            TEMPLATES
                .render("blog_form.html", &context)
                .unwrap_or_else(error_to_page)
        }
        None => error_to_page(BlogError::BlogNotFound(path)),
    };

    Html(page)
}

fn get_base_context() -> Context {
    let mut context = Context::new();
    context.insert("visitors", &get_visitors());
    context
}
