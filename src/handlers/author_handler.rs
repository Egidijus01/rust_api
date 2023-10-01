use std::fs::{File, self};
use std::io;
use crate::ws::ws_handler::send_message_to_clients;
use crate::ws::clients::Clients;
use base64::decode;
use futures_util::StreamExt;

use crate::models::authors::Author;
use crate::models::response::{SingeAuthorResponse, AuthorResponse, CreateAuthorRequest, UpdateAuthorRequest, StatusResponse, PageQueryParam};
use sqlx::SqlitePool;

use warp::reject::Reject;
use crate::Middleware::mime_check::{check_image_format, check_image_size};
use warp::{ Rejection, Reply};
use serde:: Serialize;


// use worker::{FormData, FormEntry};
use warp::multipart::{FormData, Part};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
    pub fn new(message: String) -> Self {
        ErrorResponse { message }
    }
}

impl warp::reject::Reject for ErrorResponse {}

#[derive(Debug)]
pub enum FileError {
    Io(io::Error),
}

impl warp::reject::Reject for FileError {}

impl From<io::Error> for FileError {
    fn from(error: io::Error) -> Self {
        FileError::Io(error)
    }
}

#[derive(Debug)]
struct MyError(sqlx::Error);

impl Reject for MyError {}




//GET ALL AUTHORS
pub async fn get_all_authors(params: PageQueryParam, search_param: Option<String>, db: &SqlitePool) -> Result<impl Reply, Rejection> {
    let page = params.page.unwrap_or(1);
    let items_per_page = 10;
    let offset = (page - 1) * items_per_page;

    // Build the SQL query based on search criteria
    let mut query = "
        SELECT
            id,
            name,
            surname,
            photo,
            created_at,
            updated_at
        FROM authors
    ".to_owned();



    //IF PARAM IS NOT EMPTY
    if let Some(search) = search_param {


        query.push_str(&format!(" WHERE name LIKE '{}' OR surname LIKE '{}'", search, search));
    }

    query.push_str(" LIMIT (?) OFFSET (?)");


    match sqlx::query_as(&query)
    .bind(items_per_page)
    .bind(offset)
    .fetch_all(db).await {
        Ok(authors) => {
            let length = authors.len();
            let response = AuthorResponse {
                status: "Success".to_string(),
                authors,
                results: length,
            };
            Ok(warp::reply::json(&response))
        }
        Err(e) => {
            eprintln!("Error fetching authors: {:?}", e);
            Err(warp::reject::custom(ErrorResponse::new(e.to_string())))
        }
    }
}

//GET SINGLE AUTHOR
pub async fn get_author(db: &SqlitePool, id: i64) -> Result<impl Reply, Rejection>{

    let query = "
        SELECT id, name, surname, photo, created_at, updated_at
        FROM authors 
        WHERE id = ?
    ";

    match sqlx::query_as::<_, Author>(query)
        .bind(id)
        .fetch_optional(db)
        .await
    {
        Ok(author) => {
            if let Some(author) = author {
                Ok(warp::reply::json(&SingeAuthorResponse {
                    status: "Success".to_string(),
                    data: author,
                }))
            } else {
                Err(warp::reject::not_found())
            }
        }
        Err(e) => {
            eprintln!("Error fetching author: {:?}", e);
            let error_response = ErrorResponse::new("Error fetching author".to_string());
            Err(warp::reject::custom(error_response))
        }
    }
}




