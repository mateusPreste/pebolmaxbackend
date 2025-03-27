mod controller;
mod handler;
mod middleware;
mod model;
mod modules;
mod route;
mod schema;

use std::sync::Arc;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use dotenv::dotenv;
use modules::auth::auth_service::AuthService;
use route::create_router;
use tower_http::cors::CorsLayer;

use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::{Client, Error, NoTls};

pub struct AppState {
    db: Client,
    auth: AuthService,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    // jwt_secret is used to sign the JWT token
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let auth_service = AuthService::new(jwt_secret);

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the database with tokio_postgres
    let (client, connection) = match tokio_postgres::connect(&database_url, NoTls).await {
        Ok((client, connection)) => {
            println!("✅Connection to the database is successful!");
            (client, connection)
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState {
        db: client,
        auth: auth_service,
    }))
    .layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
