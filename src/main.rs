mod services;
mod model;
mod schema;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Carregar variáveis de ambiente do arquivo .env
    dotenv().ok();

    // Obter a URL do banco de dados
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL deve estar definido");

    // Criar pool de conexões
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Falha ao conectar ao banco de dados");

    println!("Conexão com o banco de dados bem-sucedida!");

    // Iniciar o servidor HTTP
    HttpServer::new(move || {
        App::new()
            .data(pool.clone()) // Passar pool de conexões para o App
            .configure(services::config) // Certifique-se de que `services::config` esteja implementado
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
