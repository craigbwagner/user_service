use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::env;

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
    password: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); //loads environment variables

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    //create a connection pool
    let pool = Pool::<Postgres>::connect(&database_url)
        .await
        .expect("Failed to create database connection pool");

    println!("Connected to PostgreSQL");

    HttpServer::new(|| App::new().route("/health", web::get().to(health_check)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("User service is running")
}
