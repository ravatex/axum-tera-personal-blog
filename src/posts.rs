use comrak::markdown_to_html;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

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
