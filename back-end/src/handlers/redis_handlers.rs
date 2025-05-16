use crate::models::rooms::Room;
use redis::Client;
pub async fn redis_room_creation(room: Room, connection: &Client) {}