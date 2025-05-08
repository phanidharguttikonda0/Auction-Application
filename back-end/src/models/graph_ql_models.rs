use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Batsman {
    pub player_id: i32,
    pub player_name: String,
}

#[derive(SimpleObject)]
pub struct Bowlers{
    pub player_id: i32,
    pub player_name: String,
}

#[derive(SimpleObject)]
pub struct AllRounders{
    pub player_id: i32,
    pub player_name: String,
}

#[derive(SimpleObject)]
pub struct All{
    pub player_id: i32,
    pub player_name: String,
    pub role: String
}

#[derive(SimpleObject)]
pub struct UnsoldBatsmans {
    pub batsman: Vec<Batsman>,
}

#[derive(SimpleObject)]
pub struct UnsoldBowlers {
    pub bowlers: Vec<Bowlers>
}

#[derive(SimpleObject)]
pub struct UnsoldAllRounders {
    pub all_rounders: Vec<AllRounders>
}

#[derive(SimpleObject)]
pub struct UnsoldPlayers{
    pub players: Vec<All>
}




/*

*/