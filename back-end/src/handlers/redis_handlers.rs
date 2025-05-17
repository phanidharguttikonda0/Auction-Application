use crate::models::rooms::{RedisRoom, Room, RoomStatus, RoomType};
use redis::Client;
use uuid::Uuid;

pub async fn redis_room_creation(room: Room, connection: &Client) {


}

pub async fn participant_exists(user_id: i32, room_id: String, connection: &Client) -> (bool, i32){
    (true, 1) // (that is participant-id)
}

pub async fn room_exists(room_id: i32, connection: &Client) -> bool{
    true
}

pub async fn is_in_waiting(room_id: String, connection: &Client) -> bool{
    true
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

pub async fn new_participant(room_id: String, user_id: i32, team: String ,participant_id: i32 ,connection: &Client) -> Result<bool, String>{
    Ok(true)
}


pub async fn new_bid(room_id: String, participant_id: i32, bid: i32, connection: &Client) -> Result<String, String>{
    Ok(String::from("Team Name to be returned"))
}


pub async fn check_for_ready(room_id: String, connection: &Client) -> bool{
    // if all the participants are max, then returns true and also room to be in waiting state intially and changes to ongoing state
    true
}

pub async fn add_intrested_players(intrested_players: Vec<i32>, room_id: String, connection: &Client) -> Result<String, String>{
    Ok(String::from("Team Name to be returned"))
}