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

pub fn get_business_inquiries_by_company() -> Result<Vec<Inquiry>, diesel::result::Error> {
    use schema::inquiries::dsl::*;

    let connection = &mut establish_connection();

    let results = inquiries
        .filter(business.eq(true))
        .order_by(name.asc())
        .select(Inquiry::as_select())
        .load(connection);

    results
}
