mod database;
mod html_insertion;
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
use database::blog_posts::*;
use posts::{blog_refresher, BlogError};
use tower_http::services::ServeDir;

use html_insertion::*;

use crate::posts::BlogPost;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]

async fn main() {
    use std::env::args;

    if args().nth(2).map_or(true, |s| s == "insert") {
        match args().nth(3) {
            None => println!("You need to supply an extra argument for the path to insert"),
            Some(val) => database::blog_posts::insert_blog_post(
                posts::load_blog_post(std::path::Path::new(&val)).unwrap(),
            )
            .unwrap(),
        }
    }

    database::cool_stuff();
    println!("Starting the server");

    tokio::spawn(blog_refresher(tokio::time::Duration::new(5, 0)));

    let app = Router::new()
        .route("/", get(|| async {Html(index_page_filled())}))
        .route(
            "/message",
            get(|| async { Html(contact_form()) }).post(message_post),
        )

        .route("/blogs", get(|| async { Html(blogs_page_filled()) }))
        .route("/blogs/{path}", get(|x| async { Html(blog_path(x)) }))
        .route("/notfound", get(|| async { Html(not_found_page()) }))
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
        *response.body_mut() = Body::from(not_found_page());
    }
    response
}



fn blog_path(Path(path): Path<i32>) -> Html<String> {
    use html_insertion::*;
    let render = match get_blog_post_from_id(path) {
        Some(blog) => make_blog(crate::BlogPost::from(blog)),
        None => error_to_page(BlogError::BlogNotFound(format!("{path}"))),
    };

    Html(render)
}

fn blogs_page_filled() -> String 
 {
    match database::blog_posts::get_blog_posts() {
        Ok(blogs) => {
            let s = blogs;
            blogs_page(s)
        }
        Err(e) => error_to_page(e),
    }
}

fn index_page_filled() -> String {
    let blog_posts = database::blog_posts::get_blog_posts();
    println!("{blog_posts:?}");
    match blog_posts {
        Ok(blogs) => {
            let s = blogs;
            index_page(s)
        }
        Err(e) => error_to_page(e),
    }


}
