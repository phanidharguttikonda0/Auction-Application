use std::time::Duration;
use async_graphql::futures_util::{SinkExt, StreamExt};
use axum::{extract::{WebSocketUpgrade, State}};
use axum::extract::{ws::WebSocket, Path};
use axum::extract::ws::Message;
use axum::response::IntoResponse;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use uuid::Uuid;
use crate::AppState;
use crate::handlers::players::{is_foreign_player, player};
use crate::handlers::redis_handlers::{add_intrested_players, next_player, check_for_ready, get_Room, is_in_waiting, new_bid, sell_player, new_participant, participant_exists, redis_room_creation, room_exists, intrested_players_set, player_from_redis,  get_current_player,  bid_allowance_data, is_owner, all_teams_16_players};
use crate::handlers::room_handler::{get_room_type, get_team_name, create_room, join_room, player_sold, change_room_status};
use crate::middlewares::authentication::authorization_decode;
use crate::models::players::Player;
use crate::models::rooms::{Bid, BidReturn, CreateRoom, CurrentBid, IntrestedPlayers, JoinRoom, NewJoiner, ParticipantsConnections, PlayerSold, Players, RedisRoom, Room, RoomCreation, RoomJoin, RoomStatus};


pub async fn handle_ws_upgrade(ws: WebSocketUpgrade, State(connections): State<AppState>, Path((room_id, user_id)):Path<(String, i32)>) -> impl IntoResponse{
    ws.on_upgrade(move |socket| handle_ws(socket,connections,room_id,user_id))
} // while establishing connection for initial request we need to send these room-id and participant-id

