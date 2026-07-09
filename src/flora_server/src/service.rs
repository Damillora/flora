use flora_core::{
    errors::FloraError,
    manager::FloraManager,
    seed::{FloraProtonSeed, FloraSeed, FloraSeedApp, FloraSeedType, FloraWineSeed},
};
use tokio::fs::File;
use tonic::{Request, Response, Status};

use crate::proto::{
    self, CreateAppRequest, CreateAppResponse, DeleteAppRequest, DeleteAppResponse, DeleteEnvironmentRequest, DeleteEnvironmentResponse, ListAppItem, ListEnvironmentItem, ListEnvironmentRequest, ListEnvironmentResponse, ListSeedItem, ListSeedRequest, ListSeedResponse, RenameAppRequest, RenameAppResponse, RunAppRequest, RunAppResponse, RunConfigRequest, RunConfigResponse, RunExecutableRequest, RunExecutableResponse, RunTricksRequest, RunTricksResponse, SeedType::{Proton, Unspecified}, SetEnvironmentRequest, SetEnvironmentResponse, UpdateAppRequest, UpdateAppResponse, flora_manager_service_server::FloraManagerService
};

pub struct FloraManagerServiceImpl {
    manager: FloraManager,
}

impl FloraManagerServiceImpl {
    pub fn new(manager: FloraManager) -> Self {
        Self { manager }
    }
}

// Error handling functions
fn invalid_error(error: FloraError) -> Status {
    invalid_error_custom(error.to_string())
}

fn invalid_error_custom(error: String) -> Status {
    Status::new(tonic::Code::InvalidArgument, error)
}

fn internal_error(error: FloraError) -> Status {
    internal_error_custom(error.to_string())
}

fn internal_error_custom(error: String) -> Status {
    Status::new(tonic::Code::Internal, error)
}

#[tonic::async_trait]
impl FloraManagerService for FloraManagerServiceImpl {
    async fn create_seed(
        &self,
        request: Request<proto::CreateSeedRequest>,
    ) -> Result<Response<proto::CreateSeedResponse>, Status> {
        let req = request.into_inner();
        let mut new_seed = FloraSeed::default();
        new_seed.seed_type = match proto::SeedType::try_from(req.seed_type) {
            Ok(proto::SeedType::Wine) => FloraSeedType::Wine(FloraWineSeed {
                wine_prefix: req.prefix,
                wine_runtime: req.runtime,
            }),
            Ok(Proton) => FloraSeedType::Proton(FloraProtonSeed {
                proton_prefix: req.prefix,
                proton_runtime: req.runtime,
                game_id: req.game_id,
                store: req.game_store,
            }),
            Ok(Unspecified) | Err(_) => {
                return Err(invalid_error_custom(String::from("Incorrect seed type")));
            }
        };

        self.manager
            .create_seed(&req.seed_name, &new_seed)
            .map_err(invalid_error)?;

        Ok(Response::new(proto::CreateSeedResponse {}))
    }

    async fn get_seed(
        &self,
        request: Request<proto::GetSeedRequest>,
    ) -> Result<Response<proto::GetSeedResponse>, Status> {
        let req = request.into_inner();
        let seed = self.manager.get_seed(&req.seed_name).map_err(invalid_error)?;
        let res = proto::GetSeedResponse {
            seed_name: req.seed_name,
            seed_type: match seed.seed_type {
                FloraSeedType::Wine(_) => String::from("wine"),
                FloraSeedType::Proton(_) => String::from("proton"),
                FloraSeedType::None => String::from("none")
            },
            launcher_command: seed.settings.clone().map(|s| s.launcher_command).flatten(),
            apps: seed.get_apps().iter().map(|a| proto::ListAppItem {
                app_name: a.application_name.clone(),
                app_location: a.application_location.clone(),
            }).collect()
        };

        Ok(Response::new(res))
    }

