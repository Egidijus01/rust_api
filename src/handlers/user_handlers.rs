
use crate::handlers::posts_handler::ErrorResponse;

use crate::models::response::{UserRequest, StatusResponse, LoginResponse};
use sha2::digest::generic_array::GenericArray;
use sha2::{Sha256, Digest};
use sqlx::{SqlitePool, FromRow};
use warp::reply::with_status;
use crate::Middleware::auth::*;
use warp::{ Rejection, Reply};
use warp::http::StatusCode;
use warp::reply;


use warp::reject::Reject;
#[derive(Debug)]
struct WrongCredentialsError;


impl Reject for WrongCredentialsError {}
#[derive(Debug, FromRow)]
pub struct UserPassword {
    pub password: String,
}


#[utoipa::path(
    post,
    request_body = UserRequest,
    path = "/api/register",
    responses(
        (status = 200, description = "Registration successfull"),
        (status = NOT_FOUND, description = "Basic auth required")
    ),
)]
pub async fn register_user_handler(db: &SqlitePool, data:UserRequest) -> Result<impl Reply, Rejection> {
    let mut hasher = Sha256::new();
    
    
    hasher.update(&data.password.as_bytes());
    let hashed_result = hasher.finalize();
    
    let query = "
        INSERT INTO users (username, password)
        VALUES (?, ?)
    ";

    println!("{:?}", &hashed_result);
    _ = sqlx::query(query)
        .bind(&data.username)
        .bind(hex::encode(hashed_result))
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


async fn get_users_hash(db: &SqlitePool, username:String) -> Result<String, sqlx::Error> {
    let query = "
    SELECT password
    FROM users
    WHERE username = (?)
    ";

    let res: Result<UserPassword, sqlx::Error> = sqlx::query_as(query)
    .bind(username)
    .fetch_one(db)
    .await;

    match res {
        Ok(user_password) => Ok(user_password.password),
        Err(err) => Err(err),
    }
}




#[utoipa::path(
    post,
    request_body = UserRequest,
    path = "/api/login",
    responses(
        (status = 200, description = "Login using credentials. Returns bearer token.", body = LoginResponse),
        (status = NOT_FOUND, description = "Basic auth required")
    ),
    security(
        ("basic_auth" = [])
    )
)]
pub async fn login_user_handler(db: &SqlitePool, data:UserRequest) -> Result<impl Reply, Rejection> {
    let username = data.username;
    let form_password = data.password;
    


    match get_users_hash(db, username.clone()).await {
        Ok(db_password_hash) => {
         
            let mut hasher = Sha256::new();
            hasher.update(form_password.as_bytes());
            let provided_password_hash = hasher.finalize();
            
            let db_password_bytes = hex::decode(&db_password_hash);

            match db_password_bytes {
                Ok(db_password_bytes) => {
                    let db_password_array = GenericArray::clone_from_slice(&db_password_bytes);
                    // Compare the hashed passwords
                    if provided_password_hash == db_password_array {
                        
                        let token = create_jwt(&username)?;
                        let response = LoginResponse { token };
                        Ok(with_status(reply::json(&response), StatusCode::OK))
                    
                    } else {
                        // Passwords don't match
                        return Err(warp::reject::custom(ErrorResponse::new("Invalid credentials".to_string())));
                    }
                }
                Err(_) => {
                    // Handle the case where the hex string is not valid
                    return Err(warp::reject::custom(ErrorResponse::new("Invalid credentials".to_string())));
                }
            }
        }
        Err(_) => {
            // User not found in the database
            return Err(warp::reject::custom(ErrorResponse::new("User not found".to_string())));
        }
    }
    }

