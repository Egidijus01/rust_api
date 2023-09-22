
use warp::{Filter, Rejection, Reply};

use sqlx::SqlitePool;

use crate::handler::{self, create_post, update_post, update_author};
use crate::handler::post_author;
use crate::response::{CreateAuthorRequest, CreatePostRequest, UpdatePostRequest, UpdateAuthorRequest};

pub fn routes(db: &SqlitePool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone + Send {
    let author_filter =  get_author(db.clone());
    let authors_filter = get_authors(db.clone());
    let post_author_filter = post_author_route(db.clone());
    let posts_filter = get_posts(db.clone());
    let post_filter = get_post(db.clone());
    let create_post_filter = create_post_route(db.clone());
    let get_posts_by_author = get_posts_by_author(db.clone());
    let update_post_filter = update_post_route(db.clone());
    let update_author_filter = update_author_route(db.clone());
    let delete_post_filter = delete_post_route(db.clone());
    let delete_author_filter = delete_author_route(db.clone());

    author_filter
    .or(authors_filter)
    .or(post_author_filter)
    .or(post_filter)
    .or(posts_filter)
    .or(create_post_filter)
    .or(get_posts_by_author)
    .or(update_post_filter)
    .or(update_author_filter)
    .or(delete_author_filter)
    .or(delete_post_filter)

}

//ROUTE FOR ALL AUTHORS
fn get_authors(db: SqlitePool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("api" / "authors")
        .and(warp::get())
        .and_then(move || {
            let db_clone = db.clone();
            async move {
                handler::get_all_authors(&db_clone).await
            }
        })
}



//ROUTE FOR SINGLE AUTHOR
fn get_author(db: SqlitePool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "authors" / i64)
        .and(warp::get())
        .and_then(move |id: i64| {
            let db_clo = db.clone(); 
            async move {
                handler::get_author(&db_clo, id).await 
            }
        })
}

//ROUTE FOR ALL POSTS
fn get_posts(db: SqlitePool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("api" / "posts")
        .and(warp::get())
        .and_then(move || {
            let db_clone = db.clone();
            async move {
                handler::get_all_posts(&db_clone).await 
            }
        })
}



//ROUTE FOR SINGLE POST
fn get_post(db: SqlitePool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "posts" / i64)
        .and(warp::get())
        .and_then(move |id: i64| {
            let db_clo = db.clone(); 
            async move {
                handler::get_post(&db_clo, id).await 
            }
        })
}


//ROUTE TO POST AN AUTHOR
pub fn post_author_route(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path!("api" / "authors"))
        .and(warp::body::json())
        .and(warp::any().map(move || pool.clone()))
        .and_then(|data: CreateAuthorRequest, db: SqlitePool| async move {

            let result = post_author(&db, data).await;

            // Convert the result to a warp::Reply
            match result {
                Ok(reply) => Ok(reply),
                Err(rejection) => Err(rejection),
            }
        })
    }


//ROUTE TO CREATE A POST
pub fn create_post_route(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path!("api" / "posts"))
        .and(warp::body::json())
        .and(warp::any().map(move || pool.clone()))
        .and_then(|data: CreatePostRequest, db: SqlitePool| async move {

            let result = create_post(&db, data).await;

            // Convert the result to a warp::Reply
            match result {
                Ok(reply) => Ok(reply),
                Err(rejection) => Err(rejection),
            }
        })
    }

//ROUTE FOR POSTS BY AUTHOR ID

fn get_posts_by_author(db: SqlitePool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("api" / "posts" / "author" / i64)
        .and(warp::get())
        .and_then(move |id: i64| {
            let db_clone = db.clone();
            async move {
                handler::get_posts_by_auth(&db_clone, id).await
            }
        })
}


//ROUTE TO UPDATE A POST
pub fn update_post_route(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::patch()
        .and(warp::path!("api" / "posts" / "update" / i64))
        .and(warp::body::json())
        .and(warp::any().map(move || pool.clone()))
        .and_then(|id: i64, data: UpdatePostRequest, db: SqlitePool| async move {

            let result = update_post(&db, data, id).await;

            // Convert the result to a warp::Reply
            match result {
                Ok(reply) => Ok(reply),
                Err(rejection) => Err(rejection),
            }
        })
}


//ROUTE TO UPDATE AN AUTHOR
pub fn update_author_route(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::patch()
        .and(warp::path!("api" / "authors" / "update" / i64))
        .and(warp::body::json())
        .and(warp::any().map(move || pool.clone()))
        .and_then(|id: i64, data: UpdateAuthorRequest, db: SqlitePool| async move {

            let result = update_author(&db, data, id).await;

          
            match result {
                Ok(reply) => Ok(reply),
                Err(rejection) => Err(rejection),
            }
        })
}

//DELETE POST ROUTE
fn delete_post_route(db: SqlitePool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "posts" / "delete" / i64)
        .and(warp::delete())
        .and_then(move |id: i64| {
            let db_clo = db.clone(); // Clone the db object
            async move {
                handler::delete_post(&db_clo, id).await // Call the function with the cloned db
            }
        })
}


//DELTE AUTHOR ROUTE
fn delete_author_route(db: SqlitePool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "authors" / "delete" / i64)
        .and(warp::delete())
        .and_then(move |id: i64| {
            let db_clo = db.clone(); // Clone the db object
            async move {
                handler::delete_author(&db_clo, id).await // Call the function with the cloned db
            }
        })
}