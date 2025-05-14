
use axum::extract::{State,Path, Request};
use axum::{Extension, Form, Json};
use crate::middlewares::authentication::hash_password;
use crate::models::authentication::Claims;
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

pub async fn reset_password(State(state): State<crate::AppState>, Extension(claims): Extension<Claims>, Form(password):Form<Password>) -> Json<bool> {

    // hash the password
    let hashed_password = hash_password(password.password);

    let done = sqlx::query("UPDATE users SET password = $1 WHERE username = $2")
        .bind(&claims.username).execute(&state.sql_database).await ;

    match done {
        Ok(done) => {
            tracing::info!("updated password") ;
            Json(true)
        },
        Err(err) => {
            tracing::error!("error was {}",err) ;
            Json(false)
        }
    }
    
}

pub async fn get_profile(State(state): State<crate::AppState>, Path(username): Path<String>) -> Json<Profile> {
    Json(get_user_profile("".to_string(), 0).await)
}


async fn get_user_profile(username: String, user_id: i32) -> Profile {
    Profile{ auctions: vec![], mail_id: String::from(""), username: String::from("") }
}