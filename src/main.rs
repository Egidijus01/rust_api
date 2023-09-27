
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use warp::{http::Method, Filter};
mod Middleware;
mod db;
mod routes;
mod models;
mod handlers;
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




    
    // Apply migrations
    apply_migrations(&db).await;


    

    // let routes = all_authors;
   
    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origins(vec!["http://localhost:3000/", "http://localhost:8000/"])
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);

      
        // let routes = log_content_type()
        // .and(database::routes(&db).with(cors)).boxed();
  
        let routes = check_content_type()
        .and(database::routes(&db).with(cors))
        .boxed();
   


    warp::serve(routes)
        .run(([127, 0, 0, 1], 8000)) 
        .await;
}


