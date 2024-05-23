use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{Value, serde_json::json};

use crate::common::{create_test_rustacean, delete_test_rustacean};
pub mod common;



#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let rustacean1 = create_test_rustacean(&client);
    let rustacean2 = create_test_rustacean(&client);
    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
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
    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:8000/rustaceans")
        .json(&json!({
            "name": "Rustacean",
            "email": "Rustacean@gmail.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();

    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Rustacean",
        "email": "Rustacean@gmail.com",
        "created_at": rustacean["created_at"]
    }));

    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_get_rustacean() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);

    let response = client
        .get(format!("http://127.0.0.1:8000/rustaceans/{}", &rustacean["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let rustacean: Value = response.json().unwrap();

    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Rustacean",
        "email": "Rustacean@gmail.com",
        "created_at": rustacean["created_at"]
    }));

    delete_test_rustacean(&client, rustacean);
}



#[test]
fn test_update_rustacean() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);
    let response = client
        .put(format!("http://127.0.0.1:8000/rustaceans/{}", &rustacean["id"]))
        .json(&json!({
            "name": "Rust",
            "email": "Rust@gmail.com"
        }))
        .send()
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    let rustacean: Value = response.json().unwrap();

    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Rust",
        "email": "Rust@gmail.com",
        "created_at": rustacean["created_at"]
    }));
    delete_test_rustacean(&client, rustacean);

}



#[test]
fn test_delete_rustacean() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);
    let response = client
        .delete(format!("http://127.0.0.1:8000/rustaceans/{}", rustacean["id"]))
        .send()
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

}