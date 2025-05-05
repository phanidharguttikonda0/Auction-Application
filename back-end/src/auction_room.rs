use axum::{extract::{WebSocketUpgrade, State}};
use axum::extract::ws::WebSocket;
use axum::response::IntoResponse;
use crate::AppState;

pub async fn handle_ws_upgrade(ws: WebSocketUpgrade, State(connections): State<AppState>) -> impl IntoResponse{
    ws.on_upgrade(move |socket| handle_ws(socket,connections))
}

async fn handle_ws(mut socket: WebSocket,connections:AppState) {
    tracing::info!("New connection was created");
} 