use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{config::FloraConfig, errors::FloraError};

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
    pub pretty_name: String,
    pub executable_location: String,

    #[serde(flatten)]
    pub seed_type: FloraSeedType,
}

pub enum FloraCreateSeed {
    WineOptions(FloraCreateWineSeed),
    ProtonOptions(FloraCreateProtonSeed),
}

pub struct FloraCreateWineSeed {
    pub pretty_name: Option<String>,
    pub executable_location: String,
    pub wine_prefix: Option<String>,
    pub wine_runner: Option<String>,
}

pub struct FloraCreateProtonSeed {
    pub pretty_name: Option<String>,
    pub executable_location: String,
    pub proton_prefix: Option<String>,
    pub proton_runtime: Option<String>,
    pub game_id: Option<String>,
    pub store: Option<String>,
}

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
                        pretty_name: match &opts.pretty_name {
                            Some(pretty_name) => pretty_name.to_owned(),
                            None => name.to_owned(),
                        },
                        executable_location: opts.executable_location.to_owned(),

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
                        pretty_name: match &opts.pretty_name {
                            Some(pretty_name) => pretty_name.to_owned(),
                            None => name.to_owned(),
                        },
                        executable_location: opts.executable_location.to_owned(),
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
