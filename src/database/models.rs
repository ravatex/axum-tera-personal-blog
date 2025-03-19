use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
pub struct Post {
    pub id: i32,
    pub name: String,
    pub date: chrono::NaiveDate,
    pub message: String,
    pub published: bool,
    pub thumbnail: Option<String>,
}


impl crate::html_insertion::IntoBlog for Post {
    fn blog_name(&self) -> String {
        self.name.clone()
    }

    fn blog_date(&self) -> String {
        self.date.format("%Y-%m-%d").to_string()
    }

    fn blog_message(&self) -> String {
        self.message.clone()
    }

    fn blog_thumbnail(&self) -> Option<String> {
        self.thumbnail.clone()
    }

    fn blog_id(&self) -> String {
        format!("{}",self.id)
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::inquiries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
pub struct Inquiry {
    pub id: i32,
    pub name: String,
    pub message: String,
    pub email: String,
    pub business: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::thumbnails)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
pub struct ImageThumbnail {
    pub filepath: String,
    pub alt_text: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::comments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
pub struct Comment {
    pub id: i32,
    pub post_id: i32,
    pub name: String,
    pub date: chrono::NaiveDate,
    pub contents: String,
}
