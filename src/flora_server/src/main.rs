use flora_core::manager::FloraManager;
use tonic::transport::Server;

use crate::{proto::flora_manager_service_server::FloraManagerServiceServer, service::FloraManagerServiceImpl};

/// Protobuf defs
pub mod proto;
/// Service implementation
pub mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;

    let manager = FloraManager::new()?;

    let greeter = FloraManagerServiceImpl::new(manager);

    Server::builder()
        .add_service(FloraManagerServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
