#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;

mod examples;
mod bayes;


use bayes::bayes_theorem;
use examples::{delay, world, hello, not_found};

// #[launch] // Non async variant
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/hello", routes![world])
        
// }

#[rocket::main]
async fn main() {
    if let Err(e) = rocket::build()
        .mount("/examples", routes![delay, hello, world])
        .mount("/bayesian", routes![bayes_theorem])
        .register("/", catchers![not_found])
        .launch()
        .await {
            println!("Whoops! Rocket didn't launch!");
            // We drop the error to get a Rocket-formatted panic.
            drop(e);
        };
}
