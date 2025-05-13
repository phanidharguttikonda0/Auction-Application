use axum::{extract::{State, Path, Json}, http::StatusCode};
use sqlx::{Error, Postgres, Transaction};
use uuid::Uuid;
use crate::AppState;
use crate::middlewares::authentication::authorization_decode;
use crate::models::rooms::{CreateRoom, JoinRoom, PlayerSold, PlayerUnsold, Room, RoomCreation, RoomJoin, RoomType, Team};

pub async fn room_creation(room: CreateRoom,connections: &AppState) -> Result<Uuid,String> {


    let room_id = sqlx::query_scalar::<_,Uuid>("insert into rooms (max_participants,room_status,accessibility,owner_id,) values ($1,$2,$3,$4) returning id)")
        .bind(room.max_players as i32).bind("WAITING").bind(room.accessibility).bind(room.user_id)
        .fetch_one(&connections.sql_database).await ;

    match room_id {
        Ok(room_id) => {
            tracing::info!("created the room successfully") ;
            Ok(room_id)
        },
        Err(err) => {
            tracing::error!("Error Occured {}", err) ;
            Err(String::from("Unable to create room, Make sure You didn't have a room with waiting or ongoing status created by the user"))
        }
    }

}

fn get_team_name(team:Team) -> String {
    match team {
        Team::MUMBAIINDIANS => String::from("MUMBAIINDIANS"),
        Team::CHENNAISUPERKINGS => String::from("CHENNAISUPERKINGS"),
        Team::DELHICAPITALS => String::from("DELHICAPITALS"),
        Team::GUJARATTITANS => String::from("GUJARATTITANS"),
        Team::KOLKATAKINGKNIGHTRIDERS => String::from("KOLKATAKINGKNIGHTRIDERS"),
        Team::LUCKNOWSUPERGAINTS => String::from(""),
        Team::PUNJABKINGS => String::from("PUNJABKINGS"),
        Team::RAJASTHANROYALS => String::from("RAJASTHANROYALS"),
        Team::SUNRISERSHYDERABAD => String::from("SUNRISERSHYDERABAD"),
        Team::ROYALCHALLENGERSBENGALURU => String::from("ROYALCHALLENGERSBENGALURU"),
    }
}

fn get_room_type(room: RoomType) -> String {
    match room {
        RoomType::PUBLIC => String::from("PUBLIC"),
        RoomType::PRIVATE => String::from("PRIVATE"),
    }
}

pub async fn room_join(room_join: JoinRoom, connections: &AppState) -> Result<i32,String> { // participant_id

    let value = sqlx::query_scalar::<_,i32>("insert into participants (room_id, participant_id,team_selected) values ($1,$2,$3) returning id")
        .bind(room_join.room_id).bind(room_join.user_id).bind(room_join.team_selected)
        .fetch_one(&connections.sql_database).await ;
    match value {
        Ok(participant_id) => {
            tracing::info!("successfully joined the room") ;
            Ok(participant_id)
        },
        Err(err) => {
            tracing::error!("error was {}", err) ;
            Err(err.to_string())
        }
    }
}

pub async fn get_remaining_teams(State(state): State<AppState>, Path(room_id): Path<String>) -> Json<Vec<String>> {
    Json(vec![])
}

pub async fn get_public_rooms(State(state): State<AppState>) -> Json<Vec<String>> {
    Json(vec![])
}


pub async fn get_team(State(state): State<AppState>, Path((room_id, team_name)): Path<(String,String)>) -> Json<Vec<String>> {
    //
    Json(vec![]) // returns players and the amount they have been bought for
}

pub async fn get_teams(State(state): State<AppState>, Path(room_id): Path<String>) -> Json<Vec<String>> {
    Json(vec![]) // return the teams that are participating in this auction-room
}


pub async fn get_pool(State(state): State<AppState>, Path(pool_id): Path<i32>) -> Json<Vec<String>> {
    Json(vec![]) // returning player-names along with their id's and stats id
}


pub async fn player_sold(player: PlayerSold) -> String {
    String::from("MumbaiIndians") // adding the player to the sqlx and redis, returning the team bought that player
}

pub async fn player_unsold(player: PlayerUnsold) -> bool {
    true // adding to unsold players list
}