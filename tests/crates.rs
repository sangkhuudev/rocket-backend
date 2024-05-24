use reqwest::{blocking::Client, StatusCode};
use serde_json::{Value, json};

use crate::common::*;
pub mod common;

#[test]
fn test_create_crate() {
    //setup
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);
    // Test
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
    assert_eq!(response.status(), StatusCode::CREATED);

    let a_crate: Value = response.json().unwrap();

    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "rustacean_id": rustacean["id"],
        "code": "Webdev",
        "name": "Rust",
        "version": "0.1.0",
        "description": "A webdev crate",
        "created_at": a_crate["created_at"]
    }));

    // Cleanup
    delete_test_crate(&client, a_crate);
    delete_test_rustacean(&client, rustacean);
}
#[test]
fn test_get_crates() {
    //Setup
    let client = Client::new();
    let rustacean1 = create_test_rustacean(&client);
    let rustacean2 = create_test_rustacean(&client);

    let a_crate1 = create_test_crate(&client, &rustacean1);
    let a_crate2 = create_test_crate(&client, &rustacean2);

    // Test
    let response = client
        .get(format!("{}/crates", APP_HOST))
        .send()
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);

    let json_object: Value = response.json().unwrap();
    assert!(json_object.as_array().unwrap().contains(&a_crate1));
    assert!(json_object.as_array().unwrap().contains(&a_crate2));

    // Cleanup
    delete_test_crate(&client, a_crate1);
    delete_test_rustacean(&client, rustacean1);

    delete_test_crate(&client, a_crate2);
    delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_get_crate() {
    //Setup
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);
    let a_crate = create_test_crate(&client, &rustacean);

    // Test
    let response = client
        .get(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();

    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "rustacean_id": rustacean["id"],
        "code": "Webdev",
        "name": "Rust",
        "version": "0.1.0",
        "description": "A webdev crate",
        "created_at": a_crate["created_at"]
    }));

    // Cleanup
    delete_test_crate(&client, a_crate);
    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_crate() {
    //Setup
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);
    let a_crate = create_test_crate(&client, &rustacean);

    // Test
    let response = client
        .put(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "Webdev 2",
            "name": "Rust 2",
            "version": "0.1.0",
            "description": "A webdev crate",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();

    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "rustacean_id": rustacean["id"],
        "code": "Webdev 2",
        "name": "Rust 2",
        "version": "0.1.0",
        "description": "A webdev crate",
        "created_at": a_crate["created_at"]
    }));

    // Cleanup
    delete_test_crate(&client, a_crate);
    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_crate() {
    //Setup
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);
    let a_crate = create_test_crate(&client, &rustacean);

    // Test
    let response = client
        .delete(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Cleanup
    delete_test_rustacean(&client, rustacean);
}

