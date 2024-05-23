
use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{Value, serde_json::json};


pub fn create_test_rustacean(client: &Client) -> Value {
    let response = client
        .post("http://127.0.0.1:8000/rustaceans")
        .json(&json!({
            "name": "Rustacean",
            "email": "Rustacean@gmail.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

pub fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let _response = client
        .delete(format!("http://127.0.0.1:8000/rustaceans/{}", rustacean["id"]))
        .send()
        .unwrap();
}