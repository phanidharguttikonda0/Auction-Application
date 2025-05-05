use async_graphql::{Context, Object, Schema, EmptyMutation, EmptySubscription};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
pub struct Query;

#[Object]
impl Query {
    async fn hello(&self, _ctx: &Context<'_>) -> &str {
        "Hello from GraphQL + Axum!"
    }
}

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription).finish()
}
