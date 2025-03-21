use comrak::markdown_to_html;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug)]
#[allow(dead_code)]
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

pub struct SerdeNaiveDate(pub chrono::NaiveDate);

impl Serialize for SerdeNaiveDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.format("%Y-%m-%d").to_string())
    }
}

impl<'de> Deserialize<'de> for SerdeNaiveDate {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let date_err = chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(|e| {
            serde::de::Error::custom(format!(
                "Error parsing date, format needed %Y-%m-%d;\n got {s};\n Message: {e:?}"
            ))
        })?;

        Ok(SerdeNaiveDate(date_err))
    }
}

#[derive(Serialize)]
pub struct BlogPost {
    pub path: String,
    pub contents: String,
    pub blog_data: BlogData,
}

#[derive(Serialize, Deserialize)]
pub struct BlogData {
    pub title: String,
    pub date: SerdeNaiveDate,
    pub visible: bool,
    pub thumbnail: Option<String>,
}

use crate::database::models::Post;
impl From<Post> for BlogPost {
    fn from(value: Post) -> Self {
        let blog_data = BlogData {
            title: value.name,
            date: SerdeNaiveDate(value.date),
            thumbnail: value.thumbnail,
            visible: value.published,
        };

        BlogPost {
            path: format!("{}", value.id),
            contents: value.message,
            blog_data,
        }
    }
}

use crate::html_insertion::IntoBlog;
impl IntoBlog for BlogPost {
    fn blog_name(&self) -> String {
        self.blog_data.title.clone()
    }

    fn blog_date(&self) -> String {
        self.blog_data.date.0.format("%Y-%m-%d").to_string()
    }

    fn blog_message(&self) -> String {
        self.contents.clone()
    }

    fn blog_thumbnail(&self) -> Option<String> {
        self.blog_data.thumbnail.clone()
    }

    fn blog_id(&self) -> String {
        format!("{}", self.path)
    }
}

pub fn load_blog_post(path: &Path) -> Result<BlogPost, Box<dyn std::error::Error>> {
    let blog_post = fs::read_to_string(path)?;

    let lines: Vec<&str> = blog_post.splitn(3, "---").collect();

    if lines.len() < 2 {
        return Err("No blog data (---)".into());
    }

    let blog_data: BlogData = serde_json::from_str(lines[1])?;

    println!("Is ok! : {path:?}");

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


#[allow(dead_code)]
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





