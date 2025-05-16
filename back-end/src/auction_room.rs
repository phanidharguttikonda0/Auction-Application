use std::time::Duration;
use async_graphql::futures_util::{SinkExt, StreamExt};
use axum::{extract::{WebSocketUpgrade, State}};
use axum::extract::{ws::WebSocket, Path};
use axum::extract::ws::Message;
use axum::response::IntoResponse;
use tokio::sync::mpsc::unbounded_channel;
use uuid::Uuid;
use crate::AppState;

pub async fn handle_ws_upgrade(ws: WebSocketUpgrade, State(connections): State<AppState>,Path((room_id, participant_id)):Path<(String,i32)>) -> impl IntoResponse{
    ws.on_upgrade(move |socket| handle_ws(socket,connections,room_id,participant_id))
} // while establishing connection for initial request we need to send these room-id and participant-id

async fn handle_ws(mut socket: WebSocket,connections:AppState, room_id:String,participant_id:i32) {
    tracing::info!("New connection was created");
    let (tx, mut rx) = unbounded_channel::<Message>();
    let second_connection = connections.clone();

    let (mut sender, mut reciever) = socket.split();
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if tokio::time::timeout(Duration::from_secs(12), sender.send(msg)).await.is_err() { // if the message was not reached to the client with in 12 seconds then user connection will be removed , so user needs to re-join again

                tracing::error!("User was not able to reach messages on time");

                if let Err(err) = sender.close().await { // if error occurs while closing this block executes
                    tracing::error!("Error while closing the connection : {:?}",err);
                }
                let mut read_sockets = second_connection.websocket_connections.read() ;

                match read_sockets {
                    Ok(read_sockets) => {
                        let participants_list = read_sockets.get(&room_id) ;
                        let participants_list = participants_list.unwrap();
                        let mut index:usize = 0 ;
                        for participant in participants_list {
                            if participant.participant_id == participant_id {
                                let mut write_socket = second_connection.websocket_connections.write().unwrap() ;
                                let mut participants_list = write_socket.get_mut(&room_id).unwrap() ;
                                participants_list.remove(index) ;
                                drop(write_socket);
                                break;
                            }
                        }
                    },
                    Err(err) => {
                        tracing::error!("Error while writing to the websocket connections : {:?}",err);
                    }
                }

                break;

            }
        }
    }) ;




    while let Some(msg) = reciever.next().await { // where we actually recieve messages from the client

        match msg {
            Ok(msg) => {
                match msg {
                    Message::Text(text) => {
                        tracing::info!("Text message received : {:?}", text) ;
                        
                        
                        
                        
                        
                        
                    },
                    Message::Binary(bin) => {
                        tracing::info!("Binary message received : {:?}",bin);
                    },
                    Message::Ping(bin) => {
                        tracing::info!("Ping Message received : {:?}",bin) ;
                    },
                    Message::Pong(bin) => {
                        tracing::info!("Pong Message received : {:?}",bin) ;
                    },
                    Message::Close(reason) => { // here client closes the connection,
                        // but no need to close because when another notification we are sending to all
                        // participants if the user weren't recieved then user automatically will be removed
                        // and also while adding the new connection we will check whether the connection
                        // for the already existing user was there or not , if it's there we will be
                        // overided the existing and then we will update with new one, in room-join request
                        tracing::info!("Client closed the connection : {:?}",reason);
                        break;
                    }
                }
            },
            Err(err) => {
                tracing::error!("Error occured while recieving messages from the client : {:?}",err);
            }
        }


    }

}



async fn broadcast_message(msg: Message, room_id: String,state: &mut AppState) {

    let mut read_sockets = state.websocket_connections.read().unwrap() ;
    let participants_list = read_sockets.get(&room_id).unwrap() ;
    
    for participant in participants_list {
        
        if let Err(err) = participant.connection.send(msg.clone()) {
            tracing::error!("Error while sending message to the client : {:?}",err);
        }else{
            tracing::info!("Message was sent to the client successfully");   
        }
        
    }
     
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