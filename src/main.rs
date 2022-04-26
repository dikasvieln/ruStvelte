//#![feature(proc_macro_hygiene, decl_macro)]

//#[macro_use] extern crate serde_derive;


//mod categories;

/*
#[get("/")]
fn api() -> String {
    format!("Hello, from rust")
}
*/
//#[get("/message")]
/*fn json_message() -> Json<api> {
    json!({
        "app" : "rust svelte",
        "version" : "0.0.1",
        "status" : "Ok Computer"
    })
}*/


use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

