use axum::extract::ws::Message;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
pub struct RoomCreation { // the data passed to the websocket in the following way
    pub authorization_header: String, // where it contains the username and user-id
    pub max_players: u8,
    pub team: Team,
    pub room_type: RoomType
}

#[derive(Debug, Deserialize,Serialize, Clone)]
pub enum RoomStatus {
    WAITING,
    ONGOING,
    FINISHED,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub enum RoomType {
    PRIVATE,
    PUBLIC,
}

#[derive(Debug, Deserialize,Clone)]
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
    pub(crate) authorization_header: String,
    pub(crate) room_id: Uuid,
    pub(crate) team_selected: Team
}

#[derive(Debug)]
pub struct JoinRoom { // the data passed to the joinRoom handler in this way
    pub room_id: Uuid,
    pub user_id: i32,
    pub team_selected: String,
}

#[derive(Debug)]
pub struct CreateRoom { // the data passed to the create_room handler in this way
    pub accessibility: String,
    pub max_players: u8,
    pub team_selected: String,
    pub user_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct PlayerSold {
    pub player_id: i32,
    pub participant_id: i32,
    pub amount: i32,
    pub room_id: Uuid
}

#[derive(Debug, Deserialize)]
pub struct PlayerUnsold {
    pub player_id: i32,
    pub room_id: Uuid
}

#[derive(Debug,Serialize, Clone)]
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
    pub user_id: i32
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct TeamPlayer {
    pub id: i32,
    pub name: String,
    pub role: String,
    pub amount: i32,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct PoolPlayer{
    pub id: i32,
    pub name: String,
    pub role: String,
    pub base_price: i32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentBid {
    pub participant_id: i32,
    pub amount: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bid {
    pub participant_id: i32,
    pub amount: i32,
    pub room_id: String
}

#[derive(Debug, Serialize)]
pub struct RedisRoom { // redis storing room-data
    pub current_bid:CurrentBid,
    pub current_player: Option<i32>, // player-id
    pub go_with_intrested: bool,
    pub max_participants: u8,
    pub participants: Vec<(i32,i32,String)>, // (user_id, participant_id, team_name)
    pub purse_remaining: Vec<(i32,i32)>, //(participant_id, amount)
    pub players_bought: Vec<(i32,i32,i32)>, //(participant_id, players_brought, foriegn_players
    pub room_status: RoomStatus,
    pub intrested_players: HashSet<i32>, // player-id will be stored (no duplicates will be stored)
} // while sending intrested-players , it will check whether the player was sold or not

/*
room_id : RedisRoom
*/

#[derive(Debug)]
pub struct ParticipantsConnections {
    pub participant_id: i32,
    pub connection: UnboundedSender<Message>
}

#[derive(Debug, Serialize)]
pub struct BidReturn {
    pub team: String,
    pub amount: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntrestedPlayers {
    pub room_id: String,
    pub players: Vec<i32>
}