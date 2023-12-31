use warp::{Filter, Rejection, Reply};
use crate::ws::clients::Clients;
use sqlx::SqlitePool;
use crate::handlers::author_handler::*;
use crate::Middleware::auth::with_auth;
use crate::models::response::{CreateAuthorRequest, UpdateAuthorRequest, PageQueryParam, SearchQueryParam};



//ROUTE FOR ALL AUTHORS
pub fn get_authors(db: SqlitePool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db_clone = db.clone();
    warp::path!("api" / "authors")
        .and(warp::get())
        .and(warp::query::<PageQueryParam>())
        .and(warp::query::<SearchQueryParam>())
        .and_then(move |page_query_param: PageQueryParam, search_query_params: SearchQueryParam| {
            let db_clone = db_clone.clone();
         

            async move {
                let search_param = search_query_params.search;
                get_all_authors(page_query_param, search_param, &db_clone).await
            }
        })
}


//ROUTE FOR SINGLE AUTHOR
pub fn get_author_route(db: SqlitePool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "authors" / i64)
        .and(with_auth())
        .and(warp::get())
        .and_then(move |id: i64, _: String| {
            let db_clone = db.clone(); 
            async move {
                get_author(&db_clone, id).await 
            }
        })
}

//ROUTE TO POST AN AUTHOR
pub fn post_author_route(
    pool: SqlitePool,
    clients: Clients
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {

    let clients_clone = clients.clone();



    warp::post()
        .and(warp::path!("api" / "authors"))
        .and(with_auth())
        .and(warp::body::json())
        .and(warp::any().map(move || pool.clone()))
        .and_then(move|_: String, data: CreateAuthorRequest, db: SqlitePool| {
                let clients_clone = clients_clone.clone();
    
                async move {
                    
                    post_author(&db, data, clients_clone).await
                }
            })

}


    //ROUTE TO UPDATE AN AUTHOR
pub fn update_author_route(
    pool: SqlitePool,
    clients: Clients
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {

    warp::patch()
        .and(warp::path!("api" / "authors" / i64))
        .and(with_auth())
        .and(warp::body::json())
        .and(warp::any().map(move || pool.clone()))
        .and_then(move|id: i64, _: String, data: UpdateAuthorRequest, db: SqlitePool| {

            let clients_clone = clients.clone();

            async move {
                
                let result = update_author(&db, data, id, clients_clone).await;

          
            match result {
                Ok(reply) => Ok(reply),
                Err(rejection) => Err(rejection),
            }
            }
        })
            
            
        
}

//DELTE AUTHOR ROUTE
pub fn delete_author_route(db: SqlitePool, clients: Clients) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "authors" / i64)
        .and(warp::delete())
        .and(with_auth())
        .and_then(move |id: i64, _: String| {
            let db_clo = db.clone(); // Clone the db object
            let clients_clone = clients.clone();
            async move {
                delete_author(&db_clo, id, clients_clone).await // Call the function with the cloned db
            }
        })
}

// pub fn upload_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {


//     warp::path("upload")
//         .and(warp::post())
//         .and(warp::multipart::form().max_length(5_000_000))
//         .and_then(upload)
// }
