
use axum::extract::{State, Path};
use axum::Json;
use crate::AppState;
use crate::models::players::{Country, Player, Stats};

pub async fn get_player(State(state): State<AppState>, Path(player_id): Path<i32>) -> Json<Result<Player,String>> {

    Json(player(&state, player_id).await)

}



pub async fn player(state: &AppState, player_id: i32) -> Result<Player,String> {
    let player = sqlx::query_as::<_,Player>("select * from players where id=($1)")
        .bind(player_id).fetch_one(&state.sql_database).await ;

    match player {
        Ok(player) => {
            tracing::info!("got the player for the give player_id {}", player_id) ;
            (Ok(player))
        },
        Err(err) => {
            tracing::error!("error was {}", err) ;
            (Err(String::from("Invalid player-id")))
        }
    }
}


pub async fn get_stats(State(state): State<AppState>, Path(stats_id): Path<i32>) -> Json<Result<Stats,String>> {
    
    let stats = sqlx::query_as::<_,Stats>("select * from stats where id=($1)")
        .bind(stats_id).fetch_one(&state.sql_database).await ;
    
    match stats { 
        Ok(stats) => {
            tracing::info!("got the stats for the given stats_id {}", stats_id) ;
            Json(Ok(stats))
        },
        Err(err) => {
            tracing::error!("error was {}", err) ;
            Json(Err(String::from("invalid stats-id")))
        }
    }

}


pub async fn is_foreign_player(mut state: &AppState, player_id: i32) -> Result<bool,String> {
    
    let value = sqlx::query_as::<_,Country>("select * from players where id=($1)")
        .bind(player_id).fetch_one(&state.sql_database).await ;
    
    match value {
        Ok(mut country) => {
            country.country = country.country.to_uppercase() ;
            if country.country == "india" {
                Ok(false)
            }else{
                Ok(true)    
            }
            
        },
        Err(err) => {
            tracing::error!("the error while fetching player country , error was {}",err) ;
            Err(String::from("Unable to fetch player"))
        }
    }
    
}