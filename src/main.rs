#[macro_use]
extern crate lazy_static;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use graphql::{
    playground_source, DeferSchema, EmptySubscription, GQLRequest, GQLResponseStream, Mutation,
    Query, Schema,
};
use controllers::index::Index;
mod config;
mod graphql;
mod controllers;
mod models;

pub async fn index(schema: web::Data<DeferSchema>, req: GQLRequest) -> GQLResponseStream {
    req.into_inner().execute_stream(&schema).await.into()
}

pub async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/", Some("/"))))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::new(Query, Mutation, EmptySubscription);

    println!("Playground: http://localhost:8000");
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default())
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
            .service(Files::new("/static", "public/static/")) //静态文件目录
            .service(Files::new("/upload", "public/upload/"))
            .route("/index/upload", web::post().to(Index::upload_images)) //上传文件目录
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
