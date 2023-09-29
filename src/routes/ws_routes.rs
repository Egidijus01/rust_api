use warp::Filter;
use warp::ws::{WebSocket, Ws};



pub fn websocket_route() -> impl Filter<Extract = (Ws,), Error = warp::Rejection> {
    warp::path!("api" / "ws")
        .and(warp::ws())
        .map(|ws: Ws| ws)
}

// pub fn cleanup_route() -> impl Filter<Extract = (Ws,), Error = warp::Rejection> {
//     warp::path!("api" / "cleanup")
//         .and(warp::ws)
//         .and(wit)
// }