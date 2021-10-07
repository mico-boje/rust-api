use rocket::response::status;

#[get("/bayes_theorem/<a>/<b>/<pba>")]
pub fn bayes_theorem(a: f64, b: f64, pba: f64) -> status::Accepted<String> {
    // Json({
    //     "Probability": "(pba * a) / b"
        
    // })
    status::Accepted(Some(format!("Probability: '{}'", (pba * a) / b)))
}
