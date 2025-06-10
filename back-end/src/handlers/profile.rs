
use axum::extract::{State,Path, Request};
use axum::{Extension, Form, Json};
use axum::http::Extensions;
use crate::AppState;
use crate::middlewares::authentication::hash_password;
use crate::models::authentication::Claims;
use crate::models::profile::{Auction, Password, Profile, SimpleProfile};

pub async fn profile(State(state): State<crate::AppState>, Extension(claims):Extension<Claims>) -> Json<Result<Profile, String>> {
    // from authorization header we will get the username and user-id
    match get_user_profile(claims.username.clone(), claims.user_id, &state).await {
        Ok(user) =>
            {
                tracing::info!("got the user for {}", &claims.username);
                Json(Ok(user))
            },
        Err(err) => {
            tracing::error!("error was {}",err) ;
            Json(
                Err(String::from("Error getting user profile"))
            )
        }
    }
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
    tracing::info!("reset-password handler was going to execute") ;
    // hash the password
    let hashed_password = hash_password(password.password);

    let done = sqlx::query("UPDATE users SET password = $1 WHERE username = $2")
        .bind(&hashed_password).bind(&claims.username).execute(&state.sql_database).await ;

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

pub async fn get_profile(State(state): State<crate::AppState>, Path((username,user_id)): Path<(String,i32)>) -> Json<Result<Profile, String>> {

    match get_user_profile(username.clone(), user_id, &state).await {
        Ok(user) =>
            {
                tracing::info!("got the user for {}", &username);
                Json(Ok(user))
            },
        Err(err) => {
            tracing::error!("error was {}",err) ;
            Json(
                Err(String::from("Error getting user profile"))
            )
        }
    }

}


async fn get_user_profile(username: String, user_id: i32, state: &AppState) -> Result<Profile,String> {

    // we need to get the mail-id username

let simple_profile = sqlx::query_as::<_,SimpleProfile>("select mail_id from users where user_id=$1")
.bind(user_id).fetch_one(&state.sql_database).await ;

    match simple_profile {
        Ok(simple_profile) => {
            let auctions = sqlx::query_as::<_, Auction>(
                r#"
    SELECT
        r.id AS room_id,
        p.team_selected,
        p.participant_id,
        r.createdAt,
        r.accessibility,
        r.room_status
    FROM participants p
    JOIN rooms r ON r.id = p.room_id
    WHERE p.participant_id = $1
    "#
            )
                .bind(user_id)
                .fetch_all(&state.sql_database)
                .await;


            match auctions {
                    Ok(auctions) => {
                        tracing::info!("Got the profile") ;
                        Ok(Profile{ username, mail_id: simple_profile.mail_id, auctions})
                    },
                    Err(err) => {
                        tracing::error!("go the error while getting the profile") ;
                        Err(String::from("Invalid user-id or server problem"))
                    }
                }
        },
        Err(err) => {
            tracing::error!("cannot able to find mail-id");
            Err(String::from("Cannot able to Find Mail-id"))
        }
    }



}