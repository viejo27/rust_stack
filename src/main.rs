mod routes {
    pub mod web {
        pub mod home;
    }
}

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;
use routes::web::home::index;
use sqlx::MySqlPool;

// struct AppState {
//     db_pool: Pool<ConnectionManager<MysqlConnection>>,
// }

// fn establish_connection_pool(database_ulr: &str) -> Pool<ConnectionManager<MysqlConnection>> {
//     let manager = ConnectionManager::<MysqlConnection>::new(database_ulr);
//     Pool::builder().build(manager).expect("Failed to create connection pool")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env");
    //
    // let db_pool = MySqlPool::connect(&database_url).await?;
    let db_pool = MySqlPool::connect(&database_url).await.expect("Failed to connect to the database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
