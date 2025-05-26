use sqlx::{ postgres::PgPoolOptions, PgPool };
use dotenv::dotenv;
use std::{ env };

use crate::models::{BlogPost, NewBlogPost};
// Create db connection
pub async fn open_connection() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();

    let conn_string = env::var("CONNECTION_STRING").expect("Connection string");

    PgPoolOptions::new().max_connections(5).connect(&conn_string).await
}

pub  async  fn create_post(post: &NewBlogPost)->Result<BlogPost, sqlx::Error>{
    todo!()
}