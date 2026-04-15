use tokio::net::TcpStream;
use tokio_rustls::{TlsConnector, client::TlsStream};
use rustls::{ClientConfig, RootCertStore};
use std::sync::Arc;
use crate::core::errors::ProtocolError;

pub struct TlsTransport;

impl TlsTransport {
    pub async fn connect(domain: &str, stream: TcpStream) -> Result<TlsStream<TcpStream>, ProtocolError> {
        let root_store = RootCertStore::empty();

        let config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let connector = TlsConnector::from(Arc::new(config));

        let server_name = domain.try_into().map_err(|_| ProtocolError::IoError)?;

        connector.connect(server_name, stream)
            .await
            .map_err(|_| ProtocolError::IoError)
    }
}
