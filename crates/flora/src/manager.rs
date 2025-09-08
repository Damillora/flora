use std::{
    fs::{self, read_dir},
    path::PathBuf,
};

use directories::ProjectDirs;
use log::debug;

use crate::{
    config::FloraConfig,
    dirs::FloraDirs,
    errors::FloraError,
    responses::FloraSeedItem,
    runners,
    seed::{FloraCreateSeed, FloraSeed},
};

/// Manages Flora seeds configurations
pub struct FloraManager {
    flora_dirs: Box<FloraDirs>,
    config: Box<FloraConfig>,
}

// Instance functions
impl FloraManager {
    fn seed_path(&self, name: &String) -> PathBuf {
        let mut new_seed_location = self.flora_dirs.get_seed_root();
        new_seed_location.push(format!("{}.toml", name));

        new_seed_location
    }

    fn is_seed_exists(&self, name: &String) -> Result<bool, FloraError> {
        let new_seed_location = self.seed_path(name);

        let result = fs::exists(new_seed_location).map_err(|_| FloraError::InternalError)?;

        Ok(result)
    }

    fn read_seed_config(&self, name: &String) -> Result<FloraSeed, FloraError> {
        let seed_config_path = self.seed_path(name);
        let config_toml = fs::read_to_string(&seed_config_path)?;
        let config: FloraSeed = toml::from_str(config_toml.as_str())?;

        Ok(config)
    }

    /// Creates a new Flora seed
    pub fn create_seed(
        &self,
        name: &String,
        seed_opts: &FloraCreateSeed,
    ) -> Result<(), FloraError> {
        if self.is_seed_exists(name)? {
            return Err(FloraError::SeedExists);
        }

        let new_seed_location = self.seed_path(name);

        debug!(
            "Creating seed at {}",
            &new_seed_location
                .clone()
                .into_os_string()
                .into_string()
                .map_err(|_| FloraError::InternalError)?
        );

        let new_seed = FloraSeed::from_options(&self.config, name, seed_opts)?;
        let new_toml = toml::to_string(&new_seed).map_err(|_| FloraError::InternalError)?;

        // Write the content to the file
        fs::write(&new_seed_location, new_toml.as_bytes())?;

        Ok(())
    }

    /// Deletes new Flora seed
    pub fn delete_seed(&self, name: &String) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound);
        }

        let new_seed_location = self.seed_path(name);

        debug!(
            "Deleting seed at {}",
            &new_seed_location
                .clone()
                .into_os_string()
                .into_string()
                .map_err(|_| FloraError::InternalError)?
        );

        fs::remove_file(&new_seed_location)?;

        Ok(())
    }

    pub fn list_seed(&self) -> Result<Vec<FloraSeedItem>, FloraError> {
        let seed_dir = self.flora_dirs.get_seed_root();

        let files = read_dir(&seed_dir)?;
        let list_items = files
            .map(|seed_config_path| -> FloraSeedItem {
                let path = seed_config_path.unwrap().path();
                let file_stem = path.file_stem().unwrap_or_default();
                let name = file_stem.to_os_string().into_string().unwrap();

                let config = self.read_seed_config(&name).unwrap();

                FloraSeedItem::from_config(&name, &config)
            })
            .collect();

        Ok(list_items)
    }

    /// Deletes new Flora seed
    pub fn show_seed(&self, name: &String) -> Result<FloraSeedItem, FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound);
        }

        let seed_config = self.read_seed_config(name)?;

        Ok(FloraSeedItem::from_config(name, &seed_config))
    }

    /// Launches the prefix configuration dialog of an seed (usually winecfg)
    pub fn seed_config(
        &self,
        name: &String,
        args: &Option<Vec<String>>,
        quiet: bool,
        wait: bool,
    ) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound);
        }

        let seed_config = self.read_seed_config(name)?;

        runners::run_seed_config(
            name,
            &self.flora_dirs,
            &self.config,
            &seed_config,
            args,
            quiet,
            wait,
        )
    }

    /// Launches wine(proton)tricks inside an seed's prefix
    pub fn seed_tricks(
        &self,
        name: &String,
        args: &Option<Vec<String>>,
        quiet: bool,
        wait: bool,
    ) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound);
        }

        let seed_config = self.read_seed_config(name)?;

        runners::run_seed_tricks(
            name,
            &self.flora_dirs,
            &self.config,
            &seed_config,
            args,
            quiet,
            wait,
        )
    }

    /// Launches an app entry inside an seed's prefix
    pub fn seed_run_app(
        &self,
        name: &String,
        app_name: &Option<String>,
        quiet: bool,
        wait: bool,
    ) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound);
        }
        let seed_config = self.read_seed_config(name)?;

        let app_entry = match &app_name {
            Some(app_name) => seed_config
                .apps
                .iter()
                .find(|item| &item.application_name == app_name),
            None => seed_config.apps.first(),
        };

        if let Some(app_entry) = app_entry {
            // Determine arguments to be passed to runner
            let new_args = &vec![app_entry.application_location.clone()];

            runners::run_seed_executable(
                name,
                &self.flora_dirs,
                &self.config,
                &seed_config,
                new_args,
                quiet,
                wait,
            )
        } else {
            Err(FloraError::SeedNoApp)
        }
    }

    /// Launches an executable inside an seed's prefix
    pub fn seed_run_executable(
        &self,
        name: &String,
        args: &Vec<String>,
        quiet: bool,
        wait: bool,
    ) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound);
        }

        let seed_config = self.read_seed_config(name)?;

        // Determine arguments to be passed to runner
        let new_args = args;

        runners::run_seed_executable(
            name,
            &self.flora_dirs,
            &self.config,
            &seed_config,
            new_args,
            quiet,
            wait,
        )
    }

    /// Creates a desktop entry for seed
    pub fn create_desktop_entry(&self, name: &String) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound);
        }

        let seed_config = self.read_seed_config(name)?;

        runners::create_desktop_entry(name, &self.flora_dirs, &seed_config)
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

impl Default for FloraManager {
    fn default() -> Self {
        Self::new()
    }
}
