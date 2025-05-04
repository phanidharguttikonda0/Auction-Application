use axum::extract::{State, Path, Json};
use uuid::Uuid;
use crate::AppState;
use crate::models::rooms::{PlayerSold, PlayerUnsold, RoomCreation, RoomJoin};

pub async fn room_creation(room: RoomCreation, connections: &AppState) -> String {
    // we are going to create the room and also storing it in redis
    "room created".to_string() // here we are going to send a room-id
}


pub async fn room_join(room_join: RoomJoin, connections: &AppState) {
    // adds the joiner to the room in sqlx and redis
    // return room-details
}

pub async fn get_remaining_teams(State(state): State<AppState>, Path(room_id): Path<String>) -> Json<Vec<String>> {
    Json(vec![])
}

pub async fn get_public_rooms(State(state): State<AppState>) -> Json<Vec<String>> {
    Json(vec![])
}


pub async fn get_team(State(state): State<AppState>, Path((room_id, team_name)): Path<(String,String)>) -> Json<Vec<String>> {
    //
    Json(vec![]) // returns players and the amount they have been bought for
}

pub async fn get_teams(State(state): State<AppState>, Path(room_id): Path<String>) -> Json<Vec<String>> {
    Json(vec![]) // return the teams that are participating in this auction-room
}


pub async fn player_sold(player: PlayerSold) -> String {
    String::from("MumbaiIndians") // adding the player to the sqlx and redis, returning the team bought that player
}

pub async fn player_unsold(player: PlayerUnsold) -> bool {
    true // adding to unsold players list
}