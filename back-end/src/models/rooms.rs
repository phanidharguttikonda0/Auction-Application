use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RoomCreation {
    authorization_header: String, // where it contains the username and user-id
    max_players: u8,
    team: Team,
    room_type: RoomType
}

#[derive(Debug, Deserialize)]
pub enum RoomStatus {
    Waiting,
    Started,
    Finished
}

#[derive(Debug, Deserialize)]
pub enum RoomType {
    PRIVATE,
    PUBLIC,
}

#[derive(Debug, Deserialize)]
pub enum Team {
    MumbaiIndians,
    ChennaiSuperKings,
    KolkataKingKnightRiders,
    RajasthanRoyals,
    GujaratTitans,
    SunrisersHyderabad,
    DelhiCapitals,
    LucknowSuperGiants,
    PunjabKings,
    RoyalChallengersBangalore
}

#[derive(Debug, Deserialize)]
pub struct RoomJoin {
    authorization_header: String,
    room_id: String,
    team_selected: Team
}

#[derive(Debug, Deserialize)]
pub struct PlayerSold{
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