use std::fs::{File, self};
use std::{io};


use base64::decode;
use futures_util::StreamExt;

use crate::models::authors::Author;
use crate::models::response::{SingeAuthorResponse, AuthorResponse, CreateAuthorRequest, UpdateAuthorRequest, StatusResponse, PageQueryParam};
use sqlx::SqlitePool;
use uuid::Uuid;
use warp::reject::{Reject, reject, self};
use crate::Middleware::mime_check::{check_image_format, check_image_size};
use warp::{ Rejection, Reply};
use serde:: Serialize;
use bytes::{BufMut, Buf};
use futures::TryStreamExt;


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
pub async fn delete_author(db: &SqlitePool, id: i64)-> Result<impl Reply, Rejection>{

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

    Ok(warp::reply::with_status(
        response,
        warp::http::StatusCode::CREATED,
    ))
}





// GET FILE WITH FORMDATA
// pub async fn uploadas(form: FormData) -> Result<impl Reply, Rejection> {
//     let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
//         eprintln!("form error: {}", e);
//         warp::reject::reject()
//     })?;

//     for p in parts {
//         if p.name() == "file" {
//             let content_type = p.content_type();
//             let file_ending;
//             match content_type {
//                 Some(file_type) => match file_type {
//                     "application/pdf" => {
//                         file_ending = "pdf";
//                     }
//                     "image/png" => {
//                         file_ending = "png";
//                     }
//                     v => {
//                         eprintln!("invalid file type found: {}", v);
//                         return Err(warp::reject::reject());
//                     }
//                 },
//                 None => {
//                     eprintln!("file type could not be determined");
//                     return Err(warp::reject::reject());
//                 }
//             }

//             let value = p
//                 .stream()
//                 .try_fold(Vec::new(), |mut vec, data| {
//                     vec.put(data);
//                     async move { Ok(vec) }
//                 })
//                 .await
//                 .map_err(|e| {
//                     eprintln!("reading file error: {}", e);
//                     warp::reject::reject()
//                 })?;

//             let file_name = format!("./files/{}.{}", Uuid::new_v4().to_string(), file_ending);
//             tokio::fs::write(&file_name, value).await.map_err(|e| {
//                 eprint!("error writing file: {}", e);
//                 warp::reject::reject()
//             })?;
//             println!("created file: {}", file_name);
//         }
//     }

//     Ok("success")
// }

// use tokio::task;
// pub async fn upload(form: warp::multipart::FormData) -> Result<impl Reply, Rejection> {
//     task::spawn(async move {
//         let mut parts = form.into_stream();
//         println!("{:?}", parts);

//         while let Ok(p) = parts.next().await.unwrap() {
            


//             let filename = p.filename().unwrap_or("photo.png");
//             let filepath = format!("uploads/{}", filename);
//             println!("{}", filename.to_string());
//             println!("{}", filepath.to_string());


//             fs::create_dir_all("uploads").unwrap();

//             save_part_to_file(&filepath, p).await.expect("save error");
//         }
//     });

//     Ok("Upload successful!")
// }

// async fn save_part_to_file(path: &str, part: warp::multipart::Part) -> Result<(), std::io::Error> {
//     let data = part
//         .stream()
//         .try_fold(Vec::new(), |mut acc, buf| async move {
//             acc.extend_from_slice(buf.chunk());
//             Ok(acc)
//         })
//         .await.expect("folding error");
//     std::fs::write(path, data)
// }