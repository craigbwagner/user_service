use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, query};
use std::env;

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
    password: String,
}

async fn create_user(pool: web::Data<Pool<Postgres>>, user: web::Json<User>) -> impl Responder {
    let result = query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING id",
        user.username,
        user.email,
        user.password
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().body(format!("User created with ID: {}", record.id)),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
    }
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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/create_user", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
