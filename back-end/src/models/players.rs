use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Player{
    pub id: i32,
    pub name: String,
    pub stats_id: i32,
    pub age: i32,
    pub capped: bool,
    pub country: String,
    pub role: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Stats{
    pub id: i32,
    pub matches: Option<i32>,
    pub runs: Option<i32>,
    pub average: Option<f32>,
    pub fifties: Option<i32>,
    pub hundreads: Option<i32>,
    pub wickets: Option<i32>,
    pub strike_rate: Option<i32>,
    pub five_wickets: Option<i32>,
    pub three_wickets: Option<i32>,
}