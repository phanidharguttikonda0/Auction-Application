use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Player{
    pub id: i32,
    pub name: String,
    pub stats_id: i32,
    pub dob: NaiveDate,
    pub country: String,
    pub base_price: i32,
    pub role: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Stats{
    pub id: i32,
    pub matches: Option<i32>,
    pub runs: Option<i32>,
    pub average: Option<f64>,
    pub fifties: Option<i32>,
    pub hundreads: Option<i32>,
    pub wickets: Option<i32>,
    pub strike_rate: Option<f64>,
    pub five_wickets: Option<i32>,
    pub three_wickets: Option<i32>,
    stats_from: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Country {
    pub country: String,
}