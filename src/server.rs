use crate::{http::Request, router::Router};
use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

/// The server is responsible for accepting connections and routing requests.
///
/// # Example
///
/// ```rust,no_run
/// use lil_http::{Server, Response};
///
/// #[tokio::main]
/// async fn main() {
///    let mut http = Server::new().await.unwrap();
///
///    http.routes
///        .get("/", |request| {
///           println!("Received {} request to {}", request.method, request.path);
///
///          Response::text("Hello, World!")
///       });
///
///    http.run().await;
/// }
/// ```
///
/// # Notes
///
/// The server will not stop until the process is killed.
pub struct Server {
    /// The underlying TCP listener.
    listener: TcpListener,
    /// The router instance that will handle requests.
    pub routes: Router,
}

impl Server {
    /// # Errors
    ///
    /// Will return an error if port 3000 is already in use.
    pub async fn new() -> Result<Self> {
        Self::with_port("3000").await
    }

    /// # Errors
    ///
    /// Will return an error if the port is already in use.
    pub async fn with_port(port: &str) -> Result<Self> {
        let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
        println!("Server listening on port {port}");

        Ok(Self {
            listener,
            routes: Router::new(),
        })
    }

    /// # Panics
    ///
    /// Will panic if the server fails to accept a connection.
    pub async fn run(&self) {
        loop {
            let incoming = self.listener.accept().await;

            match incoming {
                Ok((mut stream, _)) => {
                    let router = self.routes.clone();

                    tokio::spawn(async move {
                        Self::handle_connection(&mut stream, router).await.unwrap();
                    });
                }
                Err(e) => {
                    println!("error: {e}");
                }
            }
        }
    }

    async fn handle_connection(stream: &mut TcpStream, router: Router) -> Result<()> {
        loop {
            let mut buf = [0; 1024];
            _ = stream.read(&mut buf).await?;
            let Ok(request) = Request::try_from(&buf) else { break };

            let response = router.handle(request);

            stream.write_all(response.to_string().as_bytes()).await?;
            stream.shutdown().await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_can_create_server() {
        let _ = Server::new().await.unwrap();
    }

    #[tokio::test]
    async fn test_can_create_server_with_port() {
        let _ = Server::with_port("3001").await.unwrap();
    }

    #[tokio::test]
    async fn test_can_run_server() {
        let http = Server::with_port("1023").await.unwrap();

        tokio::spawn(async move {
            http.run().await;
        });

        let _ = TcpStream::connect("127.0.0.1:1023").await.unwrap();
    }
}
