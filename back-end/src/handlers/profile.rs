
use axum::extract::{State,Path, Request};
use axum::{Form, Json};

use crate::models::profile::{Password, Profile};

pub async fn profile(State(state): State<crate::AppState>,req: Request) -> Json<Profile> {
    // from authorization header we will get the username and user-id
    Json(get_user_profile("".to_string(), 0).await)
}

pub async fn search(State(state): State<crate::AppState>, Path(username): Path<String>) -> Json<Vec<(String, i32)>> {
    Json(vec![]) // where username and user-id we are going to return
}

pub async fn reset_password(State(state): State<crate::AppState>, Form(password):Form<Password>) -> Json<bool> {
    Json(true)
}

pub async fn get_profile(State(state): State<crate::AppState>, Path(username): Path<String>) -> Json<Profile> {
    Json(get_user_profile("".to_string(), 0).await)
}


async fn get_user_profile(username: String, user_id: i32) -> Profile {
    Profile{ auctions: vec![], mail_id: String::from(""), username: String::from("") }
}