    async fn update_seed(
        &self,
        request: Request<proto::UpdateSeedRequest>,
    ) -> Result<Response<proto::UpdateSeedResponse>, Status> {
        let req = request.into_inner();
        let mut seed = self
            .manager
            .get_seed(&req.seed_name)
            .map_err(invalid_error)?;
        match seed.seed_type {
            FloraSeedType::Wine(ref mut wine_settings) => {
                if req.prefix.is_some() {
                    wine_settings.wine_prefix = req.prefix.clone();
                }
                if req.runtime.is_some() {
                    wine_settings.wine_runtime = req.runtime.clone();
                }
            }
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
            }
            FloraSeedType::None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "Seed is not valid",
                ));
            }
        };
        self.manager
            .update_seed(&req.seed_name, &seed)
            .map_err(internal_error)?;

        Ok(Response::new(proto::UpdateSeedResponse {}))
    }

    async fn list_seed(
        &self,
        _: Request<ListSeedRequest>,
    ) -> Result<Response<ListSeedResponse>, Status> {
        let seeds = self.manager.list_seed().map_err(internal_error)?;
        let seed_item: Result<Vec<_>, Status> =  seeds
            .iter()
            .map(|e| {
                let seed = self.manager.get_seed(&e.seed_name).map_err(internal_error)?;
                let (prefix, runtime, game_id, game_store) = match &seed.seed_type {
                    FloraSeedType::Wine(wine) => (wine.wine_prefix.clone(), wine.wine_runtime.clone(), None, None),
                    FloraSeedType::Proton(proton) => (proton.proton_prefix.clone(), proton.proton_runtime.clone(), proton.game_id.clone(), proton.store.clone()),
                    _ => unimplemented!(),
                };
                let launcher_command = seed.settings.clone().map(|e| e.launcher_command).flatten();
                let apps: Vec<_> = seed.get_apps().iter().map(|e| ListAppItem {
                    app_name: e.application_name.clone(),
                    app_location: e.application_location.clone(),
                }).collect();

                Ok(ListSeedItem {
                    seed_name: e.seed_name.clone(),
                    seed_type: e.seed_type.clone(),
                    prefix: prefix,
                    runtime: runtime,
                    game_id: game_id,
                    game_store: game_store,
                    launcher_command: launcher_command,
                    apps: apps,
                })
            })
            .collect();
        Ok(Response::new(ListSeedResponse {
            seeds: seed_item?,
        }))
    }

    async fn delete_seed(
        &self,
        request: Request<proto::DeleteSeedRequest>,
    ) -> Result<Response<proto::DeleteSeedResponse>, Status> {
        let req = request.into_inner();
        self.manager
            .delete_seed(&req.seed_name)
            .map_err(internal_error)?;

        Ok(Response::new(proto::DeleteSeedResponse {}))
    }

    async fn create_app(
        &self,
        request: Request<CreateAppRequest>,
    ) -> Result<Response<CreateAppResponse>, Status> {
        let req = request.into_inner();
        let mut seed = self
            .manager
            .get_seed(&req.seed_name)
            .map_err(invalid_error)?;
        seed.add_app(FloraSeedApp {
            application_name: req.app_name,
            application_location: req.app_location,
            category: None,
        })
        .map_err(invalid_error)?;
        self.manager
            .update_seed(&req.seed_name, &seed)
            .map_err(internal_error)?;

        Ok(Response::new(CreateAppResponse {}))
    }

    async fn update_app(
        &self,
        request: Request<UpdateAppRequest>,
    ) -> Result<Response<UpdateAppResponse>, Status> {
        let req = request.into_inner();
        let mut seed = self
            .manager
            .get_seed(&req.seed_name)
            .map_err(invalid_error)?;

        let mut app = seed.get_app(&req.app_name).map_err(invalid_error)?;
        app.application_location = req.app_location;
        seed.update_app(&req.app_name, app).map_err(invalid_error)?;

        self.manager
            .update_seed(&req.seed_name, &seed)
            .map_err(internal_error)?;

        Ok(Response::new(UpdateAppResponse {}))
    }

    async fn rename_app(
        &self,
        request: Request<RenameAppRequest>,
    ) -> Result<Response<RenameAppResponse>, Status> {
        let req = request.into_inner();
        let mut seed = self
            .manager
            .get_seed(&req.seed_name)
            .map_err(invalid_error)?;

        let mut app = seed.get_app(&req.app_name).map_err(invalid_error)?;
        app.application_name = req.new_app_name;
        seed.update_app(&req.app_name, app).map_err(invalid_error)?;

        self.manager
            .update_seed(&req.seed_name, &seed)
            .map_err(internal_error)?;

        Ok(Response::new(RenameAppResponse {}))
    }

    async fn delete_app(
        &self,
        request: Request<DeleteAppRequest>,
    ) -> Result<Response<DeleteAppResponse>, Status> {
        let req = request.into_inner();

        let mut seed = self
            .manager
            .get_seed(&req.seed_name)
            .map_err(invalid_error)?;
        seed.delete_app(&req.app_name).map_err(invalid_error)?;

        self.manager
            .update_seed(&req.seed_name, &seed)
            .map_err(internal_error)?;

        Ok(Response::new(DeleteAppResponse {}))
    }

    async fn list_environment(
        &self,
        request: Request<ListEnvironmentRequest>,
    ) -> Result<Response<ListEnvironmentResponse>, Status> {
        let req = request.into_inner();
        let seed = self
            .manager
            .get_seed(&req.seed_name)
            .map_err(invalid_error)?;

        let items: Vec<_> = seed
            .get_env()
            .iter()
            .map(|t| ListEnvironmentItem {
                env_name: t.0.clone(),
                env_value: t.1.clone(),
            })
            .collect();

        Ok(Response::new(ListEnvironmentResponse { items }))
    }

    async fn set_environment(
        &self,
        request: Request<SetEnvironmentRequest>,
    ) -> Result<Response<SetEnvironmentResponse>, Status> {
        let req = request.into_inner();
        let mut seed = self
            .manager
            .get_seed(&req.seed_name)
            .map_err(invalid_error)?;
        seed.update_env(&req.env_name, &req.env_value);

        self.manager
            .update_seed(&req.seed_name, &seed)
            .map_err(internal_error)?;

        Ok(Response::new(SetEnvironmentResponse {}))
    }

    async fn delete_environment(
        &self,
        request: Request<DeleteEnvironmentRequest>,
    ) -> Result<Response<DeleteEnvironmentResponse>, Status> {
        let req = request.into_inner();
        let mut seed = self
            .manager
            .get_seed(&req.seed_name)
            .map_err(invalid_error)?;
        seed.delete_env(&req.env_name);

        self.manager
            .update_seed(&req.seed_name, &seed)
            .map_err(internal_error)?;

        Ok(Response::new(DeleteEnvironmentResponse {}))
    }

    async fn run_config(
        &self,
        request: Request<RunConfigRequest>,
    ) -> Result<Response<RunConfigResponse>, Status> {
        let req = request.into_inner();

        self.manager
            .seed_config(&req.seed_name, &None, true, false)
            .map_err(invalid_error)?;

        Ok(Response::new(RunConfigResponse {}))
    }

    async fn run_tricks(
        &self,
        request: Request<RunTricksRequest>,
    ) -> Result<Response<RunTricksResponse>, Status> {
        let req = request.into_inner();

        self.manager
            .seed_tricks(&req.seed_name, &None, true, false)
            .map_err(invalid_error)?;

        Ok(Response::new(RunTricksResponse {}))
    }

    async fn run_executable(
        &self,
        request: Request<RunExecutableRequest>,
    ) -> Result<Response<RunExecutableResponse>, Status> {
        let req = request.into_inner();

        let command_param = shlex::split(&req.command_line)
            .ok_or(invalid_error_custom(String::from("Invalid command line")))?;
        let args: Vec<_> = command_param.iter().map(AsRef::as_ref).collect();

        self.manager
            .seed_run_executable(&req.seed_name, &args, true, false)
            .map_err(internal_error)?;

        Ok(Response::new(RunExecutableResponse {}))
    }

    async fn run_app(
        &self,
        request: Request<RunAppRequest>,
    ) -> Result<Response<RunAppResponse>, Status> {
        let req = request.into_inner();
        self.manager
            .seed_run_app(&req.seed_name, &Some(&req.app_name), true, false)
            .map_err(invalid_error)?;

        Ok(Response::new(RunAppResponse {}))
    }
}
