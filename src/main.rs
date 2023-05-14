mod controllers;

use actix_web::{get, web::ServiceConfig, post, Responder, HttpResponse, web};
use shuttle_actix_web::ShuttleActixWeb;
use controllers::talents::talents_routes;
use dotenvy::dotenv;
use std::env;

#[get("/")]
async fn hello_world() -> &'static str {
    "Status OK"
}

#[get("/api")]
async fn api() -> &'static str {
    "API World!"
}



#[shuttle_runtime::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // load environment variables from .env file
    dotenv().expect(".env file not found");

    let config = move |cfg: &mut ServiceConfig| {
        cfg
            .service(web::scope("/talents").configure(talents_routes))
            .service(api)
            .service(hello_world);
    };

    Ok(config.into())
}
