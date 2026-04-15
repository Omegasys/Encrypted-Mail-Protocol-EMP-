use tokio::net::TcpListener;
use crate::node::services::relay_service::RelayService;
use crate::utils::logger::{Logger, LogLevel};

mod services;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:9000";

    Logger::log(LogLevel::Info, &format!("Starting EMP relay node on {}", addr));

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind");

    loop {
        match listener.accept().await {
            Ok((stream, peer)) => {
                Logger::log(LogLevel::Info, &format!("Incoming connection: {}", peer));

                tokio::spawn(async move {
                    if let Err(e) = RelayService::handle_connection(stream).await {
                        Logger::log(LogLevel::Error, &format!("Connection error: {}", e));
                    }
                });
            }
            Err(e) => {
                Logger::log(LogLevel::Error, &format!("Accept failed: {}", e));
            }
        }
    }
}
