use reqwest::{blocking::{Client, ClientBuilder}, header, StatusCode};
use serde_json::{Value, json};
use std::process::Command;

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";

pub fn create_test_rustacean(client: &Client) -> Value {
    let response = client
        .post(format!("{}/rustaceans", APP_HOST))
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
        .delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
}


pub fn create_test_crate(client: &Client, rustacean: &Value) -> Value {
    let response = client
        .post(format!("{}/crates", APP_HOST))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "Webdev",
            "name": "Rust",
            "version": "0.1.0",
            "description": "A webdev crate",
        }))
        .send()
        .unwrap();
    
    response.json().unwrap()
}

pub fn delete_test_crate(client: &Client, a_crate: Value) {
    let _response = client
        .delete(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
}

pub fn get_client_with_logged_in() -> Client {
    //setup
    let _ = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cli",
            "users",
            "create",
            "--username",
            "test_admin",
            "--password",
            "123456",
            "--roles",
            "admin",
        ])
        .output();

    let client = Client::new();
    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username": "test_admin",
            "password": "123456"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let object: Value = response.json().unwrap();
    assert!(object.get("token").is_some());

    let header_value = format!("Bearer {}", object["token"].as_str().unwrap());

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&header_value).unwrap()
    );
    ClientBuilder::new().default_headers(headers).build().unwrap()
}