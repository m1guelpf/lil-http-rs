use lil_http::{Response, Server};

#[tokio::test]
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
