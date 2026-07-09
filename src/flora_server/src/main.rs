use directories::BaseDirs;
use flora_core::{errors::FloraError, manager::FloraManager};
use tokio::{
    net::UnixListener,
    signal::{self},
};
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;
use tower_http::trace::TraceLayer;

use crate::{
    proto::flora_manager_service_server::FloraManagerServiceServer,
    service::FloraManagerServiceImpl,
};

/// Protobuf defs
pub mod proto;
/// Service implementation
pub mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:41024".parse()?;

    let manager = FloraManager::new()?;

    let greeter = FloraManagerServiceImpl::new(manager);

    let signal = signal::ctrl_c();

    log::info!("Flora gRPC Server is running!");
    Server::builder()
        .layer(TraceLayer::new_for_grpc())
        .add_service(FloraManagerServiceServer::new(greeter))
        .serve_with_shutdown(addr, async { signal.await.unwrap() })
        .await?;

    Ok(())
}
