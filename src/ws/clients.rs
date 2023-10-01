
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, mpsc};
use warp::ws::Message;

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

pub type Clients = Arc<Mutex<HashMap<String, Client>>>;

pub fn create_clients() -> Clients {
    Arc::new(Mutex::new(HashMap::new()))
}