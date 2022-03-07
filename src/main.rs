//#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
//#[macro_use] extern crate serde_derive;

//use rocket::http::{Cookie};
use rocket::response::content::Json;
//use serde::Serialize;
use rocket::response::{content, status};
use rocket::http::Status;

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

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct Success(&'static str);

#[get("/")]
fn json() -> Success {
    Success("{ \"hi\": \"world\"  }")
}

#[launch]
fn rocket() -> _  {
    //categories::categories();
    rocket::build()
        .mount("/", routes![json])

    // println!("Hello, world!");
}
