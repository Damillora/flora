use directories::BaseDirs;
use flora_core::{errors::FloraError, manager::FloraManager};
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;

use crate::{proto::flora_manager_service_server::FloraManagerServiceServer, service::FloraManagerServiceImpl};

/// Protobuf defs
pub mod proto;
/// Service implementation
pub mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_dirs = BaseDirs::new().ok_or(FloraError::NoValidHome)?;
    let mut uds_socket_path = base_dirs.runtime_dir().ok_or(FloraError::NoValidHome)?.to_path_buf();
    uds_socket_path.push("flora-server.sock");

    let uds = UnixListener::bind(uds_socket_path)?;
    let incoming = UnixListenerStream::new(uds);

    let manager = FloraManager::new()?;

    let greeter = FloraManagerServiceImpl::new(manager);

    Server::builder()
        .add_service(FloraManagerServiceServer::new(greeter))
        .serve_with_incoming(incoming)
        .await?;

    Ok(())
}
