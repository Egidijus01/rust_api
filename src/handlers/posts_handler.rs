use crate::ws::clients::Clients;
use crate::ws::ws_handler::send_message_to_clients;
use crate::Middleware::mime_check::check_file_size;
use crate::models::posts::Post;
use crate::models::response::{SingePostResponse, PostResponse, CreatePostRequest, UpdatePostRequest, StatusResponse, PageQueryParam, FileResponse};

use sqlx::{SqlitePool, Row};
use base64::decode;
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
#[utoipa::path(
    get,
    path = "/api/posts",
    responses(
        (status = 200, description = "Authors found succescully", body = PostResponse),
        (status = NOT_FOUND, description = "Authors was not found")
    )
)]
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
#[utoipa::path(
    get,
    path = "/api/posts/{id}",
    responses(
        (status = 200, description = "Post found succescully", body = SingePostResponse),
        (status = NOT_FOUND, description = "Post was not found")
    ),
    params(
        ("id" = u64, Path, description = "Post database id to get post for"),
    )
)]
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
#[utoipa::path(
    post,
    request_body = CreatePostRequest,
    path = "/api/posts",
    responses(
        (status = 201, description = "Post created successfully", body = SingePostResponse),
        (status = 400, description = "Bad Request"),
    ),
    // security(
    //     ("bearer_auth" = [])
    // )
    
)]
pub async fn create_post(
    db: &SqlitePool,
    data: CreatePostRequest,
    clients: Clients,
) -> Result<impl Reply, Rejection> {


    let file_data = match &data.uploaded_files {
        Some(files) => {
            let mut file_data = Vec::new();

            for file in files {
                let decoded = decode(file);
                match decoded {
                    Ok(decoded) => file_data.push(decoded),
                    Err(_) => {
                        return Ok(warp::reply::with_status(
                            warp::reply::json(&ErrorResponse::new("Bad Request".to_string())),
                            warp::http::StatusCode::BAD_REQUEST,
                        ));
                    }
                }
            }

            Some(file_data)
        }
        None => None,
    };

    if let Some(file_data) = &file_data {
        for file in file_data {
            if check_file_size(file).is_none() || check_file_size(file).is_none() {
                return Ok(warp::reply::with_status(
                    warp::reply::json(&ErrorResponse::new("Bad Request".to_string())),
                    warp::http::StatusCode::BAD_REQUEST,
                ));
            }
        }
    }

    let query = "
        INSERT INTO posts (title, content, author_id)
        VALUES (?, ?, ?)
    ";

    let result = sqlx::query(query)
        .bind(&data.title)
        .bind(&data.content)
        .bind(&data.author_id)
        .execute(db)
        .await;

    if let Ok(row) = result {
        let post_id = row.last_insert_rowid();


        if let Some(file_data) = &file_data {
            for file in file_data {
                
                let query = "
                    INSERT INTO files (file, post_id)
                    VALUES (?, ?)
                ";

                let file_result = sqlx::query(query)
                    .bind(&file)
                    .bind(&post_id)
                    .execute(db)
                    .await;

                if let Err(_) = file_result {
                    // Handle the error if file insertion fails.
                    // You can return an error response or log the error here.
                }
            }
        }

        let post = Post {
            id: post_id,
            title: data.title.clone(),
            content: data.content.clone(),
            author_id: data.author_id.clone(),
            created_at: Default::default(),
            updated_at: Default::default(),
        };

        send_message_to_clients("Post has been created".to_string(), &clients).await;

        let response = warp::reply::json(&SingePostResponse {
            status: "Success".to_string(),
            data: post,
        });

        return Ok(warp::reply::with_status(
            response,
            warp::http::StatusCode::CREATED,
        ));
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&ErrorResponse::new("Failed to create post".to_string())),
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

//GET ALL POSTS BY AUTHOR ID
#[utoipa::path(
    get,
    path = "/api/posts/author/{id}",
    responses(
        (status = 200, description = "Post found succescully", body = SingePostResponse),
        (status = NOT_FOUND, description = "Post was not found")
    ),
    params(
        ("id" = u64, Path, description = "Author id"),
    )
)]
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
#[utoipa::path(
    patch,
    request_body = UpdatePostRequest,
    path = "/api/posts/{id}",  // Specify the path with {id} placeholder
    responses(
        (status = 201, description = "Post updated successfully", body = SingePostResponse),
        (status = 400, description = "Bad Request"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_post(db: &SqlitePool, data:UpdatePostRequest, post_id: i64, clients: Clients)-> Result<impl Reply, Rejection>{
   
    let file_data = match &data.uploaded_files {
        Some(files) => {
            let mut file_data = Vec::new();

            for file in files {
                let decoded = decode(file);
                match decoded {
                    Ok(decoded) => file_data.push(decoded),
                    Err(_) => {
                        return Ok(warp::reply::with_status(
                            warp::reply::json(&ErrorResponse::new("Bad Request".to_string())),
                            warp::http::StatusCode::BAD_REQUEST,
                        ));
                    }
                }
            }

            Some(file_data)
        }
        None => None,
    };

    if let Some(file_data) = &file_data {
        for file in file_data {
            if check_file_size(file).is_none() || check_file_size(file).is_none() {
                return Ok(warp::reply::with_status(
                    warp::reply::json(&ErrorResponse::new("Bad Request".to_string())),
                    warp::http::StatusCode::BAD_REQUEST,
                ));
            }
        }
    }

    
    let query = "
    UPDATE posts
    SET title = ?,
    content = ?,
    updated_at = ?
    WHERE id = ?
    ";

    let updated_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    

    _ = sqlx::query(query)
        .bind(&data.title)
        .bind(&data.content)
        .bind(&updated_at)
        .bind(post_id)
        .execute(db)
        .await;

    if let Some(file_data) = &file_data {
        let _ = sqlx::query("DELETE FROM files WHERE post_id = ?")
        .bind(post_id)
        .execute(db)
        .await;



        for file in file_data {
            
            let query = "
                INSERT INTO files (file, post_id)
                VALUES (?, ?)
            ";

            let file_result = sqlx::query(query)
                .bind(&file)
                .bind(&post_id)
                .execute(db)
                .await;

            if let Err(_) = file_result {
                // Handle the error if file insertion fails.
                // You can return an error response or log the error here.
            }
        }
    }

    let post = Post {
        id: Default::default(),
        title: data.title.clone(),
        content: data.content.clone(),
        author_id: Default::default(),
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
#[utoipa::path(
    delete,
    path = "/api/posts/{id}",
    responses(
        (status = 200, description = "Post deleted succescully"),
        (status = NOT_FOUND, description = "Post was not found")
    ),
    params(
        ("id" = u64, Path, description = "Post id"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
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

#[utoipa::path(
    get,
    path = "/api/posts/{id}/download",
    responses(
        (status = 200, description = "Post returned succescully"),
        (status = NOT_FOUND, description = "Post was not found")
    ),
    params(
        ("id" = u64, Path, description = "Post id"),
    )
)]
pub async fn download_files_by_id(db: &SqlitePool, id: i64) -> Result<impl Reply, Rejection> {
    let query = "
        SELECT file
        FROM files 
        WHERE post_id = ?
    ";

    match sqlx::query(query)
        .bind(id)
        .fetch_all(db)
        .await
    {
        Ok(rows) => {
            let mut file_data_base64 = Vec::new();

            for row in rows {
                let file_data: Vec<u8> = row.get("file");
                let file_data_base64_single = base64::encode(&file_data);
                file_data_base64.push(file_data_base64_single);
            }

            let response_data = FileResponse {
                // Other fields of your response
                status: "success".to_string(),
                data: file_data_base64,
            };

            Ok(warp::reply::json(&response_data))
        }
        Err(e) => {
            eprintln!("Error fetching file data: {:?}", e);
            let error_response = ErrorResponse::new("Error fetching file data".to_string());
            Err(warp::reject::custom(error_response))
        }
    }
}