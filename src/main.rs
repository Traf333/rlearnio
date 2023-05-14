mod controllers;

use actix_web::{get, web::ServiceConfig, post, Responder, HttpResponse, web};
use shuttle_actix_web::ShuttleActixWeb;
use controllers::talents::talents_routes;
use shuttle_secrets::SecretStore;

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
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    println!("{:?} db", secret_store.get("DATABASE_URL"));

    let config = move |cfg: &mut ServiceConfig| {
        cfg
            .service(web::scope("/talents").configure(talents_routes))
            .service(api)
            .service(hello_world);
    };

    Ok(config.into())
}