async fn handle_ws(mut socket: WebSocket, mut connections:AppState, mut room_id:String, user_id:i32) {
    tracing::info!("New connection was created");
    let (tx, mut rx) = unbounded_channel::<Message>();
    let second_connection = connections.clone();

    let (mut sender, mut reciever) = socket.split();
    let mut room_id_ = room_id.clone();

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if String::from(msg.to_text().unwrap()).starts_with("$") {
                let room_id = String::from(msg.to_text().unwrap()) ;
                let room_id: Vec<&str> = room_id.split(":").collect() ;
                tracing::info!("room-id before updating was {}", room_id_) ;
                room_id_ = room_id[1].to_string() ;
                tracing::info!("The updated room_id was {}", room_id_) ;
            } else if tokio::time::timeout(Duration::from_secs(12), sender.send(msg)).await.is_err() { // if the message was not reached to the client with in 12 seconds then user connection will be removed , so user needs to re-join again

                tracing::error!("User was not able to reach messages on time");

                if let Err(err) = sender.close().await { // if error occurs while closing this block executes
                    tracing::error!("Error while closing the connection : {:?}",err);
                }


                let mut read_sockets = second_connection.websocket_connections.read() ;

                match read_sockets {
                    Ok(read_sockets) => {
                        let participants_list = read_sockets.get(&room_id_) ;
                        let participants_list = participants_list.unwrap();
                        let mut index:usize = 0 ;
                        for participant in participants_list {
                            if participant.user_id == user_id {
                                let mut write_socket = second_connection.websocket_connections.write().unwrap() ;
                                let mut participants_list = write_socket.get_mut(&room_id_).unwrap() ;
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
                        let room_creation = serde_json::from_str::<RoomCreation>(&text) ;
                        let room_join = serde_json::from_str::<RoomJoin>(&text) ;
                        let bid = serde_json::from_str::<Bid>(&text) ;
                        let intrested_players_list = serde_json::from_str::<IntrestedPlayers>(&text) ;

                        if let Ok(room_creation) = room_creation {
                            let claims = authorization_decode(room_creation.authorization_header) ;

                            match claims {
                                Some(claims) => {
                                    tracing::info!("we got the claims : {:?}",claims);
                                    let team_name = get_team_name(room_creation.team.clone()) ;
                                    let mut con = connections.sql_database.begin().await.unwrap();
                                    let room_id  = create_room(CreateRoom{
                                        max_players: room_creation.max_players,
                                        user_id: claims.user_id,
                                        team_selected: team_name.clone(),
                                        accessibility: get_room_type(room_creation.room_type.clone())
                                    }, &mut con).await;

                                    match room_id {
                                        Ok(room_id) => {
                                            let participant_id = join_room(JoinRoom{
                                                team_selected: team_name.clone(),
                                                user_id: claims.user_id,
                                                room_id
                                            }, &mut con).await ;
                                            match participant_id {
                                                Ok(participant_id) => {
                                                    // storing an websocket connection for the user
                                                    // after storing we need to change the room-id in the
                                                    // to the task that recieves the messages in the format
                                                    // room-id:"..."
                                                    add_connection(&connections, room_id.clone().to_string(), user_id, tx.clone()).await ;
                                                    // here we are rewriting the room-id that is sent something else
                                                    tx.send(Message::from(
                                                        format!("$room_id:{}",room_id.clone())
                                                    )).unwrap() ;
                                                    con.commit().await.unwrap();
                                                    let mut players_teams = Vec::new() ;
                                                    players_teams.push((participant_id,team_name.clone())) ;

                                                    let room = Room {
                                                        room_id: room_id,
                                                        room_type: room_creation.room_type,
                                                        max_players: room_creation.max_players,
                                                        players_teams,
                                                        status: RoomStatus::WAITING,
                                                    } ;
                                                    // know we need to call the redis function to store it in redis
                                                    redis_room_creation(room.clone(),claims.user_id,&connections.redis_connection).await ;

                                                    // sending the response
                                                    tx.send(Message::from(
                                                        serde_json::to_string::<Room>(&room).unwrap()
                                                    )).unwrap() ;
                                                },
                                                Err(err) => {
                                                    con.rollback().await.unwrap();
                                                    tracing::error!("Error while joining the room : {:?}",err);
                                                    tx.send(Message::from(String::from("Error while joining the room"))).unwrap()
                                                }
                                            }
                                        },
                                        Err(err) => {
                                            tracing::error!("Error while creating the room : {:?}",err);
                                            tx.send(Message::from(String::from("Error while creating the room"))).unwrap()
                                        }
                                    }

                                },
                                None => {
                                    tracing::error!("Invalid authorization header");
                                    tx.send(Message::from(String::from("Invalid authorization header"))).unwrap()
                                }
                            }

                        }else if let Ok(room_join) = room_join {
                            // firstly let's check room is exists or not if then check user already exists or not
                            // before that also check whether the room is in waiting state or not

                            let claims = authorization_decode(room_join.authorization_header) ;

                            match claims {
                                Some(claims) => {
                                    // let's check whether user exists or not using redis
                                    if room_exists(claims.user_id, &connections.redis_connection).await {
                                        let val = participant_exists(claims.user_id,Uuid::to_string(&room_join.room_id), &connections.redis_connection).await ;
                                        if  val.0 {
                                            add_connection(&connections, room_id.clone().to_string(), user_id, tx.clone()).await ;
                                            tx.send(Message::from(
                                                serde_json::to_string::<Room>(&get_Room(Uuid::to_string(&room_join.room_id), &connections.redis_connection).await).unwrap()
                                            )).unwrap();
                                            // as he was already a member, so no need to send a response to the participants
                                        }else{
                                            if is_in_waiting(Uuid::to_string(&room_join.room_id),&connections.redis_connection).await {
                                                let mut con = connections.sql_database.begin().await.unwrap();
                                                let team = get_team_name(room_join.team_selected) ;
                                                // firstly join as new participant
                                                let participant_id = join_room(JoinRoom{
                                                    team_selected: team.clone(),
                                                    user_id: claims.user_id,
                                                    room_id: room_join.room_id
                                                }, &mut con).await ;
                                                match participant_id {
                                                    Ok(participant_id) => {
                                                        // store this new participant in the redis
                                                            match new_participant(Uuid::to_string(&room_join.room_id), claims.user_id, team.clone(), participant_id, &connections.redis_connection).await {
                                                                Ok(_) => {

                                                                    con.commit().await.unwrap();
                                                                    add_connection(&connections, room_id.clone().to_string(), user_id, tx.clone()).await ;
                                                                    // thirdly send the Redis Room data back to him
                                                                    tx.send(Message::from(
                                                                        serde_json::to_string::<Room>(&get_Room(Uuid::to_string(&room_join.room_id), &connections.redis_connection).await).unwrap()
                                                                    )).unwrap();
                                                                    // finally broadcast the new participant_id, user_id and team_selected to the remaining participants
                                                                    broadcast_message(Message::from(
                                                                        serde_json::to_string(&NewJoiner{
                                                                            participant_id,
                                                                            user_id: claims.user_id,
                                                                            team_selected: team,
                                                                        }).unwrap()
                                                                    ),Uuid::to_string(&room_join.room_id),&mut connections).await ;
                                                                },
                                                                Err(err) => {
                                                                    con.rollback().await.unwrap();
                                                                    tracing::error!("Error while adding the participant to the redis : {:?}",err);
                                                                    tx.send(Message::from(String::from("Error while adding the participant to the redis"))).unwrap()
                                                                }
                                                            }

                                                    },
                                                    Err(err) => {
                                                        con.rollback().await.unwrap();
                                                        tracing::error!("Error while joining the room : {:?}",err);
                                                        tx.send(Message::from(String::from("Error while joining the room"))).unwrap()
                                                    }
                                                }

                                            }else {
                                                tracing::error!("Room is not in waiting state, as room was full");
                                                tx.send(Message::from(String::from("Room is not in waiting state, as room was full"))).unwrap() ;
                                            }
                                        }
                                    }else{
                                        tracing::error!("Room doesn't exists");
                                        tx.send(Message::from(String::from("Invalid Room-Id"))).unwrap() ;
                                    }
                                },
                                None => {
                                    tracing::error!("Invalid authorization header");
                                    tx.send(Message::from(String::from("Invalid authorization header"))).unwrap() ;
                                }
                            }


                        }else if let Ok(bid) = bid {
                            // firstly check whether the bid is valid or not
                            if bid_allowance(bid.room_id.clone(),bid.participant_id,bid.amount,&mut connections).await {
                                match new_bid(bid.room_id.clone(),bid.participant_id,bid.amount,&connections.redis_connection).await {
                                    Ok(team) => {
                                        tracing::info!("Bid was added successfully");
                                        broadcast_message(Message::from(
                                            serde_json::to_string(&BidReturn { team, amount: bid.amount}).unwrap()
                                        ),bid.room_id,&mut connections).await ;
                                    },
                                    Err(err) => {
                                        tracing::error!("Error while adding the bid to the redis : {:?}",err);
                                        tx.send(Message::from(String::from("Error while adding the bid to the redis"))).unwrap() ;
                                    }
                                }
                            }else{
                                tracing::warn!("Bid is not allowed due to less purse to buy the threshold players or foreign players limit") ;
                                tx.send(Message::from(String::from("Bid is not allowed due to less purse to buy the threshold players or foreign players limit"))).unwrap() ;
                            }
                        }else if let Ok(intrested_players) = intrested_players_list{
                            // each participant will send the list to here
                            match add_intrested_players(intrested_players.players, intrested_players.room_id.clone(),user_id, &connections.redis_connection).await {
                                Ok(team) => {
                                    broadcast_message(Message::from(team),intrested_players.room_id,&mut connections).await ;
                                },
                                Err(err) => {
                                    tracing::error!("Error while adding the interested players to the redis : {:?}",err);
                                    tx.send(Message::from(String::from("Error while adding the interested players to the redis"))).unwrap() ;
                                }
                            }
                        }else{
                            if text == "READY" { // called by room-owner only

                                // firstly let's check whether all the max-players joined or not
                                if check_for_ready(room_id.clone() ,&connections.redis_connection).await {
                                    let player = player(&connections, 1).await.unwrap() ;
                                    // need to change the room-status
                                    let status = change_room_status(&connections,room_id.clone(),RoomStatus::ONGOING).await ;
                                    if status {
                                        broadcast_message(Message::from(serde_json::to_string(&player).unwrap()),room_id.clone(),&mut connections).await ;
                                    }else{
                                        tx.send(Message::from(String::from("status was not updated"))).unwrap() ;
                                    }
                                }else{
                                    tracing::error!("Not all the players joined the room or it's not the owner who is trying to start the room");
                                    tx.send(Message::from(String::from("Not all the players joined the room or it's not the owner who is trying to start the room"))).unwrap() ;
                                }
                            }else if text == "SOLD" { // in front-end from the last-bid person this will be called
                               //* when a player has done bid then a timer will be started in his front-end and if any other bids were came that timer will be stopped

                                // first step is to called the redis function to update the player-sold and return the player_id
                                let selling_player = sell_player(room_id.clone(), &connections.redis_connection).await.unwrap() ;


                                // second-step call the sold function in rooms handler to update in sqlx
                                let value = player_sold(selling_player.clone(), &mut connections).await ;



                                // broadcast the message to the remaining participants that this particular team brought that player
                                broadcast_message(Message::from(value),room_id.clone(),&mut connections).await ;

                                // third step is get the next player

                                let player: Player = if intrested_players_set(room_id.clone(), &connections.redis_connection).await {
                                        player(&connections, player_from_redis(room_id.clone(), &connections.redis_connection).await.unwrap()).await.unwrap()
                                }else{
                                    player(&connections, selling_player.player_id+1).await.unwrap()
                                } ;

                                // fourth step is store the next player in the redis
                                next_player(room_id.clone(),player.id, &connections.redis_connection).await ;

                                // fifth step is to broadcast the new player to the remaining participants
                                broadcast_message(Message::from(serde_json::to_string(&player).unwrap()),room_id.clone(),&mut connections).await ;

                            }else if text == "WANT-TO-SET-INTRESTED-PLAYERS"{ // CALLED BY ONLY OWNER OF THE ROOM
                                // FIRSTLY NEED TO CALLED BY OWNER OR NOT AND THEN ALL TEAMS BROUGHT 16 PLAYERS MINIMUM
                             // FIRST WE NEED TO SEND THIS, IT WILL RETURN THE LIST OF PLAYERS WHICH ARE UNSOLD AND NOT CAME YET
                             // NOW USER'S NEED TO SELECT FROM THIS LIST AND THEN ALL THE TEAMS WILL SELECT THE PLAYERS AND THEN
                             // CLICK CONTINUE AND IN FRONT-END ITSELF DUPLICATES NEED TO BE REMOVED AND THEN SEND TO BACK-END
                                if check_for_intrested_players(room_id.clone(), user_id, &mut connections).await {
                                    tracing::info!("good to go with the interested_players") ;
                                    match send_players(room_id.clone(), &mut connections).await {
                                        Ok(players) => {
                                            broadcast_message(
                                                Message::from(
                                                    serde_json::to_string(
                                                        &Players{players}
                                                    ).unwrap()
                                                ), room_id.clone() ,&mut connections
                                            ).await ;
                                        },
                                        Err(err) => {
                                            broadcast_message(
                                                Message::from(
                                                    err
                                                ), room_id.clone() ,&mut connections
                                            ).await ;
                                        }
                                    }

                                }else{
                                    tracing::error!("Not all the players joined the room or it's not the owner who is trying to start the room");
                                    tx.send(Message::from(String::from("Not all the players joined the room or it's not the owner who is trying to start the room"))).unwrap() ;
                                }
                            }else{
                                tracing::warn!("Nothing do with the following request");
                                tx.send(Message::from(String::from("Nothing do with the following request"))).expect("TODO: panic message");
                            }
                        }




                    },
                    Message::Binary(bin) => {
                        tracing::info!("Binary message received : {:?}",bin);
                        tx.send(Message::from(String::from("you have sent the binary message"))).unwrap();
                    },
                    Message::Ping(bin) => {
                        tracing::info!("Ping Message received : {:?}",bin) ;
                        tx.send(Message::from(String::from("you have sent the ping message"))).unwrap();
                    },
                    Message::Pong(bin) => {
                        tracing::info!("Pong Message received : {:?}",bin) ;
                        tx.send(Message::from(String::from("you have sent the pong message"))).unwrap();
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


async fn add_connection(connections: &AppState, room_id: String, user_id: i32, tx: UnboundedSender<Message>) {
    {
        let mut ws_map = connections.websocket_connections.write().unwrap();
        let participants = ws_map.entry(room_id.clone()).or_insert_with(Vec::new);

        // Optional: Remove old connection if already present
        participants.retain(|p| p.user_id != user_id);

        participants.push(ParticipantsConnections {
            user_id,
            connection: tx.clone(),
        });
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



async fn bid_allowance(room_id: String, participant_id: i32, bid_amount: i32,state: &mut AppState) -> bool {
    // min of 18 player need to be buy and max 9 players of foreigneers in that
    // firstly check whether the player was foreigner or not
    let player_id = get_current_player(room_id.clone(), &state.redis_connection).await;
    match player_id {
        Some(player_id) => {
            match is_foreign_player(state, player_id).await{
                Ok(is_foreigener) => {
                    // let's get bid allowance data
                    let (total_players_brought, foreign_players, purse_remaining) = bid_allowance_data(room_id.clone(),participant_id, &state.redis_connection).await ;

                    if is_foreigener && foreign_players >= 9 {
                        false
                    }else{
                        if total_players_brought >= 18 && purse_remaining >= bid_amount {
                            true
                        }else{
                            if purse_remaining <= bid_amount {
                                false
                            }else {
                                let players_to_brought = 18 - total_players_brought ;
                                let amount_needed = players_to_brought * 30 ;
                                let amount_remaining = purse_remaining - bid_amount ;
                                if amount_remaining < amount_needed {
                                    false
                                }else{
                                    true
                                }

                            }

                        }

                    }

                },
                Err(err) => {
                    false
                }
            }
        },
        None => {
            false
        }
    }

}


async fn check_for_intrested_players(room_id: String,user_id: i32,state: &mut AppState) -> bool {

    if is_owner(room_id.clone(),user_id, &state.redis_connection).await {
        if all_teams_16_players(room_id.clone(), &state.redis_connection).await {
            true
        }else{
            false
        }
    }else{
        false
    }
    // it will check whether he is the owner or not and then whether all teams brought 16 players or not
}


async fn send_players(room_id: String, state: &mut AppState) -> Result<Vec<Player>, String> {

    let remaining_players = sqlx::query_as::<_,Player>("SELECT p.*
        FROM players p
        LEFT JOIN sold_players sp ON p.id = sp.player_id
        WHERE sp.player_id IS NULL;
        ")
        .bind(room_id).fetch_all(&state.sql_database).await ;
    tracing::info!("Getting the Reamining Players") ;
    match remaining_players {
        Ok(players) => {
            tracing::info!("we got the players list") ;
            Ok(players)
        },
        Err(err) => {
            tracing::error!("unable to get the players list") ;
            Err(String::from("we are unable to get the players list"))
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