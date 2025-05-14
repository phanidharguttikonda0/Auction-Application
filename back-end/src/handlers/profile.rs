
use axum::extract::{State,Path, Request};
use axum::{Form, Json};

use crate::models::profile::{Password, Profile};

pub async fn profile(State(state): State<crate::AppState>,req: Request) -> Json<Profile> {
    // from authorization header we will get the username and user-id
    Json(get_user_profile("".to_string(), 0).await)
}

pub async fn search(State(state): State<crate::AppState>, Path(username): Path<String>) -> Json<Result<Vec<(String, i32)>, String>> {
    let pattern = format!("{}%", username) ;
    let usernames = sqlx::query_as::<_,(String, i32)>("select username, id from users where username LIKE ($1) LIMIT 5")
        .bind(pattern).fetch_all(&state.sql_database).await ;

    match usernames {
        Ok(users) => {
            tracing::info!("got the usernames for users starting with usernames {}",username) ;
            Json(Ok(users))
        },
        Err(err) => {
            tracing::error!("error was {}",err) ;
            Json(Err(String::from("No user found")))
        }
    }


}

pub async fn reset_password(State(state): State<crate::AppState>, Form(password):Form<Password>) -> Json<bool> {
    // passing direct new password
    Json(true)
}

pub async fn get_profile(State(state): State<crate::AppState>, Path(username): Path<String>) -> Json<Profile> {
    Json(get_user_profile("".to_string(), 0).await)
}


async fn get_user_profile(username: String, user_id: i32) -> Profile {
    Profile{ auctions: vec![], mail_id: String::from(""), username: String::from("") }
}