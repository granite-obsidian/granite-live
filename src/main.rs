mod handler;

use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, Arc, RwLock},
};

use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;

use warp::{
    filters::ws::{Message, WebSocket},
    Filter,
};

pub struct Note {
    pub path: String,
    pub content: String,
}

pub struct Vault {
    pub vault_id: String,
    pub vault_name: String,
    pub notes: Vec<Note>,
}

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;
// type Vaults = Arc<Mutex<HashMap<String, Vault>>>;

// #[tokio::main]
// async fn main() {
//     // let vaults: Vaults = Arc::new(Mutex::new(HashMap::new()));

//     // let vaults = Vaults::default();
//     // let vaults = warp::any().map(move || vaults.clone());

//     let users = Users::default();
//     let users = warp::any().map(move || users.clone());

//     let connect = warp::path("connect")
//         .and(warp::ws())
//         .and(users)
//         .map(|ws: warp::ws::Ws, users| ws.on_upgrade(move |socket| user_connected(socket, users)));
//     // .and(warp::path::param())
//     // .and(with_clients(vaults.clone()))
//     // .and_then(handler::ws_handler);

//     let routes = connect;
//     // .or(register_routes)
//     // .or(ws_route)
//     // .or(publish)
//     // .with(warp::cors().allow_any_origin());

//     warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
// }

#[tokio::main]
async fn main() {
    // Create a WebSocket echo filter.
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(|websocket| handle_connection(websocket)));

    println!("Starting on 127.0.0.1:3030");
    // Start the Warp server.
    warp::serve(ws_route).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_connection(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();

    while let Some(result) = rx.next().await {
        // println!("Message");
        match result {
            Ok(msg) => {
                // println!(msg);
                if let Ok(text) = msg.to_str() {
                    print!("{}", text);
                }

                // Echo the received message back to the client.
                if let Err(e) = tx.send(msg).await {
                    eprintln!("Error sending message: {:?}", e);
                    return;
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {:?}", e);
                return;
            }
        }
    }
}

// fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
//     warp::any().map(move || clients.clone())
// }
