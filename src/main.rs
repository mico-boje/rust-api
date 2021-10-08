#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;

mod examples;
mod bayes;

use bayes::bayes_theorem;
use examples::{delay, world, hello, not_found};


// #[rocket::main]
// async fn main() {
//     if let Err(e) = rocket::build()
//         .mount("/examples", routes![delay, hello, world])
//         .mount("/bayesian", routes![bayes_theorem])
//         .register("/", catchers![not_found])
//         .launch()
//         .await {
//             println!("Whoops! Rocket didn't launch!");
//             // We drop the error to get a Rocket-formatted panic.
//             drop(e);
//         };
// }

use reqwest;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=IBM&apikey=LR13N6D7ZJ81KQK4").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;
    //let body_json = res.json().await?;

    println!("Body:\n\n{}", body);

    Ok(())
}
