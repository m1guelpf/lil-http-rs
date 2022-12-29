use lil_http::{Body, Response, Server};
use serde_json::{json, Value};
use serial_test::serial;
use std::collections::HashMap;

#[tokio::test]
#[serial]
async fn test_http_lifecycle() {
    let mut server = Server::new().await.unwrap();

    server.routes.get("/", |_| Response::text("Hello, World!"));

    tokio::spawn(async move {
        server.run().await;
    });

    let client = reqwest::Client::new();
    let response = client.get("http://localhost:3000/").send().await.unwrap();

    assert_eq!(response.status(), 200);
    assert_eq!(response.text().await.unwrap(), "Hello, World!");
}

#[tokio::test]
#[serial]
async fn test_http_lifecycle_with_port() {
    let mut server = Server::with_port("3001").await.unwrap();

    server.routes.get("/", |_| Response::text("Hello, World!"));

    tokio::spawn(async move {
        server.run().await;
    });

    let client = reqwest::Client::new();
    let response = client.get("http://localhost:3001/").send().await.unwrap();

    assert_eq!(response.status(), 200);
    assert_eq!(response.text().await.unwrap(), "Hello, World!");
}

#[tokio::test]
#[serial]
async fn test_post_with_body_and_response() {
    let mut server = Server::new().await.unwrap();

    server.routes.post("/", |request| {
        dbg!(&request.body);
        let Body::Json(body) = request.body else {
            return Response::invalid_request();
        };

        let Some(name) = body.get("name") else {
            return Response::invalid_request();
        };

        Response::json(&json!({
            "message": format!("Hello, {}!", name.as_str().unwrap())
        }))
    });

    tokio::spawn(async move {
        server.run().await;
    });

    let mut body = HashMap::new();
    body.insert("name", "Miguel");

    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/")
        .json(&body)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    assert_eq!(
        response.json::<Value>().await.unwrap(),
        json!({ "message": "Hello, Miguel!" })
    );
}
