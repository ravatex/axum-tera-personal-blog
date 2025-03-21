use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version,about,long_about=None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a blog from a modified md file
   Add(AddingBlog),
    /// Edit a blog by selecting the blog and changing it with a new modified md file
    Edit(EditingBlog),
    /// Remove a blog by selecting it
    Remove(RemovingBlog),

    /// Get info from the blogs
    Read(GetBlogs),
}

#[derive(Args)]
pub struct AddingBlog {
    pub path: std::path::PathBuf,
}

#[derive(Args)]
pub struct EditingBlog {
    #[arg(short, long, value_name = "BLOG_ID")]
    pub blog_id: i32,

    #[arg(short, long, value_name = "FILE")]
    pub path: std::path::PathBuf,
}

#[derive(Args)]
pub struct RemovingBlog {
    #[arg(value_name = "BLOG_ID")]
    pub id: i32,
}

#[derive(Args)]
pub struct GetBlogs {
    #[arg(short, long)]
    pub all_info: bool,
    #[arg(long)]
    pub ids: bool,
    #[arg(long)]
    pub names: bool,
    #[arg(long)]
    pub dates: bool,

    #[command(flatten)]
    pub filters: Filters,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
pub struct Filters {
    #[arg(long, value_name = "TITLE")]
    pub filter_title: Option<String>,

    #[arg(long, value_name = "ID")]
    pub filter_id: Option<i32>,
}


