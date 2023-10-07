use actix_web::{web, App, HttpServer};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::hello)
            .service(routes::echo)
            .route("/hey", web::get().to(routes::manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
