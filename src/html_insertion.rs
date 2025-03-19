use lazy_static::lazy_static;
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
pub trait IntoBlog {
    fn blog_name(&self) -> String;
    fn blog_date(&self) -> String;
    fn blog_message(&self) -> String;
    fn blog_thumbnail(&self) -> Option<String>;
    fn blog_id(&self) -> String;
}

#[derive(serde::Serialize)]
struct InsertBlog {
    blog_name: String,
    blog_date: String,
    blog_message: String,
    blog_thumbnail: Option<String>,
    blog_id: String,
}

impl InsertBlog {
    fn create_from(blog: impl IntoBlog) -> Self {
        InsertBlog {
            blog_name: blog.blog_name(),
            blog_date: blog.blog_date(),
            blog_message: blog.blog_message(),
            blog_thumbnail: blog.blog_thumbnail(),
            blog_id: blog.blog_id(),
        }
    }
}



use crate::visitor::get_visitors;

pub fn get_base_context() -> Context {
    let mut context = Context::new();
    context.insert("visitors", &get_visitors());
    context
}

pub fn error_to_page<T: std::error::Error>(error: T) -> String {
    let mut context = Context::new();

    context.insert("error", &error.to_string());
    println!("error: {:?}", error);

    TEMPLATES.render("error.html", &context).unwrap()
}

pub fn contact_form() -> String {
    TEMPLATES
        .render("contact.html", &get_base_context())
        .unwrap_or_else(error_to_page)
}
pub fn not_found_page() -> String {
    let context = get_base_context();
    TEMPLATES
        .render("notfound.html", &context)
        .unwrap_or_else(error_to_page)
}

pub fn blogs_page<I: IntoIterator<Item = impl IntoBlog>>(blogs: I) -> String {
    let mut context = get_base_context();

    let blogs: Vec<_> = blogs.into_iter().map(InsertBlog::create_from).collect();

    context.insert("posts", &blogs);

    let finished = TEMPLATES
        .render("blogs.html", &context)
        .unwrap_or_else(error_to_page);

    finished
}

pub fn index_page<I: IntoIterator<Item = impl IntoBlog>>(blogs: I) ->String {
    let mut context = get_base_context();

    let blogs: Vec<_> = blogs.into_iter().map(InsertBlog::create_from).collect();
    context.insert("posts", &blogs);

    let finished = TEMPLATES
        .render("index.html", &context)
        .unwrap_or_else(error_to_page);

    finished
}
pub fn make_blog(blog: impl IntoBlog) -> String {
    let mut context = get_base_context();

    context.insert("post", &InsertBlog::create_from(blog));

    TEMPLATES
        .render("blog_form.html", &context)
        .unwrap_or_else(error_to_page)
}
