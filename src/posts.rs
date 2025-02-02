use comrak::markdown_to_html;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use std::{fs, path::Path};
use tokio::sync::RwLock;

#[derive(Debug)]
pub enum BlogError {
    BlogNotFound(String),
    NoAccess(String),
    BlogParsingError(String),
}

impl std::fmt::Display for BlogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BlogNotFound(err) => write!(f, "Blog not found at path {}", err),
            Self::NoAccess(err) => write!(f, "No access to blog:  {}", err),
            Self::BlogParsingError(err) => write!(f, "Blog could not be parsed {}", err),
        }
    }
}

impl std::error::Error for BlogError {}

#[derive(Serialize)]
pub struct BlogPost {
    pub path: String,
    pub contents: String,
    pub blog_data: BlogData,
}
#[derive(Serialize, Deserialize)]
pub struct BlogData {
    pub title: String,
    pub date: String,
    pub visible: bool,
    pub thumbnail: Option<String>,
}

fn load_blog_post(path: &Path) -> Result<BlogPost, Box<dyn std::error::Error>> {
    let blog_post = fs::read_to_string(path)?;

    let lines: Vec<&str> = blog_post.splitn(3, "---").collect();

    if lines.len() < 2 {
        return Err("No blog data (---)".into());
    }

    let blog_data: BlogData = serde_json::from_str(lines[1])?;
    

    let md_html = markdown_to_html(lines[2], &comrak::Options::default());

    let name = path
        .file_name()
        .ok_or("not a good path")?
        .to_str()
        .ok_or("non unicode path")?;

    Ok(BlogPost {
        blog_data,
        path: name.to_string(),
        contents: md_html,
    })
}

pub fn get_all_blog_posts() -> Vec<BlogPost> {
    let mut posts = Vec::new();

    match fs::read_dir("posts") {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if !entry.path().is_dir() {
                            match load_blog_post(&entry.path()) {
                                Err(e) => {
                                    println!("error encountered when loading blog posts {e}");
                                }

                                Ok(blog) => {
                                    posts.push(blog);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("error with reading directory posts {e}");
                    }
                }
            }
        }
        Err(e) => println!("Error with posts directory {e}"),
    }

    posts
        .into_iter()
        .filter(|post| post.blog_data.visible)
        .collect()
}

static BLOGS: LazyLock<RwLock<Vec<BlogPost>>> = LazyLock::new(|| get_all_blog_posts().into());

pub async fn get_blogs() -> tokio::sync::RwLockReadGuard<'static, Vec<BlogPost>> {
    BLOGS.read().await
}

pub async fn refresh_blogs() {
    let mut blogs = BLOGS.write().await;
    *blogs = get_all_blog_posts();
}

use tokio::time::{self, Duration};
pub async fn blog_refresher(timeout_between_refreshes: Duration) {
    let mut interval = time::interval(timeout_between_refreshes);
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
    loop {
        interval.tick().await;
        refresh_blogs().await;
    }
}
