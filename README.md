# lil-http

A barebones HTTP 1.1 framework, built in Rust with no external dependencies (other than tokio).

## Features

- [x] Listening to incoming requests
- [x] Parsing method, path, query, headers, and body according to the HTTP 1.1 spec
- [x] Responding to requests with an arbitrary body and headers
- [x] Helpers for responding with text or JSON
- [x] Allow defining routes and methods as closures
- [x] Appropiately routing the request to its function, or 404'ing otherwise
- [x] Appropiately crafting and returning 405 errors on invalid methods.

## Usage

```rust
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
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
