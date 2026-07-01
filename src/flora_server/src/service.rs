use flora_core::{manager::FloraManager, seed::{FloraProtonSeed, FloraSeed, FloraSeedType, FloraWineSeed}};
use tonic::{Request, Response, Status};

use crate::proto::{self, flora_manager_service_server::FloraManagerService};

pub struct FloraManagerServiceImpl
{
    manager: FloraManager,
}

impl FloraManagerServiceImpl {
    pub fn new(manager: FloraManager) -> Self {
        Self {
            manager
        }
    }
}

#[tonic::async_trait]
impl FloraManagerService for FloraManagerServiceImpl {
    async fn create_wine_seed(&self, request: Request<proto::CreateWineSeedRequest>) -> Result<Response<proto::CreateWineSeedResponse>, Status>
    {
        let req = request.into_inner();
        let mut new_seed = FloraSeed::default();
        new_seed.seed_type = FloraSeedType::Wine(
                FloraWineSeed {
                    wine_prefix: Some(req.prefix),
                    wine_runtime: Some(req.runtime),
                }
            );

        self.manager.create_seed(&req.seed_name, &new_seed)
            .map_err(|e| Status::new(tonic::Code::InvalidArgument, e.to_string()))?;

        Ok(Response::new(proto::CreateWineSeedResponse{}))
    }
    async fn create_proton_seed(&self, request: Request<proto::CreateProtonSeedRequest>) -> Result<Response<proto::CreateProtonSeedResponse>, Status>
    {
        let req = request.into_inner();
        let mut new_seed = FloraSeed::default();
        new_seed.seed_type  = FloraSeedType::Proton(
                FloraProtonSeed {
                    proton_prefix: Some(req.prefix),
                    proton_runtime: Some(req.runtime),
                    game_id: Some(req.game_id),
                    store: Some(req.game_store),
                }
            );

        self.manager.create_seed(&req.seed_name, &new_seed)
            .map_err(|e| Status::new(tonic::Code::InvalidArgument, e.to_string()))?;

        Ok(Response::new(proto::CreateProtonSeedResponse{}))
    }
}
