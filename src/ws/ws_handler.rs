use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::{ws::{Message, WebSocket}, reply::Reply, reject::Rejection};






pub async fn client_connection(ws: WebSocket, clients: Clients) {
    println!("establishing client connection... {:?}", ws);
    
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));
    let uuid = Uuid::new_v4().to_simple().to_string();
    let new_client = Client {
        client_id: uuid.clone(),
        sender: Some(client_sender),
    };

    
    clients.lock().await.insert(uuid.clone(), new_client);
    while let Some(result) = client_ws_rcv.next().await {
   
  
    }
    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}



pub async fn ws_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply, Rejection> {
    println!("ws_handler");
    Ok(ws.on_upgrade(move |socket| client_connection(socket, clients)))
}




pub async fn send_message_to_clients(msg: String, clients: &Clients) {

    

    let clients = clients.lock().await;
    for (_, client) in clients.iter() {
        if let Some(sender) = &client.sender {
            println!("sending veikia");
            let _ = sender.send(Ok(Message::text(msg.to_string())));
        }
    }
}