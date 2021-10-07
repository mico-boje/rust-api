#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;


mod bayes;

use rocket::tokio::time::{sleep, Duration};
use rocket::Request;
use rocket::get;
use bayes::bayes_theorem;

#[derive(serde::Serialize)]
struct Response {
    reply: String,
}

#[get("/delay?<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "hello, world!"
}


#[get("/hello_person?<name>&<age>&<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

// #[launch] // Non async variant
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/hello", routes![world])
        
// }

#[rocket::main]
async fn main() {
    if let Err(e) = rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![delay, hello])
        .mount("/hello", routes![world])
        .mount("/bayesian", routes![bayes_theorem])
        .launch()
        .await {
            println!("Whoops! Rocket didn't launch!");
            // We drop the error to get a Rocket-formatted panic.
            drop(e);
        };
}
