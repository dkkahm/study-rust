use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use actix_web::middleware::Logger;
use sqlx::PgPool;
use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subcribe;

pub async fn run(
    listener: TcpListener,
    db_pool: PgPool
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subcribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}