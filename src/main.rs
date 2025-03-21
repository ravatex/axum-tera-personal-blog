mod cli;
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
    use clap::Parser;
    use cli::Commands::*;
    use cli::*;

    let cli = Cli::parse();

    match &cli.command {
        None => start_server().await,
        Some(Add(path)) => {
            let blog = posts::load_blog_post(&path.path);

            match blog {
                Err(e) => println!("Error with blog: {e}"),
                Ok(blog) => {
                    let _ = database::blog_posts::insert_blog_post(blog)
                        .inspect_err(|e| println!("Error with adding into database {e}"));
                }
            }
        }
        Some(Edit(info)) => {
            let id = info.blog_id;
            let blog = &info.path;

            let blog = posts::load_blog_post(&blog);

            match blog {
                Err(e) => println!("Error with blog: {e}"),
                Ok(blog) => match database::blog_posts::edit_blog_post(id, blog) {
                    Ok(0) => println!("Blog post with id: {id} not found"),
                    Ok(_) => println!("Blog post succesfully updated"),
                    Err(e) => println!("Error updating database {e}"),
                },
            }
        }
        Some(Remove(blog_deleting)) => {
            let id = blog_deleting.id;

            match remove_blog_post(id) {
                Ok(0) => println!("Blog with id not found"),
                Ok(_) => println!("Succesfully deleleted blog post with id {id}"),
                Err(e) => println!("Error deleting blog post {e}"),
            }
        }
        Some(Read(get_blogs)) => {
            let blogs = database::blog_posts::get_blog_posts();

            match blogs {
                Ok(blogs) => {
                    let mut blogs = blogs;

                    if let Some(id) = get_blogs.filters.filter_id {
                        blogs = blogs.into_iter().filter(|b| b.id.eq(&id)).collect();
                    }

                    if let Some(title) = &get_blogs.filters.filter_title {
                        blogs = blogs.into_iter().filter(|b| b.name.eq(title)).collect();
                    }

                    println!("Found {} matches:", blogs.len());
                    for blog in blogs {
                        println!();
                        if get_blogs.all_info {
                            println!("{blog:?}");
                            continue;
                        }
                        if get_blogs.ids {
                            println!("id: {}", blog.id);
                        }
                        if get_blogs.dates {
                            println!("date : {}", blog.date);
                        }

                        if get_blogs.names {
                            println!("name: {}", blog.name);
                        }
                    }
                }
                Err(e) => {
                    println!("Error with accessing database {e}")
                }
            }
        }
    }
}

async fn start_server() {
    println!("Starting the server");

    tokio::spawn(blog_refresher(tokio::time::Duration::new(5, 0)));

    let app = Router::new()
        .route("/", get(|| async { Html(index_page_filled()) }))
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

fn blogs_page_filled() -> String {
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
