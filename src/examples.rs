use rocket::tokio::time::{sleep, Duration};
use rocket::Request;
use rocket::get;


#[derive(serde::Serialize)]
pub struct Response {
    reply: String,
}

#[get("/delay?<seconds>")]
pub async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/hello/world")]              // <- route attribute
pub fn world() -> &'static str {  // <- request handler
    "hello, world!"
}


#[get("/hello_person?<name>&<age>&<cool>")]
pub fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Whoops, '{}' is not a valid path.", req.uri())
}