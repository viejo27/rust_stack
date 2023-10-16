mod routes {
    pub mod web {
        pub mod home;
    }
}

mod models;
mod schema;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use std::env;
use diesel::mysql::MysqlConnection;
use diesel::{
    r2d2::{ConnectionManager, Pool}
};
use routes::web::home::hello;

struct AppState {
    db_pool: Pool<ConnectionManager<MysqlConnection>>,
}

fn establish_connection_pool(database_ulr: &str) -> Pool<ConnectionManager<MysqlConnection>> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_ulr);
    Pool::builder().build(manager).expect("Failed to create connection pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env");

    let db_pool = establish_connection_pool(&database_url);

    HttpServer::new(move || {
        App::new()
            .app_data(AppState {
                db_pool: db_pool.clone(),
            })
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
