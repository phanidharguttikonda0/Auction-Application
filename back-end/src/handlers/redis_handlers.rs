use std::collections::HashSet;
use crate::models::rooms::{CurrentBid, PlayerSold, RedisRoom, Room, RoomStatus, RoomType};
use redis::{Client, RedisResult};
use redis::aio::MultiplexedConnection;
use uuid::Uuid;
use redis::AsyncCommands;

pub async fn redis_room_creation(room: Room, owner_id: i32,client: &Client) {
    // we need to create a new room in redis
    let mut connection = client.get_multiplexed_tokio_connection().await.unwrap();  // Get actual connection
    let participants = vec![(owner_id,room.players_teams[0].0, room.players_teams[0].1.clone())];
    let players_bought = vec![(room.players_teams[0].0,0,0)] ;
    let purse_remaining = vec![(room.players_teams[0].0,0)] ;
    let room_id = "room123"; // Replace with actual room.room_id
    let redis_room = RedisRoom {
        room_status: room.status,
        intrested_players: HashSet::new(),
        current_bid: None,
        current_player: None,
        go_with_intrested: false,
        owner_id,
        max_participants: room.max_players,
        participants,
        players_bought,
        purse_remaining,
    };

    // Serialize your struct to JSON if needed
    let serialized = serde_json::to_string(&redis_room).unwrap();
    tracing::info!("successfully going to set the new room in redis") ;
    let _: () = connection.set(Uuid::to_string(&room.room_id), serialized).await.unwrap();  // Store JSON string

}

pub async fn participant_exists(user_id: i32, room_id: String, connection: &Client) -> (bool, i32){
    let mut connection = match connection.get_multiplexed_tokio_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get Redis connection: {}", e);
            return (false, -1);
        }
    };

    let room: String = match connection.get(&room_id).await {
        Ok(val) =>  val,
        Err(err ) => {
            tracing::error!("error was : {}",err) ;
            return (false, -1) ;
        }
    } ;

    let redis_room: RedisRoom = match serde_json::from_str(&room) {
        Ok(data) => data,
        Err(err) => {
            tracing::error!("error was : {}",err) ;
            return (false, -1) ;
        }
    };
    let mut participants = redis_room.participants;

    for participant in participants.iter(){
        if participant.0 == user_id{
            return (true, participant.1) ;
        }
    }

    (false, -1)
}

pub async fn room_exists(room_id: i32, connection: &Client) -> bool{
    let mut connection = match connection.get_multiplexed_tokio_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get Redis connection: {}", e);
            return false ;
        }
    };

    let room: String = match connection.get(&room_id).await {
        Ok(val) =>  val,
        Err(err ) => {
            tracing::error!("error was : {}",err) ;
            return false ;
        }
    } ;

    true
}

pub async fn is_in_waiting(room_id: String, connection: &Client) -> bool{
    let mut connection = match connection.get_multiplexed_tokio_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get Redis connection: {}", e);
            return false ;
        }
    };

    let room: String = match connection.get(&room_id).await {
        Ok(val) =>  val,
        Err(err ) => {
            tracing::error!("error was : {}",err) ;
            return false ;
        }
    } ;

    let redis_room: RedisRoom = match serde_json::from_str(&room) {
        Ok(data) => data,
        Err(err) => {
            tracing::error!("error was : {}",err) ;
            return false ;
        }
    } ;

    if redis_room.max_participants != redis_room.participants.len() as u8{
        true
    }else{
        false
    }
}


pub async fn get_Room(room_id: String, connection: &Client) -> Room {
    Room {
        room_id: Uuid::parse_str(&room_id).unwrap(),
        room_type: RoomType::PUBLIC,
        max_players: 10,
        players_teams: Vec::new(),
        status: RoomStatus::WAITING,
    }  // let's update the fields of the room, where add the purse remaining as well
}

