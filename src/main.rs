use std::env;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .service(handlers::hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
