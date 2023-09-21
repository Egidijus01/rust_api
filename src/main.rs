
use chrono::{NaiveDateTime, ParseError};
use serde::de::Error;
use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool};
use chrono::{Utc };

const DB_URL: &str = "sqlite://sqlite.db";



#[derive(Clone,FromRow, Debug)]
struct Author {
    id: i64,
    name: String,
    surname: String,
    created_at: String,
    updated_at: String,
}





#[derive(Clone, FromRow, Debug)]
struct Posts {
    id: i64,
    title: String,
    content: String,
    author_id: i64,
    created_at: String,
    updated_at: String,
}





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

fn parse_datetime(s: &str) -> Result<NaiveDateTime, ParseError> {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
}


async fn print_all_authors(db: &SqlitePool) {
    let query = "
    SELECT
        id,
        name,
        surname,
        created_at as created_at,
        updated_at as updated_at
    FROM authors";

    let authors: Vec<Author> = sqlx::query_as(query)
        .fetch_all(db)
        .await
        .expect("Error fetching authors");

    for author in &authors {
        let created_at = parse_datetime(&author.created_at).expect("Error parsing created_at");
        let updated_at = parse_datetime(&author.updated_at).expect("Error parsing updated_at");

        println!(
            "ID: {}, Name: {}, Surname: {}, Created At: {}, Updated At: {}",
            author.id,
            author.name,
            author.surname,
            created_at,
            updated_at
        );
    }
}

async fn update_author(db:&SqlitePool, author_id: i64, author: Author) -> Result<(), sqlx::Error>{
    let query = "
        UPDATE authors
        SET name = ?,
        surname = ?,
        updated_at = ?
        WHERE id = ?
    ";


    let updated_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();



    sqlx::query(query)
        .bind(author.name)
        .bind(author.surname)
        .bind(&updated_at)
        .bind(author_id)
        .execute(db)
        .await?;
   
    
    Ok(())
}


async fn get_user_by_id(db: &SqlitePool, author_id: i64) -> Result<Option<Author>, sqlx::Error> {

    let query = "
        SELECT id, name, surname, created_at, updated_at
        FROM authors 
        WHERE id = ?
    ";

    sqlx::query_as::<_, Author>(query)
        .bind(author_id)
        .fetch_optional(db)
        .await



}

async fn delete_author_by_id(db: &SqlitePool, author_id: i64) -> Result<(), sqlx::Error>{

    let query: &str = "
    DELETE FROM authors
    where id = ?
    ";


    sqlx::query(query)
        .bind(author_id)
        .execute(db)
        .await?;


    Ok(())

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
    
    apply_migrations(&db).await;
    



    let author_id_to_update = 1;

    let res: Result<Option<Author>, sqlx::Error> = get_user_by_id(&db, author_id_to_update)
    .await
    .map_err(|err| {
        eprintln!("Error fetching author: {:?}", err);
        err
    });


    match res{
        Ok(Some(res)) => {
        let updated_author = Author {
                    id: res.id,
                    name: "New Bobis".to_string(),  // Replace with the new name
                    surname: "New Shmurdas".to_string(),  // Replace with the new surname
                    created_at: res.created_at,  // Keep the original created_at value
                    updated_at: res.updated_at,  // Updated by your update_author function
                };
                match update_author(&db, author_id_to_update, updated_author).await {
                    Ok(_) => println!("Author updated successfully"),
                    Err(err) => eprintln!("Error updating author: {:?}", err),
                }

        }

        
     
    Ok(None) => println!("Author not found"),
        Err(err) => eprintln!("Error fetching author: {:?}", err),
    }
    

    delete_author_by_id(&db, 3).await;


    print_all_authors(&db).await;

    // match res {
    //     Ok(author_option) => {
    //         match author_option {1
    //             Some(author) => {
    //                 // Print the user details if found
    //                 println!(
    //                     "User found - ID: {}, Name: {}, Surname: {}",
    //                     author.id, author.name, author.surname
    //                 );
    //             }
    //             None => {
    //                 println!("User not found.");
    //             }
    //         }
    //     }
    //     Err(error) => {
    //         println!("Error: {:?}", error);
    //     }
    // }
}

    // let authors = sqlx::query_as::<_, Author>("SELECT * FROM authors")
    //     .fetch_all(&db)
    //     .await
    //     .map_err(|e| {
    //         eprintln!("Error fetching authors: {}", e);
    //         e
    //     });

    //     for author in authors.unwrap_or_else(|e| {
    //         eprintln!("Error fetching authors: {}", e);
    //         vec![]
    //     }) {
    //         println!(
    //             "ID: {}, Name: {}, Surname: {}, Created At: {}, Updated At: {}",
    //             author.id,
    //             author.name,
    //             author.surname,
    //             author.created_at,
    //             author.updated_at,
    //         );Result<(), sqlx::Error>TO authors (name, surname) VALUES (?,?)")
//         .bind("bobby")
//         .bind("smurdas")
//         .execute(&db)
//         .await
//         .unwrap();
//     println!("Query result: {:?}", result);
// }










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