pub use crate::graphql::schemas::query::Query;
pub use crate::graphql::schemas::mutation::Mutation;
pub use async_graphql::http::playground_source;
pub use async_graphql::{EmptySubscription, Schema};
pub use async_graphql_actix_web::{GQLRequest, GQLResponseStream};
mod schemas;

pub type DeferSchema = Schema<Query, Mutation, EmptySubscription>;
