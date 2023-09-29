use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Sender;
use warp::ws::{WebSocket, Message};


async fn websocket_handler(
    ws: WebSocket,
    clients: Arc<Mutex<HashMap<WebSocket, Sender>>>,
) {

    // Implement WebSocket message handling here
}

async fn send_notification(
    clients: Arc<Mutex<HashMap<WebSocket, Sender>>>,
    message: String,
) {
    let mut clients = clients.lock().await;
    for (_, sender) in clients.iter_mut() {
        if sender.send(Ok(Message::text(message.clone()))).await.is_err() {
            // Handle error when sending to a client
        }
    }
}

async fn remove_client(
    ws: WebSocket,
    clients: Arc<Mutex<HashMap<WebSocket, Sender>>>,
) {
    clients.lock().await.remove(&ws);
}