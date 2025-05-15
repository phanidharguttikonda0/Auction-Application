use axum::{extract::{WebSocketUpgrade, State}};
use axum::extract::{ws::WebSocket, Path};
use axum::response::IntoResponse;
use uuid::Uuid;
use crate::AppState;

pub async fn handle_ws_upgrade(ws: WebSocketUpgrade, State(connections): State<AppState>,Path((room_id, participant_id)):Path<(String,i32)>) -> impl IntoResponse{
    ws.on_upgrade(move |socket| handle_ws(socket,connections,room_id,participant_id))
} // while establishing connection for initial request we need to send these room-id and participant-id

async fn handle_ws(mut socket: WebSocket,connections:AppState, room_id:String,participant_id:i32) {
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
Ready :
    -> It checks whether the room is full or not.
    -> send the next player details.
*/

/*
we need to write a logic for each bid , whether the bid is valid or not, by ensuring foriegn players limit
if the current bid is for foreign player and also the remaining amount is sufficient for buying 18 players
for the team.
-> room-creation
-> room-join
-> Ready [to start auction]
-> sold {send's room-id , so it will sell the player to the last bid, if last bid is empty goes unsold}
-> [both sold and Ready returns the next-player details.]
-> [once all teams has bought 16 players per team then the owner of the room can click on the intrested players
time, then in front-end we need to write logic of 3-5 mins time to be given for adding the players to the
intrested-players list, once added, all those players from each team there player-ids will be added to hash-set
and sent to the back-end via websocket and websocket recieves and redis will start looking to the intrested-players list]

-> set-intrested-players will be called to the websocket and websocket sets it to true by checking all
teams have bought 16 players per team and then it will fetch players from it and remove duplicates and
then calls those list of players and ends the auction if all are brought.



*/