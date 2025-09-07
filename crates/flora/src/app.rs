use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{config::FloraConfig, errors::FloraError};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub(crate) enum FloraAppType {
    /// Wine App
    Wine(FloraAppWineConfig),
    Other,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FloraAppWineConfig {
    pub wine_prefix: Option<String>,
    pub wine_runner: Option<String>,
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
}

pub struct FloraAppWineOptions {
    pub pretty_name: Option<String>,
    pub executable_location: String,
    pub wine_prefix: Option<String>,
    pub wine_runner: Option<String>,
}

impl FloraApp {
    /// Converts AppOptions passed from the frontend into App, which is the actual configuration used to launch apps.
    pub(crate) fn from_options(
        config: &FloraConfig,
        name: &String,
        app_options: &FloraAppOptions,
    ) -> Result<FloraApp, FloraError> {
        // Wine config must be set up in flora.toml first
        if let Some(wine_config) = &config.wine {
            match app_options {
                FloraAppOptions::WineOptions(opts) => Ok(FloraApp {
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
                                        let mut new_path = PathBuf::from(wine_config.wine_prefix_location.clone());
                                        new_path.push(prefix);

                                        Some(new_path.into_os_string().into_string().map_err(|_| FloraError::InternalError)?)
                                    } else {
                                        // Prefix is absolute
                                        Some(prefix)
                                    }
                                }
                            }
                        },
                        wine_runner: opts.wine_runner.to_owned(),
                    }),
                }),
            }
        } else {
            Err(FloraError::MissingRunner)
        }
    }
}
