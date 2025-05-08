
use axum::extract::{State,Path};
use axum::Json;
use crate::AppState;
use crate::models::players::{Player, Stats};

pub async fn get_player(State(state): State<AppState>, Path(player_id): Path<i32>) -> Json<Vec<Player>> {
    Json(vec![])
}

pub async fn get_stats(State(state): State<AppState>, Path(stats_id): Path<i32>) -> Json<Vec<Stats>> {
    Json(vec![])
}