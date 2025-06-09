#[path = "../database/mod.rs"]
mod database;

mod common;
mod router;
mod user;

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get, middleware, route,
    web::{self, Html},
};
use dotenvy::dotenv;
use juniper::http::{GraphQLRequest, graphiql::graphiql_source};

use crate::{database::connection::establish_connection, router::graphql::GraphQLSchema};

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html::new(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(
    st: web::Data<Arc<GraphQLSchema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let user = data.execute(&st.schema, &st.context).await;
    HttpResponse::Ok().json(user)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("error"));

    let schema = Arc::new(GraphQLSchema::new(Arc::new(establish_connection())));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
