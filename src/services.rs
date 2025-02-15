use actix_web::{
    web::{scope, ServiceConfig},
    get, HttpResponse, Responder,
};
use serde_json::json;

#[get("/healthchecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "Health checker route";
    HttpResponse::Ok().json(json!({ "message": MESSAGE }))
}

HttpResponse::Ok().json(json!({
    async fn health_checker() -> impl Responder {
        const MESSAGE: &str = "Health checker route";
    }
}))

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api") 
            .service(health_checker),
    );
}
