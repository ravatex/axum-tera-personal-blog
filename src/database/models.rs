use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: i32,
    pub name: String,
    pub date: chrono::NaiveDate,
    pub message: String,
    pub published: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::inquiries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Inquiry {
    pub id: i32,
    pub name: String,
    pub message: String,
    pub email: String,
    pub business: bool,
}
