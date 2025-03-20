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
    Add(AddingBlog),
    Edit(EditingBlog),
    Remove(RemovingBlog),
}

#[derive(Args)]
pub struct AddingBlog {
    pub path: std::path::PathBuf,
}

#[derive(Args)]
pub struct EditingBlog {
    #[arg(short, long, value_name = "ID")]
    pub blog_id: i32,

    #[arg(short, long, value_name = "FILE")]
    pub path: std::path::PathBuf,
}

#[derive(Args)]
pub struct RemovingBlog {
    #[arg(value_name = "ID")]
    pub id: i32,
}
