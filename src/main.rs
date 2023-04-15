use actix_web::{middleware, App, HttpServer};
use env_logger::Env;

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

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
