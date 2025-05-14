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

/*
websocket need to take care of :
    decode the authorization header and get the actual values
room-creation :
    -> get the team-selected,accessibility into String.
    -> call the room_create handler if successfully call room-join.
    -> Now create an Redis Schema for that Room.
room-join :
    -> first need to be check whether the room was in waiting or ongoing state.
    -> then check whether the user exists or not.
    -> if exists then allow him to join by sending the Room type data.
    -> else if room was not in ongoing state join the room and then send room-id
    -> else send a string with Invalid Room Id
player-sold :
    -> calling the player-sold handler it will add the player to the sql
    -> adding the player to the redis as well
*/