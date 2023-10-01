use sqlx::SqlitePool;
use warp::Filter;
use crate::routes::author_routes::*;
use crate::routes::posts_routes::*;
use crate::routes::user_routes::*;
use crate::ws::clients::Clients;



pub fn routes(db: &SqlitePool, clients: Clients) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone + Send {
    let author_filter =  get_author_route(db.clone());
    let authors_filter = get_authors(db.clone());
    let post_author_filter = post_author_route(db.clone(), clients.clone());
    let posts_filter = get_posts(db.clone());
    let post_filter = get_post_route(db.clone());
    let create_post_filter = create_post_route(db.clone(), clients.clone());
    let get_posts_by_author = get_posts_by_author(db.clone());
    let update_post_filter = update_post_route(db.clone(), clients.clone());
    let update_author_filter = update_author_route(db.clone(), clients.clone());
    let delete_post_filter = delete_post_route(db.clone(), clients.clone());
    let delete_author_filter = delete_author_route(db.clone(), clients.clone());
    let register_user_filter = register_user_route(db.clone());
    let login_user_filter = login_user_route(db.clone());
    let dowload_file_filter = download_file(db.clone());
    // let upload = upload_route();
   
   
   
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
    .or(register_user_filter)
    .or(login_user_filter)
    .or(dowload_file_filter)

}
