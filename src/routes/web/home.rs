use actix_web::{web, get, HttpResponse, Responder};
use askama::Template;
use sqlx::MySqlPool;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a i64,
}

#[get("/")]
async fn index(db_pool: web::Data<MySqlPool>) -> impl Responder {
    let res = sqlx::query!("SELECT 20 + 7 as sum")
        .fetch_one(db_pool.get_ref())
        .await
        .expect("Error executing query");
    let hello = IndexTemplate { name: &res.sum };
    HttpResponse::Ok().body(hello.render().unwrap())
}