pub async fn new_participant(
    room_id: String,
    user_id: i32,
    team: String,
    participant_id: i32,
    connection: &redis::Client,
) -> Result<bool, String> {
    // Get the Redis connection
    let mut connection = match connection.get_multiplexed_tokio_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get Redis connection: {}", e);
            return Err(format!("Redis connection error: {}", e));
        }
    };

    // Get the room data from Redis
    let room: String = match connection.get(&room_id).await {
        Ok(data) => data,
        Err(e) => {
            tracing::error!("Failed to get room data for room_id {}: {}", room_id, e);
            return Err(format!("Failed to get room data: {}", e));
        }
    };

    // Deserialize the RedisRoom
    let mut redis_room: RedisRoom = match serde_json::from_str(&room) {
        Ok(room) => room,
        Err(e) => {
            tracing::error!("Failed to deserialize RedisRoom for room_id {}: {}", room_id, e);
            return Err(format!("Deserialization error: {}", e));
        }
    };

    tracing::info!("Successfully deserialized RedisRoom for room_id {}", room_id);
    // Update the RedisRoom fields
    redis_room.participants.push((participant_id, user_id, team));
    redis_room.players_bought.push((participant_id, 0, 0));
    redis_room.purse_remaining.push((participant_id, 0));
    tracing::info!("Successfully updated RedisRoom for room_id {}", room_id);
    // Serialize the updated room
    let serialized = match serde_json::to_string(&redis_room) {
        Ok(json) => json,
        Err(e) => {
            tracing::error!("Failed to serialize RedisRoom for room_id {}: {}", room_id, e);
            return Err(format!("Serialization error: {}", e));
        }
    };
    let l: RedisResult<()> = connection.set(&room_id, serialized).await ;
    // Save the updated room back to Redis
    match  l {
        Ok(_) => Ok(true),
        Err(e) => {
            tracing::error!("Failed to update room in Redis for room_id {}: {}", room_id, e);
            Err(format!("Redis update error: {}", e))
        }
    }
}


pub async fn get_room_string(room_id: String, connection: &Client) -> Result<(String, MultiplexedConnection), String>{
    let mut connection = match connection.get_multiplexed_tokio_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get Redis connection: {}", e);
            return Err(format!("Redis connection error: {}", e));
        }
    };

    // Get the room data from Redis
    let room: String = match connection.get(&room_id).await {
        Ok(data) => data,
        Err(e) => {
            tracing::error!("Failed to get room data for room_id {}: {}", room_id, e);
            return Err(format!("Failed to get room data: {}", e));
        }
    };

    Ok((room, connection))
}

pub async fn new_bid(room_id: String, participant_id: i32, bid: i32, connection: &Client) -> Result<String, String>{

    match get_room_string(room_id.clone(), connection).await {
        Ok(mut data) => {
            let mut redis_room: RedisRoom = match serde_json::from_str(&data.0) {
                Ok(data) => data,
                Err(e) => {
                    tracing::error!("Failed to deserialize RedisRoom for room_id {}: {}", room_id, e);
                    return Err(format!("Deserialization error: {}", e));
                }
            } ;

            let team_name = redis_room.participants.iter().find(|x| x.1 == participant_id).unwrap().2.clone();
            redis_room.current_bid = Some(CurrentBid{ amount: bid, participant_id }) ;

            let serialized = match serde_json::to_string(&redis_room) {
                Ok(json) => json,
                Err(e) => {
                    tracing::error!("Failed to serialize RedisRoom for room_id {}: {}", room_id, e);
                    return Err(format!("Serialization error: {}", e));
                }
            } ;
            let l: RedisResult<()> = data.1.set(&room_id, serialized).await ;
            match l {
                Ok(_) => {
                    tracing::info!("Successfully updated RedisRoom for room_id {}", room_id);
                    Ok(team_name)
                },
                Err(e) => {
                    tracing::error!("Failed to update room in Redis for room_id {}: {}", room_id, e);
                    Err(format!("Redis update error: {}", e))
                }
            }


        },
        Err(e) => {
            tracing::error!("Failed to get room data for room_id {}: {}", room_id, e);
            Err(e)
        }
    }

}


pub async fn check_for_ready(room_id: String, connection: &Client) -> bool{
    // if all the participants are max, then returns true and also room to be in waiting state intially and changes to ongoing state
    if !is_in_waiting(room_id.clone(), connection).await {
        
        match get_room_string(room_id.clone(), connection).await {
            Ok(mut data) => {
                let mut redis_room: RedisRoom = match serde_json::from_str(&data.0) {
                    Ok(data) => data,
                    Err(e) => {
                        tracing::error!("Failed to deserialize RedisRoom for room_id {}: {}", room_id, e);
                        return false ;
                    }
                } ;
                
                redis_room.room_status = RoomStatus::ONGOING;
                let serialize = serde_json::to_string(&redis_room).unwrap();
                let l : RedisResult<()> = data.1.set(&room_id, serialize).await;
                match l { 
                    Ok(_) => true,
                    Err(e) => {
                        tracing::error!("Failed to update room in Redis for room_id {}: {}", room_id, e);
                        false 
                    }
                }
            },
            Err(e) => {
                tracing::error!("Failed to get room data for room_id {}: {}", room_id, e);
                 false 
            }
        }
    } else { 
        false
    }
    
}

