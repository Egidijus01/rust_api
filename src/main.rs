
use std::{convert::Infallible, sync::Arc};
use crate::models::posts::Post;
use crate::models::authors::Author;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use utoipa::OpenApi;
use utoipa_swagger_ui::Config;
use ws::clients::*;

use swagger::{ApiDoc, serve_swagger};
use warp::{http::Method, Filter};
mod Middleware;
mod db;
mod routes;
mod models;
mod handlers;
mod ws;
mod swagger;
use crate::Middleware::mime_check::check_content_type;
use crate::db::database;

const DB_URL: &str = "sqlite://sqlite.db";

async fn apply_migrations(db: &SqlitePool){
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");
    let migration_results = sqlx::migrate::Migrator::new(migrations)
    .await
    .unwrap()
    .run(db)
    .await;
match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    println!("migration: {:?}", migration_results);
    
}


#[tokio::main]
async fn main() {
    // Initialize your database connection pool
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let db_url = "sqlite://sqlite.db";
    let db = SqlitePool::connect(db_url)
        .await
        .expect("Failed to connect to the database");

    let clients: Clients = create_clients();
    
  
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(ws::ws_handler::ws_handler);
    
    // // Apply migrations
    apply_migrations(&db).await;

    
    
   
    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origins(vec!["http://localhost:3000", "http://localhost:8000"])
        .allow_headers(vec!["*"])
        .allow_credentials(true);

        
        let openapi = ApiDoc::openapi();
        
        let config = Arc::new(Config::from("/api-doc.json"));


        // Serve Swagger UI
        let api_doc = warp::path("api-doc.json")
        .and(warp::get())
        .map(move || warp::reply::json(&openapi));

        let swagger_ui = warp::path("swagger-ui")
            .and(warp::get())
            .and(warp::path::full())
            .and(warp::path::tail())
            .and(warp::any().map(move || config.clone()))
            .and_then(serve_swagger);

        
        

        let routes = check_content_type()
            .and(database::routes(&db, clients).with(&cors))
            .boxed()
            .or(ws_route.with(warp::cors().allow_any_origin()));


 
    warp::serve(routes.or(api_doc.with(cors.clone())).or(swagger_ui.with(cors.clone())))
        .run(([127, 0, 0, 1], 8000)) 
        .await;
}


fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
