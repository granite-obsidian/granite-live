mod handler;
mod ws;

use std::{
    collections::HashMap,
    convert::Infallible,
    sync::{Arc, Mutex},
};

use tokio::sync::mpsc;

use warp::{filters::ws::Message, reject::Rejection, Filter};

pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RegisterRequest {
    user_id: usize,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RegisterResponse {
    url: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Event {
    topic: String,
    user_id: Option<usize>,
    message: String,
}

pub struct Note {
    pub path: String,
    pub content: String,
}

pub struct Vault {
    pub vault_id: String,
    pub vault_name: String,
    pub notes: Vec<Note>,
}

type Result<T> = std::result::Result<T, Rejection>;
// type Clients = Arc<Mutex<HashMap<String, Client>>>;

type Vaults = Arc<Mutex<HashMap<String, Vault>>>;

#[tokio::main]
async fn main() {
    // let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let vaults: Vaults = Arc::new(Mutex::new(HashMap::new()));

    let ws_route = warp::path!("connect")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(vaults.clone()))
        .and_then(handler::ws_handler);

    // let health_route = warp::path!("health").and_then(handler::health_handler);

    // let register = warp::path("register");
    // let register_routes = register
    //     .and(warp::post())
    //     .and(warp::body::json())
    //     .and(with_clients(clients.clone()))
    //     .and_then(handler::register_handler)
    //     .or(register
    //         .and(warp::delete())
    //         .and(warp::path::param())
    //         .and(with_clients(clients.clone()))
    //         .and_then(handler::unregister_handler));

    // let publish = warp::path!("publish")
    //     .and(warp::body::json())
    //     .and(with_clients(clients.clone()))
    //     .and_then(handler::publish_handler);

    // let ws_route = warp::path!("ws")
    //     .and(warp::ws())
    //     .and(warp::path::param())
    //     .and(with_clients(clients.clone()))
    //     .and_then(handler::ws_handler);

    // let routes = health_route
    //     .or(register_routes)
    //     .or(ws_route)
    //     .or(publish)
    //     .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