pub async fn add_intrested_players(intrested_players: Vec<i32>, room_id: String,user_id: i32, connection: &Client) -> Result<String, String> {
    match get_room_string(room_id.clone(), connection).await {
        Ok(mut data) =>{

            let mut redis_room: RedisRoom = match serde_json::from_str(&data.0) {
                Ok(data) => data,
                Err(e) => {
                    return Err(String::from("Error in deserializing room data"))
                }
            } ;
            let mut intrested_players_set = redis_room.intrested_players;
            for player in intrested_players {
                intrested_players_set.insert(player);
            }

            let team_name = redis_room.participants.iter().find(|x| x.0 == user_id).unwrap().2.clone();

            redis_room.intrested_players = intrested_players_set;
            let serialized = match serde_json::to_string(&redis_room) {
                Ok(json) => json,
                Err(e) => {
                    return Err(String::from("Error in serializing room data"))
                }
            } ;
            let l: RedisResult<()> = data.1.set(&room_id, serialized).await ;
            match l {
                Ok(_) => {
                    Ok(team_name)
                },
                Err(e) => {
                    return Err(String::from("Error in updating room in Redis"))
                }
            }
        },
        Err(err) => {
            return Err(String::from("Error in getting room data"))
        }
    }

}

pub async fn sell_player(room_id: String, connection: &Client) -> Result<PlayerSold, String>{
    // it cleans up the last bid and sends the last bid participant_id and amount sold for

    match get_room_string(room_id.clone(), connection).await {
        Ok(mut data) => {
            let mut redis_room: RedisRoom = match serde_json::from_str(&data.0) {
                Ok(data) => data,
                Err(e) => {
                    return Err(String::from("Error in deserializing room data"))
                }
            } ;
            let current_bid = redis_room.current_bid.unwrap();
            let current_player = redis_room.current_player.unwrap();
            redis_room.current_bid = None;
            redis_room.current_player = None;
            let serialized = match serde_json::to_string(&redis_room) {
                Ok(json) => json,
                Err(e) => {
                    return Err(String::from("Error in serializing room data"))
                }
            } ;
            let l: RedisResult<()> = data.1.set(&room_id, serialized).await ;
            match l {
                Ok(_) => {
                    Ok(PlayerSold {
                        player_id: current_player,
                        room_id: Uuid::parse_str(&room_id).unwrap(),
                        participant_id: current_bid.participant_id,
                        amount: current_bid.amount
                    })
                },
                Err(err) => {
                    Err(String::from("Error in getting room data"))
                }
            }
        },
        Err(err) => {
            tracing::error!("Failed to get room data for room_id {}: {}", room_id, err);
            Err(String::from("Error in getting room data"))
        }
    }

}

pub async fn next_player(room_id: String, player_id: i32,connection: &Client) -> bool {
    match get_room_string(room_id.clone(), connection).await {
        Ok(mut data) => {
            let mut redis_room: RedisRoom = match serde_json::from_str(&data.0) {
                Ok(data) => data,
                Err(e) => {
                    return false ;
                }
            } ;
            redis_room.current_player = Some(player_id);
            let serialized = match serde_json::to_string(&redis_room) {
                Ok(data) => data,
                Err(e) => {
                    return false ;
                }
            } ;
            let l: RedisResult<()> = data.1.set(&room_id, serialized).await ;
            match l {
                Ok(_) => true,
                Err(e) => {
                    tracing::error!("Failed to update room in Redis for room_id {}: {}", room_id, e);
                    false
                }
            }
        },
        Err(err) => {
            tracing::error!("Failed to get room data for room_id {}: {}", room_id, err);
            false 
        }
    }
}


pub async fn player_from_redis(room_id: String, connection: &Client) -> Result<i32, String> {

    match get_room_string(room_id.clone(), connection).await {
        Ok(mut data) => {
            let room: RedisRoom = match serde_json::from_str(&data.0) {
                Ok(data) => data,
                Err(e) => {
                    return Err(String::from("Error in deserializing room data"))
                }
            } ;
            let mut next_player = room.intrested_players;
            for player in next_player.iter() {
                let player_id = player.clone() ;
                next_player.remove(&player_id);
                return Ok(player_id) ;
            }
            Ok(-1)
        },
        Err(err) => {
            return Err(String::from("Error in getting room data"))
        }
    }

}


pub async fn intrested_players_set(room_id: String, connection: &Client) -> bool {
    match get_room_string(room_id.clone(), connection).await {
        Ok(mut data) => {
            let mut redis_room: RedisRoom = match serde_json::from_str(&data.0) {
                Ok(data) => data,
                Err(e) => {
                    return false ;
                }
            } ;
            if redis_room.intrested_players.len() == 0 {
                false
            }else{
                true
            }
        },
        Err(err) => {
            return false ;
        }
    }
}
