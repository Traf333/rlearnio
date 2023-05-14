use actix_web::{get, HttpResponse, post, Responder, Result, web, web::ServiceConfig};

pub fn talents_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(index)
        .service(show);
}

#[get("")]
async fn index() -> &'static str {
    "Hello Talents!"
}

#[get("/{talent_id}")]
async fn show(path: web::Path<u32>) -> Result<String> {
    let talent_id = path.into_inner();
    Ok(format!("Welcome, talent_id {}!", talent_id))
}

#[post("")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
