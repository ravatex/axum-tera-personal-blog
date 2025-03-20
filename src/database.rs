#[allow(dead_code)]
pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("{database_url}");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::*;

pub fn cool_stuff() {
    use schema::posts::dsl::*;
    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");
    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.name);
        println!("-----------\n");
        println!("{}", post.message);
    }
}

pub mod inquiries {

    use super::establish_connection;
    use super::models::Inquiry;
    use super::schema::inquiries::dsl::*;
    use diesel::prelude::*;

    #[derive(Insertable)]
    #[diesel(table_name = crate::database::schema::inquiries)]
    pub struct NewInquiry {
        pub name: String,
        pub message: String,
        pub email: String,
        pub business: bool,
    }

    use crate::request::Message;
    impl From<Message> for NewInquiry {
        fn from(value: Message) -> Self {
            NewInquiry {
                name: value.name,
                message: value.message,
                email: value.email,
                business: value.is_business,
            }
        }
    }

    pub fn get_business_inquiries_by_company() -> Result<Vec<Inquiry>, diesel::result::Error> {
        let connection = &mut super::establish_connection();

        let results = inquiries
            .filter(business.eq(true))
            .order_by(name.asc())
            .select(Inquiry::as_select())
            .load(connection);

        results
    }

    pub fn insert_inquiry(inquiry: impl Into<NewInquiry>) -> Result<(), diesel::result::Error> {
        let connection = &mut establish_connection();

        let new_inquiry: NewInquiry = inquiry.into();

        diesel::insert_into(inquiries)
            .values(&new_inquiry)
            .execute(connection)
            .map(|_| ())
    }
}

pub mod blog_posts {
    use super::schema::posts::dsl::*;
    use super::*;

    #[derive(Insertable, AsChangeset)]
    #[diesel(table_name = crate::database::schema::posts)]
    pub struct NewPost {
        pub name: String,
        pub date: chrono::NaiveDate,
        pub message: String,
        pub published: bool,
        pub thumbnail: Option<String>,
    }

    impl From<crate::posts::BlogPost> for NewPost {
        fn from(value: crate::posts::BlogPost) -> Self {
            NewPost {
                name: value.blog_data.title,
                date: value.blog_data.date.0,
                message: value.contents,
                published: value.blog_data.visible,
                thumbnail: value.blog_data.thumbnail,
            }
        }
    }

    pub fn insert_blog_post(blog: impl Into<NewPost>) -> Result<(), diesel::result::Error> {
        let connection = &mut establish_connection();

        let new_blog: NewPost = blog.into();

        diesel::insert_into(posts)
            .values(&new_blog)
            .execute(connection)
            .map(|_| ())
    }

    pub fn get_blog_posts() -> Result<Vec<Post>, diesel::result::Error> {
        let connection = &mut establish_connection();

        posts
            .filter(published.eq(true))
            .select(Post::as_select())
            .load(connection)
    }

    pub fn get_blog_post_from_id(id_: i32) -> Option<Post> {
        let connection = &mut establish_connection();

        posts.find(id_).first(connection).ok()
    }

    pub fn insert_with_no_duplicate_names(blog: impl Into<NewPost>) {
        let connection = &mut establish_connection();
        println!("Gooba");

        let blog = blog.into();
        if posts
            .filter(name.eq(&blog.name))
            .select(diesel::dsl::count_star())
            .first(connection)
            .unwrap_or(0)
            == 0
        {
            println!("Adding");

            diesel::insert_into(posts).values(&blog).execute(connection);
        }
    }

    pub fn edit_blog_post(
        id_: i32,
        post: impl Into<NewPost>,
    ) -> Result<usize, diesel::result::Error> {
        let connection = &mut establish_connection();
        diesel::update(posts.find(id_))
            .set(post.into())
            .execute(connection)
    }

    pub fn remove_blog_post(id_: i32) -> Result<usize, diesel::result::Error> {
        
        let connection = &mut establish_connection();
        diesel::delete(posts.find(id_)).execute(connection)
    }
}
