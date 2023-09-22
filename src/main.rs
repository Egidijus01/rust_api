mod handler;
mod model;
mod response;


use sqlx::SqlitePool;

use warp::{http::Method, Filter};





pub mod routes;



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

    let db_url = "sqlite://sqlite.db";
    let db = SqlitePool::connect(db_url)
        .await
        .expect("Failed to connect to the database");




    
    // Apply migrations
    apply_migrations(&db).await;

    
    let db_clone = db.clone();

    


    

 


    // let routes = all_authors;
   
    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origins(vec!["http://localhost:3000/", "http://localhost:8000/"])
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);


    let routes = routes::routes(&db_clone).with(cors);
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8000)) 
        .await;
}


