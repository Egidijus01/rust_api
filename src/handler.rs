use crate::model::{Author, Post};
use crate::response::*;
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
pub async fn get_all_authors(db: &SqlitePool) -> Result<impl Reply, Rejection> {
    let query: &str = "
        SELECT
            id,
            name,
            surname,
            created_at as created_at,
            updated_at as updated_at
        FROM authors";

    match sqlx::query_as(query).fetch_all(db).await {
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

//GET ALL POSTS
pub async fn get_all_posts(db: &SqlitePool) -> Result<impl Reply, Rejection> {
    let query: &str = "
        SELECT
            id,
            title,
            content,
            author_id,
            created_at as created_at,
            updated_at as updated_at
        FROM posts";

    match sqlx::query_as(query).fetch_all(db).await {
        Ok(posts) => {
            let length = posts.len();
            let response = PostResponse {
                status: "Success".to_string(),
                posts,
                results: length,
            };
            Ok(warp::reply::json(&response))
        }
        Err(e) => {
            eprintln!("Error fetching posts: {:?}", e);
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

//GET SINGLE POST
pub async fn get_post(db: &SqlitePool, id: i64) -> Result<impl Reply, Rejection>{

    let query = "
        SELECT id, title, content, author_id, created_at, updated_at
        FROM posts 
        WHERE id = ?
    ";

    match sqlx::query_as::<_, Post>(query)
        .bind(id)
        .fetch_optional(db)
        .await
    {
        Ok(post) => {
            if let Some(post) = post {
                Ok(warp::reply::json(&SingePostResponse {
                    status: "Success".to_string(),
                    data: post,
                }))
            } else {
                Err(warp::reject::not_found())
            }
        }
        Err(e) => {
            eprintln!("Error fetching post: {:?}", e);
            let error_response = ErrorResponse::new("Error fetching post".to_string());
            Err(warp::reject::custom(error_response))
        }
    }
}


//CREATE AN AUTHOR
pub async fn post_author(db: &SqlitePool, data:CreateAuthorRequest)-> Result<impl Reply, Rejection>{
    let name = &data.name;
    let surname = &data.surname;

    
    let query = "
        INSERT INTO authors (name, surname)
        VALUES (?, ?)
    ";


    let result = sqlx::query(query)
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


//CREATE A POST
pub async fn create_post(db: &SqlitePool, data:CreatePostRequest)-> Result<impl Reply, Rejection>{
   

    
    let query = "
        INSERT INTO posts (title, content, author_id)
        VALUES (?, ?, ?)
    ";


    sqlx::query(query)
        .bind(&data.title)
        .bind(&data.content)
        .bind(&data.author_id)
        .execute(db)
        .await;

    let post = Post {
        id: Default::default(),
        title: data.title.clone(),
        content: data.content.clone(),
        author_id: data.author_id.clone(),
        created_at: Default::default(),
        updated_at: Default::default(),
    };


    let response = warp::reply::json(&SingePostResponse {
        status: "Success".to_string(),
        data: post,
    });

    Ok(warp::reply::with_status(
        response,
        warp::http::StatusCode::CREATED,
    ))


}

//GET ALL POSTS BY AUTHOR ID
pub async fn get_posts_by_auth(db: &SqlitePool, author_id: i64) -> Result<impl Reply, Rejection>{

    let query = "
        SELECT id, title, content, author_id, created_at, updated_at
        FROM posts 
        WHERE author_id = ?
    ";

    match sqlx::query_as::<_, Post>(query)
        .bind(author_id)
        .fetch_all(db)
        .await
    {
        Ok(posts) => {
            let length = posts.len();
            let response = PostResponse {
                status: "Success".to_string(),
                posts,
                results: length,
            };
            Ok(warp::reply::json(&response))
        }
        Err(e) => {
            eprintln!("Error fetching post: {:?}", e);
            let error_response = ErrorResponse::new("Error fetching post".to_string());
            Err(warp::reject::custom(error_response))
        }
    }
}


//UPDATE POST BY ID
pub async fn update_post(db: &SqlitePool, data:UpdatePostRequest, post_id: i64)-> Result<impl Reply, Rejection>{
   

    
    let query = "
    UPDATE posts
    SET title = ?,
    content = ?,
    updated_at = ?
    WHERE id = ?
    ";

    let updated_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let title = data.title.clone();
    let content = data.content.clone();
    let updated_at_clone = updated_at.clone();



    sqlx::query(query)
    .bind(&title)
    .bind(&content)
    .bind(&updated_at)
    .bind(post_id)
    .execute(db)
    .await;


    let post = Post {
        id: Default::default(),
        title: title,
        content: content,
        author_id: Default::default(),
        created_at: Default::default(),
        updated_at: updated_at,
    };


    let response = warp::reply::json(&SingePostResponse {
        status: "Success".to_string(),
        data: post,
    });

    Ok(warp::reply::with_status(
        response,
        warp::http::StatusCode::CREATED,
    ))


}


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
    let updated_at_clone = updated_at.clone();



    sqlx::query(query)
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

    sqlx::query(query)
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


//DELTE POST
pub async fn delete_post(db: &SqlitePool, id: i64)-> Result<impl Reply, Rejection>{

    let query = "
        DELETE FROM posts
        where id = ?
    ";

    sqlx::query(query)
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