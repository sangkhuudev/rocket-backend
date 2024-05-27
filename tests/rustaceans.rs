use reqwest::StatusCode;
use rocket::serde::json::{serde_json::json, Value};

use crate::common::*;
pub mod common;

#[test]
fn test_get_rustaceans() {
    let client = get_client_with_logged_in();
    let rustacean1 = create_test_rustacean(&client);
    let rustacean2 = create_test_rustacean(&client);
    let response = client
        .get(format!("{}/rustaceans", APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json_object: Value = response.json().unwrap();
    assert!(json_object.as_array().unwrap().contains(&rustacean1));
    assert!(json_object.as_array().unwrap().contains(&rustacean2));

    delete_test_rustacean(&client, rustacean1);
    delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_create_rustacean() {
    let client = get_client_with_logged_in();
    let response = client
        .post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "name": "Rustacean",
            "email": "Rustacean@gmail.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();

    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Rustacean",
            "email": "Rustacean@gmail.com",
            "created_at": rustacean["created_at"]
        })
    );

    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_get_rustacean() {
    //Setup
    let client = get_client_with_logged_in();
    let rustacean = create_test_rustacean(&client);

    //Test
    let response = client
        .get(format!("{}/rustaceans/{}", APP_HOST, &rustacean["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let rustacean: Value = response.json().unwrap();

    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Rustacean",
            "email": "Rustacean@gmail.com",
            "created_at": rustacean["created_at"]
        })
    );

    //Test for not-found id 
    let response = client
        .get(format!("{}/rustaceans/9999", APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);    
    //Cleanup
    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_rustacean() {
    let client = get_client_with_logged_in();
    let rustacean = create_test_rustacean(&client);
    let response = client
        .put(format!("{}/rustaceans/{}", APP_HOST, &rustacean["id"]))
        .json(&json!({
            "name": "Rust",
            "email": "Rust@gmail.com"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let rustacean: Value = response.json().unwrap();

    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Rust",
            "email": "Rust@gmail.com",
            "created_at": rustacean["created_at"]
        })
    );
    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustacean() {
    let client = get_client_with_logged_in();
    let rustacean = create_test_rustacean(&client);
    let response = client
        .delete(format!("{}/rustaceans/{}", APP_HOST, &rustacean["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
