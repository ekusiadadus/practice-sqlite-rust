#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::NewPost;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = "sample.db";
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &SqliteConnection, title: &str, body: &str) -> usize {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        //SQLiteはget_result()は対応していないため、execute()
        .execute(conn)
        .expect("Error saving new post")
}
