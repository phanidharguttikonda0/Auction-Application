use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub username: String,
    pub mail_id: String,
    pub auctions: Vec<Auction>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Auction {
    pub room_id: String,
    pub team_selected: String,
    pub participant_id: i32
}

#[derive(Debug, Deserialize)]
pub struct Password {
    pub password: String,
}