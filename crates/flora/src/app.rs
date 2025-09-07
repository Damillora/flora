use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{config::FloraConfig, errors::FloraError};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub(crate) enum FloraAppType {
    /// Wine App
    Wine(FloraAppWineConfig),
    Proton(FloraAppProtonConfig),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FloraAppWineConfig {
    pub wine_prefix: Option<String>,
    pub wine_runtime: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FloraAppProtonConfig {
    pub proton_prefix: Option<String>,
    pub proton_runtime: Option<String>,
    pub game_id: Option<String>,
    pub store: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FloraApp {
    pub pretty_name: String,
    pub executable_location: String,

    #[serde(flatten)]
    pub app_type: FloraAppType,
}

pub enum FloraAppOptions {
    WineOptions(FloraAppWineOptions),
    ProtonOptions(FloraAppProtonOptions),
}

pub struct FloraAppWineOptions {
    pub pretty_name: Option<String>,
    pub executable_location: String,
    pub wine_prefix: Option<String>,
    pub wine_runner: Option<String>,
}

pub struct FloraAppProtonOptions {
    pub pretty_name: Option<String>,
    pub executable_location: String,
    pub proton_prefix: Option<String>,
    pub proton_runtime: Option<String>,
    pub game_id: Option<String>,
    pub store: Option<String>,
}

impl FloraApp {
    /// Converts AppOptions passed from the frontend into App, which is the actual configuration used to launch apps.
    pub(crate) fn from_options(
        config: &FloraConfig,
        name: &String,
        app_options: &FloraAppOptions,
    ) -> Result<FloraApp, FloraError> {
        // Wine config must be set up in flora.toml first
        match app_options {
            FloraAppOptions::WineOptions(opts) => {
                if let Some(wine_config) = &config.wine {
                    Ok(FloraApp {
                        pretty_name: match &opts.pretty_name {
                            Some(pretty_name) => pretty_name.to_owned(),
                            None => name.to_owned(),
                        },
                        executable_location: opts.executable_location.to_owned(),

                        app_type: FloraAppType::Wine(FloraAppWineConfig {
                            wine_prefix: {
                                match opts.wine_prefix.to_owned() {
                                    None => Some(wine_config.default_wine_prefix.clone()),
                                    Some(prefix) => {
                                        if Path::new(&prefix).is_relative() {
                                            // Prefix is relative to wine prefix location
                                            let mut new_path = PathBuf::from(
                                                wine_config.wine_prefix_location.clone(),
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
            FloraAppOptions::ProtonOptions(opts) => {
                if let Some(proton_config) = &config.proton {
                    Ok(FloraApp {
                        pretty_name: match &opts.pretty_name {
                            Some(pretty_name) => pretty_name.to_owned(),
                            None => name.to_owned(),
                        },
                        executable_location: opts.executable_location.to_owned(),
                        app_type: FloraAppType::Proton(FloraAppProtonConfig {
                            proton_prefix: match opts.proton_prefix.to_owned() {
                                None => Some(proton_config.default_proton_prefix.clone()),
                                Some(prefix) => {
                                    if Path::new(&prefix).is_relative() {
                                        // Prefix is relative to wine prefix location
                                        let mut new_path = PathBuf::from(
                                            proton_config.proton_prefix_location.clone(),
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
