use diesel::prelude::*;
use std::env;

use super::models::Post;
use super::schema::posts::dsl::{posts, published};

fn establish_connection() -> PgConnection {
    let host = env::var("postgres_host").expect("postgres_host must be set");
    let port = env::var("postgres_port").expect("postgres_port must be set");
    let username = env::var("postgres_username").expect("postgres_username must be set");
    let password = env::var("postgres_password").expect("postgres_password must be set");
    let database = env::var("postgres_database").expect("postgres_database must be set");

    let database_url = format!("postgres://{username}:{password}@{host}:{port}/{database}");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn get_posts(connection: &mut PgConnection) {
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}

pub fn run() {
    let mut connection = establish_connection();
    get_posts(&mut connection);
}
