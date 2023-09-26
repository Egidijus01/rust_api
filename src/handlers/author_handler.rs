use crate::models::authors::{Author};
use crate::models::response::{SingeAuthorResponse, AuthorResponse, CreateAuthorRequest, UpdateAuthorRequest, StatusResponse, PageQueryParam};
use sqlx::SqlitePool;

use warp::{ Rejection, Reply};
use serde:: Serialize;


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
        SELECT id, name, surname, created_at, updated_at
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
pub async fn post_author(db: &SqlitePool, data:CreateAuthorRequest)-> Result<impl Reply, Rejection>{


    
    let query = "
        INSERT INTO authors (name, surname)
        VALUES (?, ?)
    ";


    _ = sqlx::query(query)
        .bind(&data.name)
        .bind(&data.surname)
        .execute(db)
        .await;

    let author = Author {
        id: Default::default(),
        name: data.name.clone(),
        surname: data.surname.clone(),
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
pub async fn update_author(db: &SqlitePool, data:UpdateAuthorRequest, author_id: i64)-> Result<impl Reply, Rejection>{
   

    
    let query = "
    UPDATE authors
    SET name = ?,
    surname = ?,
    updated_at = ?
    WHERE id = ?
    ";

    let updated_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let name = data.name.clone();
    let surname = data.surname.clone();




    _ =sqlx::query(query)
        .bind(&name)
        .bind(&surname)
        .bind(&updated_at)
        .bind(author_id)
        .execute(db)
        .await;


    let author = Author {
        id: Default::default(),
        name: name,
        surname: surname,
        created_at: Default::default(),
        updated_at: updated_at,
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

