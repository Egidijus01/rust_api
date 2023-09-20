
use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool};
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
const DB_URL: &str = "sqlite://sqlite.db";



#[derive(Clone, Debug)]
struct Author {
    id: i64,
    name: String,
    surname: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl<'r, Row: sqlx::Row> sqlx::FromRow<'r, Row> for Author {
    fn from_row(row: &'r Row) -> Result<Self, sqlx::Error> {
        Ok(Author {
            id: row.try_get("id")?,
            name: row.try_get_raw("name")?,
            surname: row.try_get_raw("surname")?.as_str().to_string(),
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}



#[derive(Clone, FromRow, Debug)]
struct Posts {
    id: i64,
    title: String,
    content: String,
    author_id: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");

    let migration_results = sqlx::migrate::Migrator::new(migrations)
    .await
    .unwrap()
    .run(&db)
    .await;
    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    println!("migration: {:?}", migration_results);
    



    let authors = sqlx::query_as::<_, Author>("SELECT * FROM authors")
        .fetch_all(&db)
        .await
        .map_err(|e| {
            eprintln!("Error fetching authors: {}", e);
            e
        });

        for author in authors.unwrap_or_else(|e| {
            eprintln!("Error fetching authors: {}", e);
            vec![]
        }) {
            println!(
                "ID: {}, Name: {}, Surname: {}, Created At: {}, Updated At: {}",
                author.id,
                author.name,
                author.surname,
                author.created_at,
                author.updated_at,
            );
        }
        
        
}

    //create
    // let result = sqlx::query("INSERT INTO authors (name, surname) VALUES (?,?)")
    //     .bind("bobby")
    //     .bind("smurda")
    //     .execute(&db)
    //     .await
    //     .unwrap();
    // println!("Query result: {:?}", result);






    // let result = sqlx::query("INSERT INTO users (name) VALUES (?)")
    //     .bind("bobby")
    //     .execute(&db)
    //     .await
    //     .unwrap();
    // println!("Query result: {:?}", result);


    // let user_results = sqlx::query_as::<_, User>("SELECT id, name FROM users")
    //     .fetch_all(&db)
    //     .await
    //     .unwrap();
    // for user in user_results {
    //     println!("[{}] name: {}", user.id, &user.name);
    // }


    // let delete_result = sqlx::query("DELETE FROM users WHERE name=$1")
    //     .bind("bobby")
    //     .execute(&db)
    //     .await
    //     .unwrap();
    // println!("Delete result: {:?}", delete_result);