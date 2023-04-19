use std::env;

use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use sea_orm::{Database, DatabaseConnection};

mod handler;
mod repository;

#[derive(Debug, Clone)]
pub struct AppState {
    pub database_connection: DatabaseConnection,
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let database_connection = Database::connect(&database_url).await.unwrap();
    let app_state = AppState {
        database_connection,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .service(handler::hello)
            .service(handler::articles_index)
            .service(handler::articles_create)
            .service(handler::articles_show)
            .service(handler::articles_update)
            .service(handler::articles_delete)
            .service(handler::comments_index)
            .service(handler::comments_create)
            .service(handler::comments_show)
            .service(handler::comments_update)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
