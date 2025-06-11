use std::os::linux::raw::stat;
use axum::{extract::{State, Path, Json}, http::StatusCode, Form};
use std::collections::HashMap;
use std::str::FromStr;
use sqlx::{Error, Postgres, Transaction};
use uuid::Uuid;
use crate::AppState;
use crate::middlewares::authentication::authorization_decode;
use crate::models::rooms::{CreateRoom, JoinRoom, PlayerSold, PlayerUnsold, PoolPlayer, Room, RoomCreation, RoomJoin, RoomStatus, RoomType, Team, TeamPlayer};

pub async fn create_room(room: CreateRoom,connection: &mut Transaction<'_, Postgres>) -> Result<Uuid,String> {


    let room_id = sqlx::query_scalar::<_,Uuid>("insert into rooms (max_participants,room_status,accessibility,owner_id) values ($1,$2,$3,$4) returning id")
        .bind(room.max_players as i32).bind("WAITING").bind(room.accessibility).bind(room.user_id)
        .fetch_one(&mut **connection).await ;

    match room_id {
        Ok(room_id) => {
            tracing::info!("created the room successfully") ;
            Ok(room_id)
        },
        Err(err) => {
            tracing::error!("Error Occured {}", err) ;
            Err(String::from("Unable to create room, Make sure You didn't have a room with waiting or ongoing status created by the Owner"))
        }
    }

}

pub fn get_team_name(team:Team) -> String {
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

pub fn get_room_type(room: RoomType) -> String {
    match room {
        RoomType::PUBLIC => String::from("PUBLIC"),
        RoomType::PRIVATE => String::from("PRIVATE"),
    }
}

pub async fn join_room(room_join: JoinRoom, connection: &mut Transaction<'_, Postgres>) -> Result<i32,String> { // participant_id

    let value = sqlx::query_scalar::<_,i32>("insert into participants (room_id, participant_id,team_selected) values ($1,$2,$3) returning id")
        .bind(room_join.room_id).bind(room_join.user_id).bind(room_join.team_selected)
        .fetch_one(&mut **connection).await ;
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

    // first we need to see the redis to get the data , if redis doesn't exists then we need to fetch from the psql

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

pub async fn get_public_rooms(State(state): State<AppState>) -> Json<Result<Vec<(Uuid,i32)>, String>> {

   let public_rooms = sqlx::query_scalar::<_,(Uuid,i32)>("select id,max_participants from rooms where accessibility='PUBLIC' AND room_status='WAITING'")
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

    // first we need to see the redis to get the data , if redis doesn't exists then we need to fetch from the psql


    let players =
        sqlx::query_as::<_, TeamPlayer>("select s.player_id,s.amount,pp.name,pp.role from sold_players s INNER JOIN participants p on s.participant_id=p.id INNER JOIN players pp on pp.id=s.player_id where p.room_id=($1) and p.team_selected=($2)")
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




pub async fn get_pool(State(state): State<AppState>, Path(pool_id): Path<String>) -> Json<Result<Vec<PoolPlayer>,String>> {

    let (amount, pool_type) = get_pool_mapping(&pool_id).unwrap();
    // from this we are going to fetch the players with player_id and name and their role and base_price
    let players = sqlx::query_as::<_,PoolPlayer>("select id,name,role,base_price from players where base_price=($1) AND role=($2)")
        .bind(amount).bind(pool_type).fetch_all(&state.sql_database).await ;

    match players {
        Ok(players) => {
            tracing::info!("Got the players for the pool_id of {}", pool_id) ;
            Json(Ok(players))
        },
        Err(err) => {
            tracing::error!("Error was : {}",err) ;
            Json(Err(String::from("Invalid pool_id pools are available from A to U")))
        }
    }

}


pub async fn player_sold(player: PlayerSold, connections: &AppState) -> String {

    let sold = sqlx::query_scalar::<_, String>(
        "
    WITH inserted AS (
        INSERT INTO sold_players (participant_id, player_id, amount, room_id)
        VALUES ($1, $2, $3, $4)
        RETURNING participant_id
    )
    SELECT p.team_selected
    FROM inserted
    JOIN participants p ON inserted.participant_id = p.id
    "
    )
        .bind(player.participant_id)
        .bind(player.player_id)
        .bind(player.amount)
        .bind(player.room_id)
        .fetch_one(&connections.sql_database)
        .await;


    match sold {
        Ok(sold_team) => {
            tracing::info!("solded to {}",sold_team) ;
            sold_team
        },
        Err(err) => {
            tracing::error!("error was {}",err) ;
            String::from("")
        }
    }

}

pub async fn player_unsold(player: PlayerUnsold, connections: &AppState) -> bool {

    let unsold = sqlx::query("insert into unsold_players (player_id, room_id) values ($1,$2)")
        .bind(player.player_id).bind(player.room_id).execute(&connections.sql_database).await ;

    match unsold {
        Ok(done) => {
            if done.rows_affected() > 0 {
                tracing::info!("added to unsold players successfully") ;
                true
            }else {
                tracing::warn!("rows doesn't effected") ;
                false
            }
            
        },
        Err(err) => {
            tracing::error!("error was {}",err) ;
            false
        }
    }

}



pub async fn change_room_status(state: &AppState, room_id: String, room_status: RoomStatus) -> bool {
    tracing::info!("{}",Uuid::from_str(&room_id).unwrap()) ;
    let status = sqlx::query("update rooms set room_status=$1 where id=$2")
        .bind(match room_status {
            RoomStatus::WAITING => "WAITING",
            RoomStatus::ONGOING => "ONGOING",
            RoomStatus::FINISHED => "FINISHED"
        }).bind(&Uuid::from_str(&room_id).unwrap()).execute(&state.sql_database).await ;

    match status{ 
        Ok(status) => {
            if status.rows_affected() > 0 {
                tracing::info!("rows effected while changing the room_status") ;
                true
            }else{
                tracing::info!("rows not effected while changing room_status") ;
                false
            }
        },
        Err(err) => {
            tracing::error!("error which changing room_status was {}",err) ;
            false
        }
    }

}