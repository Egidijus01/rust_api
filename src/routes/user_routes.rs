use warp::{Filter, Rejection, Reply};
use sqlx::SqlitePool;
use crate::handlers::user_handlers;
use crate::models::response::UserRequest;


pub fn register_user_route(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path!("api" / "register"))
        .and(warp::body::json())
        .and(warp::any().map(move || pool.clone()))
        .and_then(|data: UserRequest, db: SqlitePool| async move {

            let result = user_handlers::register_user_handler(&db, data).await;

            // Convert the result to a warp::Reply
            match result {
                Ok(reply) => Ok(reply),
                Err(rejection) => Err(rejection),
            }
        })
    }

pub fn login_user_route(
    pool: SqlitePool
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
    .and(warp::path!("api" / "login"))
    .and(warp::body::json())
    .and(warp::any().map(move || pool.clone()))
    .and_then(|data: UserRequest, db: SqlitePool| async move{
        let result = user_handlers::login_user_handler(&db, data).await;

        match  result {
            Ok(reply) => Ok(reply),
            Err(rejection)  => Err(rejection),
        } 
            
        
    })
}


