use warp::{Filter, Rejection, Reply};
use sqlx::SqlitePool;
use crate::Middleware::auth::with_auth;
use crate::handlers::posts_handler;

use crate::models::response::{ CreatePostRequest, UpdatePostRequest, PageQueryParam, SearchQueryParam};


//ROUTE FOR ALL POSTS
pub fn get_posts(db: SqlitePool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("api" / "posts")
        .and(warp::get())
        .and(warp::query::<PageQueryParam>())
        .and(warp::query::<SearchQueryParam>())
        .and_then(move |page_query_param:PageQueryParam, search_query:SearchQueryParam| {
            let db_clone = db.clone();
            async move {
                let search_param = search_query.search;
                posts_handler::get_all_posts(page_query_param, search_param, &db_clone).await 
            }
        })
}



//ROUTE FOR SINGLE POST
pub fn get_post_route(
    db: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "posts" / i64)
        .and(warp::get())
        .and(with_auth()) // Add authentication here
        .and(warp::any().map(move || db.clone())) // Inject the database pool
        .and_then(|id: i64, _ : String, db: SqlitePool| {
            let db_clone = db.clone();
            async move {
                posts_handler::get_post(&db_clone, id).await // Call the get_post handler
            }
        })
}


//ROUTE TO CREATE A POST
pub fn create_post_route(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path!("api" / "posts"))
        .and(with_auth())
        .and(warp::body::json())
        .and(warp::any().map(move || pool.clone()))
        .and_then(| _: String, data: CreatePostRequest, db: SqlitePool| async move {

            let result = posts_handler::create_post(&db, data).await;

            // Convert the result to a warp::Reply
            match result {
                Ok(reply) => Ok(reply),
                Err(rejection) => Err(rejection),
            }
        })
    }

//ROUTE FOR POSTS BY AUTHOR ID

pub fn get_posts_by_author(db: SqlitePool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("api" / "posts" / "author" / i64)
        .and(with_auth())
        .and(warp::get())
        .and_then(move |id: i64, _: String,| {
            let db_clone = db.clone();
            async move {
                posts_handler::get_posts_by_auth(&db_clone, id).await
            }
        })
}


//ROUTE TO UPDATE A POST
pub fn update_post_route(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::patch()
        .and(warp::path!("api" / "posts" / i64))
        .and(with_auth())
        .and(warp::body::json())
        .and(warp::any().map(move || pool.clone()))
        .and_then( |id: i64,_: String, data: UpdatePostRequest, db: SqlitePool| async move {

            let result = posts_handler::update_post(&db, data, id).await;

            // Convert the result to a warp::Reply
            match result {
                Ok(reply) => Ok(reply),
                Err(rejection) => Err(rejection),
            }
        })
}



//DELETE POST ROUTE
pub fn delete_post_route(db: SqlitePool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "posts" / i64)
        .and(warp::delete())
        .and(with_auth())
        .and_then(move |id: i64, _: String| {
            let db_clo = db.clone(); // Clone the db object
            async move {
                posts_handler::delete_post(&db_clo, id).await // Call the function with the cloned db
            }
        })
}


