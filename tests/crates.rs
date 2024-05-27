use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

use crate::common::*;
pub mod common;

#[test]
fn test_create_crate() {
    //setup
    let client = get_client_with_logged_in_admin();
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

    assert_eq!(
        a_crate,
        json!({
            "id": a_crate["id"],
            "rustacean_id": rustacean["id"],
            "code": "Webdev",
            "name": "Rust",
            "version": "0.1.0",
            "description": "A webdev crate",
            "created_at": a_crate["created_at"]
        })
    );

    // Cleanup
    delete_test_crate(&client, a_crate);
    delete_test_rustacean(&client, rustacean);
}
#[test]
fn test_get_crates() {
    //Setup
    let client = get_client_with_logged_in_admin();
    let rustacean1 = create_test_rustacean(&client);
    let rustacean2 = create_test_rustacean(&client);

    let a_crate1 = create_test_crate(&client, &rustacean1);
    let a_crate2 = create_test_crate(&client, &rustacean2);

    // Test
    let client = get_client_with_logged_in_viewer();
    let response = client.get(format!("{}/crates", APP_HOST)).send().unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json_object: Value = response.json().unwrap();
    assert!(json_object.as_array().unwrap().contains(&a_crate1));
    assert!(json_object.as_array().unwrap().contains(&a_crate2));

    // Test for unloggin user
    let client = Client::new();
    let response = client.get(format!("{}/crates", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // Cleanup
    let client = get_client_with_logged_in_admin();
    delete_test_crate(&client, a_crate1);
    delete_test_rustacean(&client, rustacean1);

    delete_test_crate(&client, a_crate2);
    delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_get_crate() {
    //Setup
    let client = get_client_with_logged_in_admin();
    let rustacean = create_test_rustacean(&client);
    let a_crate = create_test_crate(&client, &rustacean);

    // Test
    let client = get_client_with_logged_in_viewer();
    let response = client
        .get(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();

    assert_eq!(
        a_crate,
        json!({
            "id": a_crate["id"],
            "rustacean_id": rustacean["id"],
            "code": "Webdev",
            "name": "Rust",
            "version": "0.1.0",
            "description": "A webdev crate",
            "created_at": a_crate["created_at"]
        })
    );
    // Test for not-found crate
    let response = client
        .get(format!("{}/crates/9999", APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Cleanup
    let client = get_client_with_logged_in_admin();
    delete_test_crate(&client, a_crate);
    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_crate() {
    //Setup
    let client = get_client_with_logged_in_admin();
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

    assert_eq!(
        a_crate,
        json!({
            "id": a_crate["id"],
            "rustacean_id": rustacean["id"],
            "code": "Webdev 2",
            "name": "Rust 2",
            "version": "0.1.0",
            "description": "A webdev crate",
            "created_at": a_crate["created_at"]
        })
    );
    // Test author-switching for a crate and long description.
    let rustacean2 = create_test_rustacean(&client);
    let response = client
        .put(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .json(&json!({
            "rustacean_id": rustacean2["id"],
            "code": "Webdev 2",
            "name": "Rust 2",
            "version": "0.1.0",
            "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();

    assert_eq!(
        a_crate,
        json!({
            "id": a_crate["id"],
            "rustacean_id": rustacean2["id"],
            "code": "Webdev 2",
            "name": "Rust 2",
            "version": "0.1.0",
            "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
            "created_at": a_crate["created_at"]
        })
    );
    // Cleanup
    delete_test_crate(&client, a_crate);
    delete_test_rustacean(&client, rustacean);
    delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_delete_crate() {
    //Setup
    let client = get_client_with_logged_in_admin();
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
