pub use crate::graphql::schemas::query::Query;
use actix_web::{web, HttpResponse, Result};
use async_graphql::http::playground_source;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GQLRequest, GQLResponseStream};
mod schemas;

type DeferSchema = Schema<Query, EmptyMutation, EmptySubscription>;


pub async fn index(schema: web::Data<DeferSchema>, req: GQLRequest) -> GQLResponseStream {
    req.into_inner().execute_stream(&schema).await.into()
}

pub async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/", Some("/"))))
}
