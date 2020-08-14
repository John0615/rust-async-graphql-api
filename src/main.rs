use actix_cors::Cors;
use actix_web::{ guard, web, App, HttpServer, Result, HttpResponse};
use graphql::{ playground_source, DeferSchema, GQLRequest, GQLResponseStream, Query, Mutation, EmptySubscription, Schema};

mod graphql;

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
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
