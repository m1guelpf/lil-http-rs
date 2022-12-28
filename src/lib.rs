#![feature(rustdoc_missing_doc_code_examples)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    missing_docs,
    rustdoc::all
)]

//! `lil_http` is a simple HTTP server library for Rust.
//!
//! ## Example
//!
//! ```rust,no_run
//! use lil_http::{Server, Response};
//!
//! #[tokio::main]
//! async fn main() {
//!    let mut http = Server::new().unwrap();
//!
//!    http.routes
//!        .get("/", |request| {
//!            println!("Received {} request to {}", request.method, request.path);
//!
//!            Response::text("Hello, World!")
//!        });
//!
//!    http.run().await;
//! }
//! ```

mod http;
mod router;
mod server;

pub use http::{Body, Method, Request, Response, StatusCode};
pub use server::Server;
