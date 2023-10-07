use actix_web::{get, post,  HttpResponse, Responder};
use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
struct HelloTemplate<'a> { // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

#[get("/")]
async fn hello() -> impl Responder {
    let hello = HelloTemplate { name: "Askama" }; // instantiate your struct
    // println!("{}", hello.render().unwrap()); // then render it.
    // hello.render().unwrap()); // then render it.
    HttpResponse::Ok().body(hello.render().unwrap())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

