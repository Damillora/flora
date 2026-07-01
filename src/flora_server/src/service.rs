use flora_core::{manager::FloraManager, seed::{FloraProtonSeed, FloraSeed, FloraSeedType, FloraWineSeed}};
use tonic::{Request, Response, Status};

use crate::proto::{self, ListSeedItem, ListSeedRequest, ListSeedResponse, RunAppRequest, RunAppResponse, RunExecutableRequest, RunExecutableResponse, SeedType::{Proton, Unspecified}, flora_manager_service_server::FloraManagerService};

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
    async fn create_seed(&self, request: Request<proto::CreateSeedRequest>) -> Result<Response<proto::CreateSeedResponse>, Status>
    {
        let req = request.into_inner();
        let mut new_seed = FloraSeed::default();
        new_seed.seed_type = match proto::SeedType::try_from(req.seed_type) {
            Ok(proto::SeedType::Wine) => FloraSeedType::Wine(
                    FloraWineSeed {
                        wine_prefix: req.prefix,
                        wine_runtime: req.runtime,
                    }
                ),
            Ok(Proton) => FloraSeedType::Proton(
                FloraProtonSeed {
                    proton_prefix: req.prefix,
                    proton_runtime: req.runtime,
                    game_id: req.game_id,
                    store: req.game_store
                }
            ),
            Ok(Unspecified) | Err(_) => {
                return Err(Status::new(tonic::Code::InvalidArgument, "Invalid seed type"));
            }
        };

        self.manager.create_seed(&req.seed_name, &new_seed)
            .map_err(|e| Status::new(tonic::Code::InvalidArgument, e.to_string()))?;

        Ok(Response::new(proto::CreateSeedResponse{}))
    }

    async fn update_seed(&self, request: Request<proto::UpdateSeedRequest>) -> Result<Response<proto::UpdateSeedResponse>,Status>
    {
        let req = request.into_inner();
        let mut seed = self.manager.get_seed(&req.seed_name).map_err(|e| Status::new(tonic::Code::InvalidArgument, e.to_string()))?;
        match seed.seed_type {
            FloraSeedType::Wine(ref mut wine_settings) => {
                if req.prefix.is_some() {
                    wine_settings.wine_prefix = req.prefix.clone();
                }
                if req.runtime.is_some() {
                    wine_settings.wine_runtime = req.runtime.clone();
                }
            },
            FloraSeedType::Proton(ref mut proton_settings) => {
                if req.prefix.is_some() {
                    proton_settings.proton_prefix = req.prefix.clone();
                }
                if req.runtime.is_some() {
                    proton_settings.proton_runtime = req.runtime.clone();
                }
                if req.game_id.is_some() {
                    proton_settings.game_id = req.game_id.clone();
                }
                if req.game_store.is_some() {
                    proton_settings.store = req.game_store.clone();
                }
            },
            FloraSeedType::None => return Err(Status::new(tonic::Code::InvalidArgument, "Seed is not valid")),
        };
        self.manager.update_seed(&req.seed_name, &seed).map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        Ok(Response::new(proto::UpdateSeedResponse{}))
    }

    async fn list_seed(&self, _: Request<ListSeedRequest>) -> Result<Response<ListSeedResponse>, Status>
    {
        let seeds = self.manager.list_seed().map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        let mut response = ListSeedResponse::default();
        response.seeds = seeds.iter().map(|e| ListSeedItem {
            seed_name: e.seed_name.clone(),
            seed_type: e.seed_type.clone()
        }).collect();

        Ok(Response::new(response))
    }

    async fn run_executable(&self, request: Request<RunExecutableRequest>) -> Result<Response<RunExecutableResponse>, Status>
    {
        let req = request.into_inner();

        let command_param = shlex::split(&req.command_line).ok_or(Status::new(tonic::Code::InvalidArgument, "Invalid command line"))?;
        let args: Vec<_> = command_param.iter().map(AsRef::as_ref).collect();

        self.manager.seed_run_executable(&req.seed_name, &args, true, false).map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        Ok(Response::new(RunExecutableResponse {  }))
    }

    async fn run_app(&self, request: Request<RunAppRequest>) -> Result<Response<RunAppResponse>, Status>
    {
        let req = request.into_inner();
        self.manager.seed_run_app(&req.seed_name, &Some(&req.app_name), true, false).map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        Ok(Response::new(RunAppResponse {  }))
    }
}