//CREATE AN AUTHOR
pub async fn post_author(
    db: &SqlitePool,
    data: CreateAuthorRequest,
    clients:Clients
) -> Result<impl Reply, Rejection> {
    // Decode the base64-encoded photo data
    let photo_data = match &data.photo {
        Some(photo) => {
            let decoded = decode(photo);
            match decoded {
                Ok(decoded) => Some(decoded),
                Err(_) => {
                    return Ok(warp::reply::with_status(
                        warp::reply::json(&ErrorResponse::new("Bad Request".to_string())),
                        warp::http::StatusCode::BAD_REQUEST,
                    ));
                }
            }
        }
        None => None,
    };
    
    if let Some(photo_data) = &photo_data {
        if check_image_format(photo_data).is_none() || check_image_size(photo_data).is_none() {
            return Ok(warp::reply::with_status(
                warp::reply::json(&ErrorResponse::new("Bad Request".to_string())),
                warp::http::StatusCode::BAD_REQUEST,
            ));
        }
    }
    let query = "
        INSERT INTO authors (name, surname, photo)
        VALUES (?, ?, ?)
    ";

    // Insert author data into the database
    if let Err(err) = sqlx::query(query)
        .bind(&data.name)
        .bind(&data.surname)
        .bind(photo_data.as_ref().map(|s| String::from_utf8_lossy(s).to_string())) // Convert Vec<u8> to String
        .execute(db)
        .await
    {
        return Err(warp::reject::custom(MyError(err)));
    }
    send_message_to_clients("Author has been created".to_string(), &clients).await;
    let author = Author {
        id: Default::default(),
        name: data.name.clone(),
        surname: data.surname.clone(),
        photo: photo_data.clone(),
        created_at: Default::default(),
        updated_at: Default::default(),
    };



    let response = warp::reply::json(&SingeAuthorResponse {
        status: "Success".to_string(),
        data: author,
    });

    Ok(warp::reply::with_status(
        response,
        warp::http::StatusCode::CREATED,
    ))
} 

//UPDATE AUTHOR
pub async fn update_author(
    db: &SqlitePool,
    data: UpdateAuthorRequest,
    author_id: i64,
    clients:Clients
) -> Result<impl Reply, Rejection> {

    let photo_data = match &data.photo {
        Some(photo) => {
            let decoded = decode(photo);
            match decoded {
                Ok(decoded) => Some(decoded),
                Err(_) => {
                    return Ok(warp::reply::with_status(
                        warp::reply::json(&ErrorResponse::new("Bad Request".to_string())),
                        warp::http::StatusCode::BAD_REQUEST,
                    ));
                }
            }
        }
        None => None,
    };
    
    if let Some(photo_data) = &photo_data {
        if check_image_format(photo_data).is_none() || check_image_size(photo_data).is_none() {
            return Ok(warp::reply::with_status(
                warp::reply::json(&ErrorResponse::new("Bad Request".to_string())),
                warp::http::StatusCode::BAD_REQUEST,
            ));
        }
    }


    let query = "
    UPDATE authors
    SET name = ?,
    surname = ?,
    photo = ?,
    updated_at = ?
    WHERE id = ?
    ";

    let updated_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    

    _ = sqlx::query(query)
        .bind(&data.name)
        .bind(&data.surname)
        .bind(&photo_data) // Use default if photo is None
        .bind(&updated_at)
        .bind(author_id)
        .execute(db)
        .await;

    let author = Author {
        id: Default::default(),
        name: data.name.clone(),
        surname: data.surname.clone(),
        photo: photo_data.clone(),
        created_at: Default::default(),
        updated_at: updated_at,
    };
    send_message_to_clients("Author has been updated".to_string(), &clients).await;

    let response = warp::reply::json(&SingeAuthorResponse {
        status: "Success".to_string(),
        data: author,
    });

    Ok(warp::reply::with_status(
        response,
        warp::http::StatusCode::ACCEPTED,
    ))
}


//DELTE AUTHOR
pub async fn delete_author(db: &SqlitePool, id: i64, clients:Clients)-> Result<impl Reply, Rejection>{

    let query = "
        DELETE FROM authors
        where id = ?
    ";

    _ = sqlx::query(query)
    .bind(id)
    .execute(db)
    .await;

    let response = warp::reply::json(&StatusResponse {
        status: "Success".to_string(),

    });
    send_message_to_clients("Author has been deleted".to_string(), &clients).await;

    Ok(warp::reply::with_status(
        response,
        warp::http::StatusCode::CREATED,
    ))
}
