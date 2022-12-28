use crate::{http::Request, router::Router};
use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    listener: TcpListener,
    pub routes: Router,
}

impl Server {
    pub fn new() -> Result<Self> {
        Self::with_port("3000")
    }

    pub fn with_port(port: &str) -> Result<Self> {
        let std_listener = std::net::TcpListener::bind(format!("0.0.0.0:{port}"))?;
        let listener = TcpListener::from_std(std_listener)?;
        println!("Server listening on port {port}");

        Ok(Self {
            listener,
            routes: Router::new(),
        })
    }

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
