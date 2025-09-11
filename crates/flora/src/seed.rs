use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    config::FloraConfig,
    errors::FloraError,
    requests::{FloraCreateSeed, FloraCreateSeedApp, FloraSeedAppOperations, FloraUpdateSeed},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FloraSeedType {
    /// Wine App
    Wine(FloraWineSeed),
    /// Proton App
    Proton(FloraProtonSeed),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FloraWineSeed {
    pub wine_prefix: Option<String>,
    pub wine_runtime: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FloraProtonSeed {
    pub proton_prefix: Option<String>,
    pub proton_runtime: Option<String>,
    pub game_id: Option<String>,
    pub store: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FloraSeed {
    pub settings: Option<Box<FloraSeedSettings>>,
    pub apps: Vec<FloraSeedApp>,

    #[serde(flatten)]
    pub seed_type: FloraSeedType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FloraSeedSettings {
    pub launcher_command: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FloraSeedApp {
    pub application_name: String,
    pub application_location: String,
}
impl<'a> From<&FloraCreateSeedApp<'a>> for FloraSeedApp {
    fn from(value: &FloraCreateSeedApp) -> Self {
        Self {
            application_name: String::from(value.application_name),
            application_location: String::from(value.application_location),
        }
    }
}

// Instance functions
impl FloraSeed {
    pub(crate) fn merge_options(&mut self, seed_opts: &FloraUpdateSeed) -> Result<(), FloraError> {
        match (seed_opts, &mut self.seed_type) {
            (
                FloraUpdateSeed::WineOptions(flora_update_wine_seed),
                FloraSeedType::Wine(flora_wine_seed),
            ) => {
                if let Some(wine_prefix) = flora_update_wine_seed.wine_prefix {
                    flora_wine_seed.wine_prefix = Some(String::from(wine_prefix));
                }
                if let Some(wine_runtime) = flora_update_wine_seed.wine_runtime {
                    flora_wine_seed.wine_runtime = Some(String::from(wine_runtime));
                }

                Ok(())
            }
            (
                FloraUpdateSeed::ProtonOptions(flora_update_proton_seed),
                FloraSeedType::Proton(flora_proton_seed),
            ) => {
                if let Some(proton_prefix) = flora_update_proton_seed.proton_prefix {
                    flora_proton_seed.proton_prefix = Some(String::from(proton_prefix));
                }
                if let Some(proton_runtime) = flora_update_proton_seed.proton_runtime {
                    flora_proton_seed.proton_runtime = Some(String::from(proton_runtime));
                }
                if let Some(game_id) = flora_update_proton_seed.game_id {
                    flora_proton_seed.game_id = Some(String::from(game_id));
                }
                if let Some(store) = flora_update_proton_seed.store {
                    flora_proton_seed.store = Some(String::from(store));
                }

                Ok(())
            }
            _ => Err(FloraError::SeedUpdateMismatch),
        }
    }

    pub(crate) fn update_apps(
        &mut self,
        seed_ops: &Vec<FloraSeedAppOperations>,
    ) -> Result<(), FloraError> {
        for op in seed_ops {
            match &op {
                FloraSeedAppOperations::Add(flora_create_seed_app) => {
                    if !self
                        .apps
                        .iter()
                        .any(|i| i.application_name == flora_create_seed_app.application_name)
                    {
                        self.apps.push(FloraSeedApp::from(flora_create_seed_app));
                    } else {
                        return Err(FloraError::SeedAppExists);
                    }
                }
                FloraSeedAppOperations::Update(flora_update_seed_app) => {
                    if let Some(idx) = self
                        .apps
                        .iter()
                        .position(|i| i.application_name == flora_update_seed_app.application_name)
                    {
                        let app = self.apps.get_mut(idx).ok_or(FloraError::SeedNoApp)?;

                        if let Some(app_location) = flora_update_seed_app.application_location {
                            app.application_location = String::from(app_location);
                        }
                    } else {
                        return Err(FloraError::SeedNoApp);
                    }
                }
                FloraSeedAppOperations::Rename(flora_rename_seed_app) => {
                    if let Some(idx) = self.apps.iter().position(|i| {
                        i.application_name == flora_rename_seed_app.old_application_name
                    }) {
                        let app = self.apps.get_mut(idx).ok_or(FloraError::SeedNoApp)?;

                        app.application_name =
                            String::from(flora_rename_seed_app.new_application_name);
                    } else {
                        return Err(FloraError::SeedNoApp);
                    }
                }
                FloraSeedAppOperations::Delete(flora_delete_seed_app) => {
                    if let Some(idx) = self
                        .apps
                        .iter()
                        .position(|i| i.application_name == flora_delete_seed_app.application_name)
                    {
                        self.apps.remove(idx);
                    } else {
                        return Err(FloraError::SeedNoApp);
                    }
                }
            }
        }

        Ok(())
    }
}

// Static functions
impl FloraSeed {
    /// Converts Options passed from the frontend into a Seed, the actual configuration format used to launch Flora seeds.
    pub(crate) fn from_options(
        config: &FloraConfig,
        seed_opts: &FloraCreateSeed,
    ) -> Result<FloraSeed, FloraError> {
        // Wine config must be set up in flora.toml first
        match seed_opts {
            FloraCreateSeed::WineOptions(opts) => {
                if let Some(wine_opts) = &config.wine {
                    Ok(FloraSeed {
                        settings: opts
                            .settings
                            .as_ref()
                            .map(|m| FloraSeedSettings {
                                launcher_command: m.launcher_command.map(String::from),
                            })
                            .map(Box::from)
                            .to_owned(),
                        apps: match &opts.default_application {
                            Some(default_application) => {
                                vec![FloraSeedApp {
                                    application_name: default_application
                                        .application_name
                                        .to_owned(),
                                    application_location: default_application
                                        .application_location
                                        .to_owned(),
                                }]
                            }
                            None => vec![],
                        },

                        seed_type: FloraSeedType::Wine(FloraWineSeed {
                            wine_prefix: {
                                match opts.wine_prefix.to_owned() {
                                    None => Some(wine_opts.default_wine_prefix.clone()),
                                    Some(prefix) => {
                                        if Path::new(&prefix).is_relative() {
                                            // Prefix is relative to wine prefix location
                                            let mut new_path = PathBuf::from(
                                                wine_opts.wine_prefix_location.clone(),
                                            );
                                            new_path.push(prefix);

                                            Some(String::from(new_path.to_string_lossy()))
                                        } else {
                                            // Prefix is absolute
                                            Some(String::from(prefix))
                                        }
                                    }
                                }
                            },
                            wine_runtime: opts.wine_runner.map(String::from),
                        }),
                    })
                } else {
                    Err(FloraError::MissingRunnerConfig)
                }
            }
            FloraCreateSeed::ProtonOptions(opts) => {
                if let Some(proton_opts) = &config.proton {
                    Ok(FloraSeed {
                        settings: opts
                            .settings
                            .as_ref()
                            .map(|m| FloraSeedSettings {
                                launcher_command: m.launcher_command.map(String::from),
                            })
                            .map(Box::from)
                            .to_owned(),
                        apps: match &opts.default_application {
                            Some(default_application) => {
                                vec![FloraSeedApp {
                                    application_name: default_application
                                        .application_name
                                        .to_owned(),
                                    application_location: default_application
                                        .application_location
                                        .to_owned(),
                                }]
                            }
                            None => vec![],
                        },

                        seed_type: FloraSeedType::Proton(FloraProtonSeed {
                            proton_prefix: match opts.proton_prefix.to_owned() {
                                None => Some(proton_opts.default_proton_prefix.clone()),
                                Some(prefix) => {
                                    if Path::new(&prefix).is_relative() {
                                        // Prefix is relative to wine prefix location
                                        let mut new_path = PathBuf::from(
                                            proton_opts.proton_prefix_location.clone(),
                                        );
                                        new_path.push(prefix);

                                        Some(String::from(new_path.to_string_lossy()))
                                    } else {
                                        // Prefix is absolute
                                        Some(String::from(prefix))
                                    }
                                }
                            },
                            proton_runtime: opts.proton_runtime.map(String::from),
                            game_id: opts.game_id.map(String::from),
                            store: opts.store.map(String::from),
                        }),
                    })
                } else {
                    Err(FloraError::MissingRunnerConfig)
                }
            }
        }
    }
}
