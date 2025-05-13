use std::os::linux::raw::stat;
use axum::{extract::{State, Path, Json}, http::StatusCode};
use std::collections::HashMap;
use sqlx::{Error, Postgres, Transaction};
use uuid::Uuid;
use crate::AppState;
use crate::middlewares::authentication::authorization_decode;
use crate::models::rooms::{CreateRoom, JoinRoom, PlayerSold, PlayerUnsold, Room, RoomCreation, RoomJoin, RoomType, Team, TeamPlayer};

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

pub async fn get_teams(State(state): State<AppState>, Path(room_id): Path<String>) -> Json<Result<Vec<String>, String>> {

    let teams = sqlx::query_scalar::<_,String>("select team_selected from participants where room_id=($1)")
        .bind(&Uuid::parse_str(&room_id).unwrap()).fetch_all(&state.sql_database).await ;

    match teams {
        Ok(teams) => {
            tracing::info!("successfully fetched teams for the room {}",room_id) ;
            Json(Ok(teams))
        },
        Err(err) => {
            tracing::error!("error was {}", err) ;
            Json(Err(String::from("unable to fetch teams")))
        }
    }

}

pub async fn get_public_rooms(State(state): State<AppState>) -> Json<Result<Vec<String>, String>> {

   let public_rooms = sqlx::query_scalar::<_,String>("select id from rooms where accessibility='PUBLIC' AND room_status='WAITING'")
       .fetch_all(&state.sql_database).await ;

    match public_rooms {
        Ok(public_rooms) => {
            tracing::info!("successfully fetched public rooms") ;
            Json(Ok(public_rooms))
        },
        Err(err) => {
            tracing::warn!("unable to get public rooms because of  {}", err) ;
            Json(Err(String::from("unable to fetch public rooms")))
        }
    }
}


pub async fn get_team(State(state): State<AppState>, Path((room_id, team_name)): Path<(String,String)>) -> Json<Result<Vec<TeamPlayer>,String>> {

    let players =
        sqlx::query_as::<_, TeamPlayer>("select s.player_id,s.amount,pp.name from sold_players s INNER JOIN participants p on s.participant_id=p.id INNER JOIN players pp on pp.id=s.player_id where p.room_id=($1) and p.team_selected=($2)")
        .bind(Uuid::parse_str(&room_id).unwrap()).bind(&team_name).fetch_all(&state.sql_database).await ;

    match players {
        Ok(players) => {
            tracing::info!("Successfully got the players for the team {}",team_name) ;
            Json(Ok(players))
        },
        Err(err) => {
            tracing::warn!("player not found out-put was : {}",err) ;
            Json(Err(String::from("Unable to get the players for the specific team")))
        }
    }


}

fn get_pool_mapping(key: &str) -> Option<(i32, &'static str)> {
    match key {
        "A" => Some((200, "BAT")),
        "B" => Some((200, "BOWL")),
        "C" => Some((200, "ALL")),
        "D" => Some((150, "BAT")),
        "E" => Some((150, "BOWL")),
        "F" => Some((150, "ALL")),
        "G" => Some((100, "BAT")),
        "H" => Some((100, "BOWL")),
        "I" => Some((100, "ALL")),
        "J" => Some((75, "BAT")),
        "K" => Some((75, "BOWL")),
        "L" => Some((75, "ALL")),
        "M" => Some((50, "BAT")),
        "N" => Some((50, "BOWL")),
        "O" => Some((50, "ALL")),
        "P" => Some((40, "BAT")),
        "Q" => Some((40, "BOWL")),
        "R" => Some((40, "ALL")),
        "S" => Some((30, "BAT")),
        "T" => Some((30, "BOWL")),
        "U" => Some((30, "ALL")),
        _ => None,
    }
}




pub async fn get_pool(State(state): State<AppState>, Path(pool_id): Path<String>) -> Json<Vec<String>> {

    let (amount, pool_type) = get_pool_mapping(&pool_id).unwrap();
    // from this we are going to fetch the players with player_id and name and their role and base_price
    Json(vec![]) // returning player-names along with their id's and stats id
}


pub async fn player_sold(player: PlayerSold) -> String {
    String::from("MumbaiIndians") // adding the player to the sqlx and redis, returning the team bought that player
}

pub async fn player_unsold(player: PlayerUnsold) -> bool {
    true // adding to unsold players list
}