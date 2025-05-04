mod handlers;
mod models;
mod middlewares;

use axum::{Router};
use axum::{routing::{get, post}, middleware};
use sqlx::{Pool, Postgres};
use crate::handlers::authentication::{forget_password, login, sign_up};
use crate::handlers::profile::{get_profile, profile, reset_password, search};
use crate::handlers::room_handler::{get_public_rooms, get_remaining_teams, get_team, get_teams};
use crate::middlewares::authentication::{authorization_check, validate_details};

async fn server_start() -> String {
    String::from("Hello World")
}

fn authentication_routes() -> Router<AppState> {
    Router::new().route("/login", post(login))
        .route("/sign-up", post(sign_up))
        .route("/forgot-credentials", post(forget_password))
        .layer(middleware::from_fn(validate_details))
}


fn room_routes() -> Router<AppState> {
    Router::new().route("/get-remaining-teams/:room_id", get(get_remaining_teams))
        .route("/get-public-rooms", get(get_public_rooms))
        .route("/get-team/:room_id/:team_name", get(get_team))
        .route("/get-teams/:room_id", get(get_teams))
        .route("/get-team/:room_id/:team_name", get(get_team))
        .layer(middleware::from_fn(authorization_check))
}


fn profile_routes() -> Router<AppState> {
    Router::new().route("/", get(profile).layer(middleware::from_fn(authorization_check)))
        .route("/search/:username", get(search))
        .route("/reset-password", post(reset_password).layer(middleware::from_fn(authorization_check)))
        .route("/get-user/:username", get(get_profile))
}

#[derive(Clone)]
pub struct AppState {
    pub sql_database: Pool<Postgres>,
}


#[tokio::main]
async fn main() {

    let sql_database = sqlx::Pool::connect("postgres://postgres:phani@localhost:5432/auction").await.unwrap() ;
    let state = AppState{ sql_database} ;

    let app = Router::new()
        .nest("/authentication", authentication_routes())
        .nest("/room", room_routes())
        .nest("/user", profile_routes())
        .route("/", get(server_start))
        .with_state(state);


    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:9090").await.unwrap() ;
    println!("Listening on {}", tcp_listener.local_addr().unwrap());
    axum::serve(tcp_listener, app).await.unwrap()
}
