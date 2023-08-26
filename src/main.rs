mod handler;

use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, Arc, RwLock},
};

use tokio::sync::mpsc;

use warp::{filters::ws::Message, Filter};

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

#[tokio::main]
async fn main() {
    // let vaults: Vaults = Arc::new(Mutex::new(HashMap::new()));

    // let vaults = Vaults::default();
    // let vaults = warp::any().map(move || vaults.clone());

    let users = Users::default();
    let users = warp::any().map(move || users.clone());

    let connect = warp::path("connect")
        .and(warp::ws())
        .and(users)
        .map(|ws: warp::ws::Ws, users| ws.on_upgrade(move |socket| user_connected(socket, users)));
    // .and(warp::path::param())
    // .and(with_clients(vaults.clone()))
    // .and_then(handler::ws_handler);

    let routes = connect;
    // .or(register_routes)
    // .or(ws_route)
    // .or(publish)
    // .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

// fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
//     warp::any().map(move || clients.clone())
// }
