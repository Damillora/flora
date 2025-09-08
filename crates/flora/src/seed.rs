use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    config::FloraConfig,
    errors::FloraError,
    requests::{FloraCreateSeed, FloraUpdateSeed},
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
    pub apps: Vec<FloraSeedApp>,

    #[serde(flatten)]
    pub seed_type: FloraSeedType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FloraSeedApp {
    pub application_name: String,
    pub application_location: String,
}
// Instance functions
impl FloraSeed {
    pub(crate) fn merge_options(&mut self, seed_opts: &FloraUpdateSeed) -> Result<(), FloraError> {
        match (seed_opts, &mut self.seed_type) {
            (
                FloraUpdateSeed::WineOptions(flora_update_wine_seed),
                FloraSeedType::Wine(flora_wine_seed),
            ) => {
                if let Some(wine_prefix) = flora_update_wine_seed.wine_prefix.clone() {
                    flora_wine_seed.wine_prefix = Some(wine_prefix);
                }
                if let Some(wine_runtime) = flora_update_wine_seed.wine_runtime.clone() {
                    flora_wine_seed.wine_runtime = Some(wine_runtime);
                }

                Ok(())
            }
            (
                FloraUpdateSeed::ProtonOptions(flora_update_proton_seed),
                FloraSeedType::Proton(flora_proton_seed),
            ) => {
                if let Some(proton_prefix) = flora_update_proton_seed.proton_prefix.clone() {
                    flora_proton_seed.proton_prefix = Some(proton_prefix);
                }
                if let Some(proton_runtime) = flora_update_proton_seed.proton_runtime.clone() {
                    flora_proton_seed.proton_runtime = Some(proton_runtime);
                }
                if let Some(game_id) = flora_update_proton_seed.game_id.clone() {
                    flora_proton_seed.game_id = Some(game_id);
                }
                if let Some(store) = flora_update_proton_seed.store.clone() {
                    flora_proton_seed.store = Some(store);
                }

                Ok(())
            }
            _ => Err(FloraError::SeedUpdateMismatch),
        }
    }
}

// Static functions
impl FloraSeed {
    /// Converts Options passed from the frontend into a Seed, the actual configuration format used to launch Flora seeds.
    pub(crate) fn from_options(
        config: &FloraConfig,
        name: &String,
        seed_opts: &FloraCreateSeed,
    ) -> Result<FloraSeed, FloraError> {
        // Wine config must be set up in flora.toml first
        match seed_opts {
            FloraCreateSeed::WineOptions(opts) => {
                if let Some(wine_opts) = &config.wine {
                    Ok(FloraSeed {
                        apps: vec![FloraSeedApp {
                            application_name: match &opts.default_application_name {
                                Some(pretty_name) => pretty_name.to_owned(),
                                None => name.to_owned(),
                            },
                            application_location: opts.default_application_location.to_owned(),
                        }],

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

                                            Some(
                                                new_path
                                                    .into_os_string()
                                                    .into_string()
                                                    .map_err(|_| FloraError::InternalError)?,
                                            )
                                        } else {
                                            // Prefix is absolute
                                            Some(prefix)
                                        }
                                    }
                                }
                            },
                            wine_runtime: opts.wine_runner.to_owned(),
                        }),
                    })
                } else {
                    Err(FloraError::MissingRunnerConfig)
                }
            }
            FloraCreateSeed::ProtonOptions(opts) => {
                if let Some(proton_opts) = &config.proton {
                    Ok(FloraSeed {
                        apps: vec![FloraSeedApp {
                            application_name: match &opts.default_application_name {
                                Some(pretty_name) => pretty_name.to_owned(),
                                None => name.to_owned(),
                            },
                            application_location: opts.default_application_location.to_owned(),
                        }],
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

                                        Some(
                                            new_path
                                                .into_os_string()
                                                .into_string()
                                                .map_err(|_| FloraError::InternalError)?,
                                        )
                                    } else {
                                        // Prefix is absolute
                                        Some(prefix)
                                    }
                                }
                            },
                            proton_runtime: opts.proton_runtime.to_owned(),
                            game_id: opts.game_id.to_owned(),
                            store: opts.store.to_owned(),
                        }),
                    })
                } else {
                    Err(FloraError::MissingRunnerConfig)
                }
            }
        }
    }
}
