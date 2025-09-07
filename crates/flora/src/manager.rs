use std::{fs, path::PathBuf};

use directories::ProjectDirs;
use log::debug;

use crate::{
    app::{FloraApp, FloraAppOptions},
    config::FloraConfig,
    dirs::FloraDirs,
    errors::FloraError,
    runners,
};

/// Manages Flora app configurations
pub struct FloraManager {
    flora_dirs: Box<FloraDirs>,
    config: Box<FloraConfig>,
}

// Instance functions
impl FloraManager {
    fn app_path(&self, name: &String) -> PathBuf {
        let mut new_app_location = self.flora_dirs.get_app_root();
        new_app_location.push(format!("{}.toml", name));

        new_app_location
    }

    fn is_app_exists(&self, name: &String) -> Result<bool, FloraError> {
        let new_app_location = self.app_path(name);

        let result = fs::exists(new_app_location).map_err(|_| FloraError::InternalError)?;

        Ok(result)
    }

    fn read_app_config(&self, name: &String) -> Result<FloraApp, FloraError> {
        let app_config_path = self.app_path(name);
        let config_toml = fs::read_to_string(&app_config_path)?;
        let config: FloraApp = toml::from_str(config_toml.as_str())?;

        Ok(config)
    }

    /// Creates a new Flora app
    pub fn create_app(&self, name: &String, app: &FloraAppOptions) -> Result<(), FloraError> {
        if self.is_app_exists(name)? {
            return Err(FloraError::AppExists);
        }

        let new_app_location = self.app_path(name);

        debug!(
            "Creating app at {}",
            &new_app_location
                .clone()
                .into_os_string()
                .into_string()
                .map_err(|_| FloraError::InternalError)?
        );

        let new_app = FloraApp::from_options(&self.config, &name, app)?;
        let new_toml = toml::to_string(&new_app).map_err(|_| FloraError::InternalError)?;

        // Write the content to the file
        fs::write(&new_app_location, new_toml.as_bytes())?;

        Ok(())
    }

    /// Deletes new Flora app
    pub fn delete_app(&self, name: &String) -> Result<(), FloraError> {
        if !self.is_app_exists(name)? {
            return Err(FloraError::AppNotFound);
        }

        let new_app_location = self.app_path(name);

        debug!(
            "Deleting app at {}",
            &new_app_location
                .clone()
                .into_os_string()
                .into_string()
                .map_err(|_| FloraError::InternalError)?
        );

        fs::remove_file(&new_app_location)?;

        Ok(())
    }

    /// Launches the prefix configuration dialog of an app (usually winecfg)
    pub fn app_config(
        &self,
        name: &String,
        args: &Option<Vec<String>>,
        quiet: bool,
        wait: bool,
    ) -> Result<(), FloraError> {
        if !self.is_app_exists(name)? {
            return Err(FloraError::AppNotFound);
        }

        let app_config = self.read_app_config(&name)?;

        runners::run_app_config(
            &name,
            &self.flora_dirs,
            &self.config,
            &app_config,
            args,
            quiet,
            wait,
        )
    }

    /// Launches wine(proton)tricks inside an app's prefix
    pub fn app_tricks(
        &self,
        name: &String,
        args: &Option<Vec<String>>,
        quiet: bool,
        wait: bool,
    ) -> Result<(), FloraError> {
        if !self.is_app_exists(name)? {
            return Err(FloraError::AppNotFound);
        }

        let app_config = self.read_app_config(&name)?;

        runners::run_app_tricks(
            &name,
            &self.flora_dirs,
            &self.config,
            &app_config,
            args,
            quiet,
            wait,
        )
    }

    /// Launches an executable inside an app's prefix
    pub fn app_run(
        &self,
        name: &String,
        args: &Option<Vec<String>>,
        quiet: bool,
        wait: bool,
    ) -> Result<(), FloraError> {
        if !self.is_app_exists(name)? {
            return Err(FloraError::AppNotFound);
        }

        let app_config = self.read_app_config(&name)?;

        let new_args = match args {
            Some(args) => args,
            None => &vec![app_config.executable_location.clone()],
        };
        runners::run_app_executable(
            &name,
            &self.flora_dirs,
            &self.config,
            &app_config,
            new_args,
            quiet,
            wait,
        )
    }

    /// Creates a desktop entry for application
    pub fn create_desktop_entry(&self, name: &String) -> Result<(), FloraError> {
        if !self.is_app_exists(name)? {
            return Err(FloraError::AppNotFound);
        }

        let app_config = self.read_app_config(&name)?;

        runners::create_desktop_entry(&name, &self.flora_dirs, &app_config)
    }
}

// Static functions
impl FloraManager {
    /// Creates a new FloraManager instance
    pub fn new() -> Self {
        let proj_dirs = ProjectDirs::from("com", "Damillora", "Flora").unwrap();
        let flora_root = proj_dirs.data_dir().to_path_buf();

        let dirs = FloraDirs::new(flora_root);
        dirs.create_dirs();

        // Read config
        let config = FloraConfig::read_config(&dirs).unwrap();

        Self {
            flora_dirs: Box::new(dirs),
            config: Box::new(config),
        }
    }
}
