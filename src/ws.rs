use std::sync::mpsc;

use futures::StreamExt;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::filters::ws::{ws, WebSocket};

use crate::{Client, Result};

// pub async fn unregister_handler(id: String, clients: Clients) -> Result<impl Reply> {
//     clients.lock().await.remove(&id);
//     Ok(())
// }

pub async fn client_connection(ws: WebSocket, id: String, clients: Clients, mut client: Client) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));
}
