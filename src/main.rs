#[path = "../database/mod.rs"]
mod database;

mod common;
mod router;
mod routes;
mod schema;
mod user;

use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get, middleware, route,
    web::{self, Html},
};
use dotenvy::dotenv;
use juniper::http::{GraphQLRequest, graphiql::graphiql_source};

use crate::{
    database::connection::establish_connection,
    routes::ping::rping,
    schema::ping::{Schema, create_schema},
};

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html::new(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db_pool = establish_connection();

    let schema = std::sync::Arc::new(create_schema());
    let port = 8081;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            .service(rping)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
