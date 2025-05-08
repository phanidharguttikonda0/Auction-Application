use async_graphql::{Context, Object, SimpleObject};
use crate::AppState;
use crate::models::graph_ql_models::*;
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, ctx: &Context<'_>) -> User {
        let room_id = ctx.data_unchecked::<String>() ;
        tracing::info!("accessing the user from {}", room_id);
        User{
            username: String::from("phani_the_rockzz") ,
            mail_id: String::from("phanidharguttikonda0@gmail.com")
        }
    }

    async fn unsold_players(&self, ctx: &Context<'_>) -> UnsoldPlayers { // returns all unsold players
        let room_id = ctx.data_unchecked::<String>() ;
        tracing::info!("accessing the unsold_players from {}", room_id);
        UnsoldPlayers {
            players: Vec::new()
        }
    }

    async fn unsold_players_batsman(&self, ctx: &Context<'_>) -> UnsoldBatsmans {
        let room_id = ctx.data_unchecked::<String>() ;
        tracing::info!("accessing the unsold_batsman's from {}", room_id);
        UnsoldBatsmans {
            batsman: Vec::new()
        }
    }


    async fn unsold_players_bowlers(&self, ctx: &Context<'_>) -> UnsoldBowlers {
        let room_id = ctx.data_unchecked::<String>() ;
        tracing::info!("accessing the unsold_bowlers from {}", room_id);
        UnsoldBowlers {
            bowlers: Vec::new()
        }
    }

    async fn  unsold_all_rounders(&self, ctx: &Context<'_>) -> UnsoldAllRounders {
        let room_id = ctx.data_unchecked::<String>() ;
        tracing::info!("accessing the unsold_all-rounder's from {}", room_id);
        UnsoldAllRounders {
            all_rounders: Vec::new()
        }
    }





}

#[derive(SimpleObject)]
pub struct User{
    pub username: String,
    pub mail_id: String,
}