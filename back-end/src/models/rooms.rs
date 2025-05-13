use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::players::Player;

#[derive(Debug, Deserialize)]
pub struct RoomCreation { // the data passed to the websocket in the following way
    pub authorization_header: String, // where it contains the username and user-id
    pub max_players: u8,
    pub team: Team,
    pub room_type: RoomType
}

#[derive(Debug, Deserialize,Serialize)]
pub enum RoomStatus {
    WAITING,
    ONGOING,
    FINISHED,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RoomType {
    PRIVATE,
    PUBLIC,
}

#[derive(Debug, Deserialize)]
pub enum Team {
    MUMBAIINDIANS,
    CHENNAISUPERKINGS,
    KOLKATAKINGKNIGHTRIDERS,
    RAJASTHANROYALS,
    GUJARATTITANS,
    SUNRISERSHYDERABAD,
    DELHICAPITALS,
    LUCKNOWSUPERGAINTS,
    PUNJABKINGS,
    ROYALCHALLENGERSBENGALURU
}

#[derive(Debug, Deserialize)]
pub struct RoomJoin { // the data passed to the websocket in this way
    authorization_header: String,
    room_id: Uuid,
    team_selected: Team
}

#[derive(Debug)]
pub struct JoinRoom { // the data passed to the joinRoom handler in this way
    pub room_id: Uuid,
    pub user_id: i32,
    pub team_selected: String,
}

#[derive(Debug)]
pub struct CreateRoom { // the data passed to the create_room handler in this way
    pub room_id: Uuid,
    pub accessibility: String,
    pub max_players: u8,
    pub team_selected: String,
    pub user_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct PlayerSold {
    player_id: i32,
    participant_id: i32,
    amount: u16,
    room_id: String
}

#[derive(Debug, Deserialize)]
pub struct PlayerUnsold {
    player_id: i32,
    room_id: String
}

#[derive(Debug,Serialize)]
pub struct Room { // will be sent for joining or creating a room
    pub room_id: Uuid,
    pub room_type: RoomType,
    pub max_players: u8,
    pub players_teams: Vec<(i32, String)>, // (participant_id, team_name)
    pub status: RoomStatus
}

#[derive(Debug, Serialize)]
pub struct NewJoiner { // will be sent to the rest of room member when a new player joins
    pub team_selected: String,
    pub participant_id: i32,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct TeamPlayer {
    pub player_id: i32,
    pub player_name: String,
    pub amount: i32,
}