use std::{collections::HashMap, net::SocketAddr, sync::Mutex};

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    http::Method,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

use types::UserInfo;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/user", get(user_handler))
        .route("/", get(handler))
        .route("/login", post(login).get(login))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::GET, Method::POST])
                .allow_headers(Any)
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct User {
    user_id: u16,
    password: String,
}

lazy_static! {
    static ref MP: Mutex<HashMap<u16, bool>> = Default::default();
}

#[inline]
fn mp_get(k: &u16) -> bool {
    match MP.lock().unwrap().get(k) {
        Some(res) => res.to_owned(),
        None => false,
    }
}

#[inline]
fn mp_insert(k: u16, v: bool) {
    MP.lock().unwrap().insert(k, v);
}

#[derive(Serialize)]
enum LoginStatus {
    WrongPassword,
    Accepted,
    RepeatLogin,
}

async fn login(data: Json<User>) -> impl IntoResponse {
    let user = data.0;
    println!("login request: {}", serde_json::to_string(&user).unwrap());
    if user.user_id == 1 && user.password == "a" {
        if mp_get(&user.user_id) {
            Json(LoginStatus::RepeatLogin)
        } else {
            mp_insert(user.user_id, true);
            Json(LoginStatus::Accepted)
        }
    } else {
        Json(LoginStatus::WrongPassword)
    }
}

async fn handler() -> impl IntoResponse {
    "Hello, from server!"
}

async fn user_handler() -> impl IntoResponse {
    let user = UserInfo {
        id: 1,
        name: "Server user".to_owned(),
    };

    Json(user)
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    loop {
        if let Some(msg) = socket.recv().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(t) => {
                        // Echo
                        if socket
                            .send(Message::Text(format!("Echo from backend: {}", t)))
                            .await
                            .is_err()
                        {
                            return;
                        }
                    }
                    Message::Close(_) => {
                        return;
                    }
                    _ => {}
                }
            } else {
                return;
            }
        }
    }
}
