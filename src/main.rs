mod services;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .configure(services::config) // Certifique-se de que `services::config` existe
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
