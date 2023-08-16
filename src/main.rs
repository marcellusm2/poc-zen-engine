use actix_web::{web, App, HttpServer};

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/evaluate", web::post().to(handlers::evaluate))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
