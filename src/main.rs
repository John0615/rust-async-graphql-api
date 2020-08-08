use actix_cors::Cors;
use actix_web::{guard, web, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

mod graphql;

use graphql::{index, index_playground, Query};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

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
