mod handlers;
mod models;
mod middlewares;
mod graph_ql_fields;
mod auction_room;

use std::sync::{Arc, RwLock};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use handlers::players::{get_player, get_stats};
use tracing_subscriber::EnvFilter;


use axum::{Extension, Router, extract::Path};
use axum::{routing::{get, post}, middleware};
use sqlx::{Pool, Postgres};
use crate::handlers::authentication::{forget_password, login, sign_up};
use crate::handlers::profile::{get_profile, profile, reset_password, search};
use crate::handlers::room_handler::{get_pool, get_public_rooms, get_team, get_teams};
use crate::middlewares::authentication::{authorization_check};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use std::collections::HashMap;
use redis::Client;
use crate::auction_room::handle_ws_upgrade;
use crate::graph_ql_fields::QueryRoot;
use crate::models::authentication::Claims;
use crate::models::rooms::ParticipantsConnections;

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)// Whether to show the crate/module name in logs.default is true
        .pretty()
    .init();
}


async fn server_start(Extension(data): Extension<Claims>) -> String {
    tracing::info!("data was {:#?}", data) ;
    tracing::trace!("server_started with server_start function") ;
    tracing::debug!("we are the function") ;
    tracing::warn!("warning occured") ;
    String::from("Hello World")
}

fn authentication_routes() -> Router<AppState> {
    Router::new().route("/login", post(login))
        .route("/sign-up", post(sign_up))
        .route("/forgot-credentials", post(forget_password))
}


fn room_routes() -> Router<AppState> {
    Router::new()
        .route("/get-public-rooms", get(get_public_rooms))
        .route("/get-team/{room_id}/{team_name}", get(get_team))
        .route("/get-teams/{room_id}", get(get_teams))
        .route("/get-pool/{pool_id}", get(get_pool))
        .layer(middleware::from_fn(authorization_check))
}


fn profile_routes() -> Router<AppState> {
    Router::new().route("/", get(profile).layer(middleware::from_fn(authorization_check)))
        .route("/search/{username}", get(search))
        .route("/reset-password", post(reset_password).layer(middleware::from_fn(authorization_check)))
        .route("/get-user/{username}/{user_id}", get(get_profile)) // returns room-id's along with data played and username mail-id etc
}

fn player_routes() -> Router<AppState> {
    Router::new().route("/get-player/{player_id}", get(get_player))
    .route("/get-stats/{stats_id}", get(get_stats))
    .layer(middleware::from_fn(authorization_check))
}

async fn graphql_handler(Path(room_id): Path<String>, schema: Extension<Schema<QueryRoot, EmptyMutation, EmptySubscription>>, req: GraphQLRequest) -> GraphQLResponse {
    let ctx = room_id ;
    schema.execute(req.into_inner().data(ctx)).await.into()
}

#[derive(Clone)]
pub struct AppState {
    pub sql_database: Pool<Postgres>,
    pub websocket_connections: Arc<RwLock<HashMap<String,Vec<ParticipantsConnections>>>>,
    pub redis_connection: Client
}

impl AppState {
    pub fn new(sql_database: Pool<Postgres>, redis_connection: Client) -> Self {
        Self {
            sql_database,
            websocket_connections: Arc::new(RwLock::new(HashMap::new())),
            redis_connection
        }
    }
}

#[tokio::main]
async fn main() {
    init_tracing();

    tracing::info!("Starting server"); // TRACE < DEBUG < INFO < WARN < ERROR
    //if specify RUST_LOG=info then from info to error everything will stdout

    let sql_database = sqlx::Pool::connect("postgres://postgres:phani@localhost:5432/auction").await.unwrap() ;
    let redis_connection = Client::open("redis://127.0.0.1:6379/").unwrap() ;

    let state = AppState::new(sql_database, redis_connection) ;

    
    let schema = Schema::build(QueryRoot,EmptyMutation,EmptySubscription).finish() ;
    let app = Router::new()
        .route("/", get(server_start)).layer(middleware::from_fn(authorization_check))
        .nest("/authentication", authentication_routes())
        .nest("/rooms", room_routes())
        .nest("/user", profile_routes())
        .nest("/player", player_routes())
        .route("/ws/{room_id}/{user_id}", get(handle_ws_upgrade))
        .route("/graphql/{room_id}", post(graphql_handler))
        .with_state(state)
        .layer(Extension(schema));


    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:9090").await.unwrap() ;
    tracing::info!("{}", format!("Listening on {}", tcp_listener.local_addr().unwrap())) ;
    axum::serve(tcp_listener, app).await.unwrap()
}


/*
error! -> occurs when something is broken and the app can never be recovered
warn! -> Something suspicious or unexpected happened, but it continues
info! -> normal application events that user cares about
debug! -> Developer-focused info that helps understand app internals
trace! -> Very detailed logs for step-by-step tracing
*/