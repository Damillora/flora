use std::
    collections::BTreeMap
;

use serde::{Deserialize, Serialize};

use crate::{errors::FloraError};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FloraSeedType {
    /// Wine App
    Wine(FloraWineSeed),
    /// Proton App
    Proton(FloraProtonSeed),
    /// Empty App
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FloraWineSeed {
    pub wine_prefix: Option<String>,
    pub wine_runtime: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FloraProtonSeed {
    pub proton_prefix: Option<String>,
    pub proton_runtime: Option<String>,
    pub game_id: Option<String>,
    pub store: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FloraSeed {
    pub settings: Option<Box<FloraSeedSettings>>,
    apps: Vec<FloraSeedApp>,
    env: Option<BTreeMap<String, String>>,

    #[serde(flatten)]
    pub seed_type: FloraSeedType,
}

// Create seed
impl Default for FloraSeed {
    fn default() -> Self {
        Self {
            settings: None,
            apps: Vec::new(),
            env: None,

            seed_type: FloraSeedType::None,
        }
    }
}

// App functions
impl FloraSeed {
    pub fn get_apps(&self) -> Vec<FloraSeedApp> {
        self.apps.clone()
    }

    pub fn get_app(&self, app_name: &str) -> Result<FloraSeedApp, FloraError> {
        if let Some(idx) = self.apps.iter().position(|i| {
            i.application_name == app_name
        })
        && let Some(app) = self.apps.get(idx)
        {
            return Ok(app.clone());
        }

        Err(FloraError::AppNotFound(app_name.to_string()))
    }

    pub fn add_app(&mut self, new_app: FloraSeedApp) -> Result<(), FloraError> {
        if self.apps.iter().position(|i| {
            i.application_name == new_app.application_name
        }).is_none() {
            self.apps.push(new_app);

            Ok(())
        } else {
            Err(FloraError::AppExists(new_app.application_name))
        }
    }

    pub fn delete_app(&mut self, app_name: &str) -> Result<(), FloraError> {
        if let Some(idx) = self
            .apps
            .iter()
            .position(|i| i.application_name == app_name)
        {
            self.apps.remove(idx);

            Ok(())
        } else {
            Err(FloraError::AppNotFound(app_name.to_string()))
        }
    }

    pub fn rename_app(&mut self, old_app_name: &str, new_app_name: &str) -> Result<(), FloraError>  {
        if let Some(idx) = self.apps.iter().position(|i| {
            i.application_name == old_app_name
        }) {
            if let Some(app) = self.apps.get_mut(idx)
            {
                app.application_name =
                    String::from(new_app_name);
            }

            Ok(())
        } else {
            Err(FloraError::AppNotFound(old_app_name.to_string()))
        }
    }


    pub fn update_app(&mut self, app_name: &str, app: FloraSeedApp) -> Result<(), FloraError> {
        if let Some(idx) = self.apps.iter().position(|i| {
            i.application_name == app_name
        }) {
            self.apps[idx] = app;

            Ok(())
        } else {
            Err(FloraError::AppNotFound(app_name.to_string()))
        }
    }

    pub fn get_app_or_default(&self, app_name: &Option<&str>) -> Result<FloraSeedApp, FloraError> {
        let app_entry = match &app_name {
            Some(app_name) => self.apps
                .iter()
                .find(|item| &item.application_name == app_name).ok_or(FloraError::AppNotFound(app_name.to_string()))?,
            None => self.apps.first().ok_or(FloraError::AppNotFound(String::from("")))?,
        };

        Ok(app_entry.clone())
    }
}

// Env functions
impl FloraSeed {
    pub fn get_env(&self) -> BTreeMap<String, String>
    {
        self.env.clone().unwrap_or_default()
    }
    pub fn update_env(&mut self, env_name: &str, env_value: &str)
    {
        // Edit environment
        let seed_env = self.env.get_or_insert(BTreeMap::new());
        seed_env.insert(String::from(env_name), String::from(env_value));
    }

    pub fn delete_env(&mut self, env_name: &str)
    {
        // Edit environment
        let seed_env = self.env.get_or_insert(BTreeMap::new());
        seed_env.remove(env_name);
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FloraSeedSettings {
    pub launcher_command: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FloraSeedApp {
    pub application_name: String,
    pub application_location: String,
    pub category: Option<String>,
}
