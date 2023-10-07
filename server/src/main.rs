#[macro_use] extern crate rocket;

use std::env;
use dotenvy::dotenv;
use rocket::serde::{Deserialize, json::Json};
use rocket::serde::json::serde_json::json;
use rocket_cors::CorsOptions;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Prompt {
    prompt: String,
}

#[get("/")]
fn index() -> &'static str {
    "Ahhh so you're a nerd I see!!!!"
}

#[post("/prompt", data = "<prompt>")]
async fn prompt(prompt: Json<Prompt>) -> String {
    let client = reqwest::Client::new();
    let res = client.post(format!("https://generativelanguage.googleapis.com/v1beta2/models/text-bison-001:generateText?key={}", env::var("PALM_API_KEY").unwrap()))
        .json(&json!({
            "prompt": {
                "text": &prompt.prompt
            },
            "temperature": 1.0,
            "candidateCount": 1
        }))
        .send()
        .await
        .unwrap();

    String::from_utf8(res
        .bytes()
        .await
        .unwrap()
        .to_vec())
        .unwrap()
}

#[launch]
fn rocket() ->  _ {
    dotenv().unwrap();
    rocket::build().mount("/", routes![index, prompt]).attach(CorsOptions::default().to_cors().unwrap())
}
