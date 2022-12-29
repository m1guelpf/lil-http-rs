use lil_http::{Body, Response, Server};

#[tokio::main]
async fn main() {
    let mut http = Server::new().await.unwrap();

    http.routes
        .get("/", |request| {
            println!("Received {} request to {}", request.method, request.path);

            Response::text(
                format!(
                    "Hello, {}!",
                    request.query.get("name").unwrap_or(&"World".to_string())
                )
                .as_str(),
            )
        })
        .get("/api/user", |request| {
            println!("Received {} request to {}", request.method, request.path);

            Response::json(&serde_json::json!({
                "name": "Miguel Piedrafita",
                "age": 20,
            }))
        })
        .post("/api/hello", |request| {
            println!("Received {} request to {}", request.method, request.path);

            let Body::Json(body) = request.body else {
                return Response::invalid_request();
            };

            let Some(name) = body.get("name") else {
                return Response::invalid_request();
            };

            Response::json(&serde_json::json!({
                "message": format!("Hello, {name}!"),
            }))
        });

    http.run().await;
}
