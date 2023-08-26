use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc::unbounded_channel;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

use crate::{Users, NEXT_USER_ID};

pub async fn user_connected(ws: WebSocket, users: Users) {
    let user_id = NEXT_USER_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    println!("user_connected: {}", user_id);

    let (user_ws_tx, mut user_ws_rx) = ws.split();

    let (tx, rx) = unbounded_channel();

    let rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
            println!("ws sending error: {}", e);
        }
    }));

    // Blocks and waits for write access
    if let Ok(mut hashmap) = users.write() {
        hashmap.insert(user_id, tx);
    }

    // let users2 = users.clone();

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("ws error(uid={}): {}", user_id, e);
                break;
            }
        };
        user_message(user_id, msg, &users).await;
    }

    user_disconnected(user_id, &users).await;
}

async fn user_message(user_id: usize, msg: Message, users: &Users) {}
async fn user_disconnected(user_id: usize, users: &Users) {}
