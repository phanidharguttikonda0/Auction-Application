use serde::{Deserialize, Serialize};
use validator::Validate;

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

#[derive(Debug, Deserialize, Validate)]
pub struct Password {
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}