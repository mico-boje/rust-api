use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BayesTheorem {
    event_a: f64,
    event_b: f64,
    b_given_a: f64,
    a_given_b: f64
}

#[derive(Debug, Deserialize)]
pub struct Input {
    event_a: f64,
    event_b: f64,
    b_given_a: f64
}

#[post("/bayes_theorem", data = "<input>")]
pub fn bayes_theorem(input: Json<Input>) -> Json<BayesTheorem> {
    let bayes = BayesTheorem {event_a: input.event_a, event_b: input.event_b, b_given_a: input.b_given_a, a_given_b: ((input.b_given_a * input.event_a) / input.event_b), };
    Json(bayes)
}

