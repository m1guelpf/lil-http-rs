#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

mod http;
mod router;
mod server;

pub use http::{Body, Method, Request, Response};
pub use server::Server;
