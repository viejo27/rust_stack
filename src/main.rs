use actix_web::{App, HttpServer};

mod routes {
    pub mod web {
        pub mod home;
    }
}

use routes::web::home::hello;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
