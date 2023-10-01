use crate::ws::clients::Clients;
use crate::ws::ws_handler::send_message_to_clients;
use crate::Middleware::mime_check::{check_content_type, check_file_size};
use crate::models::posts::Post;
use crate::models::response::{SingePostResponse, PostResponse, CreatePostRequest, UpdatePostRequest, StatusResponse, PageQueryParam, FileResponse};

use sqlx::{SqlitePool, Row};
use base64::{decode, encode};
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
pub async fn get_all_posts(params: PageQueryParam, search_param: Option<String>, db: &SqlitePool) -> Result<impl Reply, Rejection> {
    let page = params.page.unwrap_or(1);
    let items_per_page = 10;
    let offset = (page-1) * items_per_page;

    let mut query = "
        SELECT
            *
        FROM posts
        
        ".to_owned();

    if let Some(search) = search_param{
        query.push_str(&format!(" WHERE name LIKE '{}' OR surname LIKE '{}'", search, search));
    }

    query.push_str(" LIMIT (?) OFFSET (?)");

    match sqlx::query_as(&query)
    .bind(items_per_page)
    .bind(offset)
    .fetch_all(db).await {
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
        SELECT id, title, content, author_id, uploaded_file, created_at, updated_at
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
pub async fn create_post(db: &SqlitePool, data:CreatePostRequest, clients: Clients)-> Result<impl Reply, Rejection>{
    let file_data = match &data.uploaded_file {
        Some(post) => {
            let decoded = decode(post);
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

    if let Some(file_data) = &file_data {
        if check_file_size(file_data).is_none() || check_file_size(file_data).is_none() {
            return Ok(warp::reply::with_status(
                warp::reply::json(&ErrorResponse::new("Bad Request".to_string())),
                warp::http::StatusCode::BAD_REQUEST,
            ));
        }
    }


    
    let query = "
        INSERT INTO posts (title, content, author_id, uploaded_file)
        VALUES (?, ?, ?, ?)
    ";


    _ = sqlx::query(query)
        .bind(&data.title)
        .bind(&data.content)
        .bind(&data.author_id)
        .bind(&file_data)
        .execute(db)
        .await;

    let post = Post {
        id: Default::default(),
        title: data.title.clone(),
        content: data.content.clone(),
        author_id: data.author_id.clone(),
        uploaded_file: file_data.clone(),
        created_at: Default::default(),
        updated_at: Default::default(),
    };

    send_message_to_clients("Post has been created".to_string(), &clients).await;

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
pub async fn update_post(db: &SqlitePool, data:UpdatePostRequest, post_id: i64, clients: Clients)-> Result<impl Reply, Rejection>{
   
    let file_data = match &data.uploaded_file {
        Some(post) => {
            let decoded = decode(post);
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

    if let Some(file_data) = &file_data {
        if check_file_size(file_data).is_none() || check_file_size(file_data).is_none() {
            return Ok(warp::reply::with_status(
                warp::reply::json(&ErrorResponse::new("Bad Request".to_string())),
                warp::http::StatusCode::BAD_REQUEST,
            ));
        }
    }

    
    let query = "
    UPDATE posts
    SET title = ?,
    content = ?,
    uploaded_file = ?
    updated_at = ?
    WHERE id = ?
    ";

    let updated_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    

    _ = sqlx::query(query)
        .bind(&data.title)
        .bind(&data.content)
        .bind(&file_data)
        .bind(&updated_at)
        .bind(post_id)
        .execute(db)
        .await;


    let post = Post {
        id: Default::default(),
        title: data.title.clone(),
        content: data.content.clone(),
        author_id: Default::default(),
        uploaded_file: file_data.clone(),
        created_at: Default::default(),
        updated_at: updated_at,
    };

    send_message_to_clients("Post has been updated".to_string(), &clients).await;

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
pub async fn delete_post(db: &SqlitePool, id: i64, clients: Clients)-> Result<impl Reply, Rejection>{

    let query = "
        DELETE FROM posts
        where id = ?
    ";

    _ = sqlx::query(query)
    .bind(id)
    .execute(db)
    .await;

    let response = warp::reply::json(&StatusResponse {
        status: "Success".to_string(),

    });
    send_message_to_clients("Post has been deleted".to_string(), &clients).await;

    Ok(warp::reply::with_status(
        response,
        warp::http::StatusCode::CREATED,
    ))
}

//DOWNLOAD BASE64 FORMAT FILE BY POST ID
pub async fn download_file_by_id(db: &SqlitePool, id: i64) -> Result<impl Reply, Rejection> {
    let query = "
        SELECT uploaded_file
        FROM posts 
        WHERE id = ?
    ";

    match sqlx::query(query)
        .bind(id)
        .fetch_optional(db)
        .await
    {
        Ok(Some(row)) => {
            // Assuming `your_binary_data_column_name` is the correct column name
            let file_data: Vec<u8> = row.get("uploaded_file");

            // Now you have the binary data

            let file_data_base64 = base64::encode(&file_data);

            // Now you have the binary data encoded as base64
            // You can use `file_data_base64` in your response

            let response_data = FileResponse {
                // Other fields of your response
                status: "success".to_string(),
                data: file_data_base64,
            };

            Ok(warp::reply::json(&response_data))
        }
        Ok(None) => Err(warp::reject::not_found()),
        Err(e) => {
            eprintln!("Error fetching file data: {:?}", e);
            let error_response = ErrorResponse::new("Error fetching file data".to_string());
            Err(warp::reject::custom(error_response))
        }
    }
}



// async fn download_file(form: FormData) -> Result<impl Reply, Rejection> {
//     let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
//         eprintln!("form error: {}", e);
//         warp::reject::reject()
//     })?;
// }