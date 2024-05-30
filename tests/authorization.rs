use std::process::Command;

use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

use crate::common::*;
pub mod common;

#[test]
fn test_login() {
    //setup
    let client = Client::new();
    let output = Command::new("cargo")
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
    println!("{:?}", output);

    // Test for successful login
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
    assert_eq!(object["token"].as_str().unwrap().len(), 128);

    // Test for failure login
    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username": "test_admin",
            "password": "12345"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn test_me() {
    let client = get_client_with_logged_in_viewer();
    // Test for successful login
    let response = client
        .get(format!("{}/me", APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let object: Value = response.json().unwrap();
    assert!(object.get("id").is_some());
    assert!(object.get("username").is_some());
    assert!(object.get("created_at").is_some());
    assert!(object.get("password").is_none());
    assert_eq!(object["username"], "test_viewer");
}
