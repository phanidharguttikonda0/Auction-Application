use crate::models::rooms::{RedisRoom, Room};
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
    Room{} // let's update the fields of the room , where add the purse remaining as well
}

pub async fn new_participant(room_id: String, user_id: i32, team: String ,participant_id: i32 ,connection: &Client) -> Result<bool, String>{
    Ok(true)
}