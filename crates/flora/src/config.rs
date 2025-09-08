use std::fs;

use serde::{Deserialize, Serialize};

use crate::{dirs::FloraDirs, errors::FloraError};

#[derive(Serialize, Deserialize, Debug)]
pub struct FloraConfig {
    pub wine: Option<FloraWineConfig>,
    pub proton: Option<FloraProtonConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FloraWineConfig {
    pub wine_prefix_location: String,

    pub default_wine_prefix: String,

    pub default_wine_runtime: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FloraProtonConfig {
    pub proton_prefix_location: String,

    pub default_proton_prefix: String,

    pub default_proton_runtime: String,
}

impl FloraConfig {
    pub fn read_config(dirs: &FloraDirs) -> Result<FloraConfig, FloraError> {
        let config_path = {
            let mut config_path = dirs.flora_root.clone();
            config_path.push("flora.toml");

            config_path
        };

        let default_config = FloraConfig {
            wine: Some(FloraWineConfig {
                wine_prefix_location: {
                    let prefixes_dir = dirs.get_prefixes_root();

                    prefixes_dir
                        .into_os_string()
                        .into_string()
                        .map_err(|_| FloraError::InternalError)?
                },
                default_wine_prefix: {
                    let prefixes_dir = dirs.get_fallback_prefix();

                    prefixes_dir
                        .into_os_string()
                        .into_string()
                        .map_err(|_| FloraError::InternalError)?
                },
                default_wine_runtime: None,
            }),
            proton: Some(FloraProtonConfig {
                proton_prefix_location: {
                    let prefixes_dir = dirs.get_prefixes_root();

                    prefixes_dir
                        .into_os_string()
                        .into_string()
                        .map_err(|_| FloraError::InternalError)?
                },
                default_proton_prefix: {
                    let prefixes_dir = dirs.get_fallback_prefix_proton();

                    prefixes_dir
                        .into_os_string()
                        .into_string()
                        .map_err(|_| FloraError::InternalError)?
                },
                default_proton_runtime: String::from(""),
            }),
        };

        if !fs::exists(&config_path)? {
            let new_config_toml =
                toml::to_string(&default_config).map_err(|_| FloraError::InternalError)?;

            // Write the content to the file
            fs::write(&config_path, new_config_toml.as_bytes())?;
        }

        let config_toml = fs::read_to_string(&config_path)?;
        let config = toml::from_str(config_toml.as_str())?;

        Ok(config)
    }
}
