use crate::models::posts::Post;
use crate::models::response::{SingePostResponse, PostResponse, CreatePostRequest, UpdatePostRequest, StatusResponse};
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