use std::{
    fs::{self, read_dir},
    path::PathBuf,
};

use directories::ProjectDirs;
use log::debug;

use crate::{
    config::FloraConfig, desktop, dirs::FloraDirs, errors::FloraError, runners, seed::{FloraSeed, FloraSeedApp, FloraSeedType}, start_menu::FloraSeedStartMenuItem,
};

/// Manages Flora seeds configurations
pub struct FloraManager {
    flora_dirs: Box<FloraDirs>,
    config: Box<FloraConfig>,
}

// Instance functions
impl FloraManager {
    fn seed_path(&self, name: &str) -> PathBuf {
        let mut new_seed_location = self.flora_dirs.get_seed_root();
        new_seed_location.push(format!("{}.toml", name));

        new_seed_location
    }

    fn is_seed_exists(&self, name: &str) -> Result<bool, FloraError> {
        let new_seed_location = self.seed_path(name);

        let result = fs::exists(new_seed_location).map_err(FloraError::from)?;

        Ok(result)
    }

    fn read_seed(&self, name: &str) -> Result<FloraSeed, FloraError> {
        let seed_path = self.seed_path(name);
        let seed_toml = fs::read_to_string(&seed_path)?;
        let seed: FloraSeed = toml::from_str(seed_toml.as_str())?;

        Ok(seed)
    }

    /// Creates a new Flora seed
    pub fn create_seed(&self, name: &str, new_seed: &FloraSeed) -> Result<(), FloraError> {
        if self.is_seed_exists(name)? {
            return Err(FloraError::SeedExists(name.to_string()));
        }

        let new_seed_location = self.seed_path(name);

        debug!("Creating seed at {}", &new_seed_location.to_string_lossy());

        let new_toml = toml::to_string(new_seed).map_err(FloraError::from)?;

        // Write the content to the file
        fs::write(&new_seed_location, new_toml.as_bytes())?;

        Ok(())
    }

    /// Edit seed
    pub fn update_seed(&self, name: &str, seed: &FloraSeed) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound(name.to_string()));
        }

        let seed_location = self.seed_path(name);

        debug!("Updating seed at {}", &seed_location.to_string_lossy());

        let seed_toml = toml::to_string(seed)?;

        let seed_path = self.seed_path(name);
        fs::write(seed_path, seed_toml)?;

        Ok(())
    }

    pub fn list_start_menu_entries(
        &self,
        name: &str,
    ) -> Result<Vec<FloraSeedStartMenuItem>, FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound(name.to_string()));
        }

        let seed = self.read_seed(name)?;

        let runner = runners::create_runner(name, &self.flora_dirs, &self.config, &seed)?;
        runner.list_start_menu_entries()
    }

    /// Creates an app for seed from Start Menu item
    pub fn create_start_menu_app(&self, name: &str, menu_name: &str) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound(name.to_string()));
        }

        let seed = self.read_seed(name)?;
        let runner = runners::create_runner(name, &self.flora_dirs, &self.config, &seed)?;
        let start_menu_location = runner.get_start_menu_entry_location(menu_name)?;

        let mut upd_seed = seed.clone();
        upd_seed.add_app(FloraSeedApp{
            application_name: menu_name.to_string(),
            application_location: start_menu_location,
            category: None,
        })?;

        self.update_seed(name, &upd_seed)
    }

    /// Deletes new Flora seed
    pub fn delete_seed(&self, name: &str) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound(name.to_string()));
        }

        let seed_location = self.seed_path(name);

        debug!("Deleting seed at {}", &seed_location.to_string_lossy());

        fs::remove_file(&seed_location)?;

        Ok(())
    }

    pub fn list_seed(&self) -> Result<Vec<FloraSeedListItem>, FloraError> {
        let seed_dir = self.flora_dirs.get_seed_root();

        let files = read_dir(&seed_dir)?;

        files
            .map(|file_path| -> Result<PathBuf, FloraError> { Ok(file_path?.path()) })
            .map(|seed_config_path| -> Result<FloraSeedListItem, FloraError> {
                let file_path = seed_config_path?;
                let file_stem = file_path.file_stem().unwrap_or_default();
                let name = String::from(file_stem.to_string_lossy());

                let config = self.read_seed(&name)?;

                Ok(FloraSeedListItem {
                    seed_name: name,
                    seed_type: match config.seed_type {
                        FloraSeedType::Wine(_) => "wine".to_string(),
                        FloraSeedType::Proton(_) => "proton".to_string(),
                        FloraSeedType::None => "none".to_string(),
                    },
                })
            })
            .collect()
    }

    /// Deletes new Flora seed
    pub fn get_seed(&self, name: &str) -> Result<FloraSeed, FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound(name.to_string()));
        }

        let seed_config = self.read_seed(name)?;

        Ok(seed_config)
    }

    /// Launches the prefix configuration dialog of an seed (usually winecfg)
    pub fn seed_config(
        &self,
        name: &str,
        args: &Option<Vec<&str>>,
        quiet: bool,
        wait: bool,
    ) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound(name.to_string()));
        }

        let seed = self.read_seed(name)?;

        let runner = runners::create_runner(name, &self.flora_dirs, &self.config, &seed)?;
        runner.run_config(args, quiet, wait)
    }

    /// Launches wine(proton)tricks inside an seed's prefix
    pub fn seed_tricks(
        &self,
        name: &str,
        args: &Option<Vec<&str>>,
        quiet: bool,
        wait: bool,
    ) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound(name.to_string()));
        }

        let seed = self.read_seed(name)?;

        let runner = runners::create_runner(name, &self.flora_dirs, &self.config, &seed)?;
        runner.run_tricks(args, quiet, wait)
    }

    /// Launches an app entry inside an seed's prefix
    pub fn seed_run_app(
        &self,
        name: &str,
        app_name: &Option<&str>,
        quiet: bool,
        wait: bool,
    ) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound(name.to_string()));
        }
        let seed = self.read_seed(name)?;

        let app_entry = seed.get_app_or_default(app_name)?;

        // Determine arguments to be passed to runner
        let new_args = [app_entry.application_location.as_str()];

        let runner = runners::create_runner(name, &self.flora_dirs, &self.config, &seed)?;
        runner.run_executable(&new_args, quiet, wait)
    }

    /// Launches an executable inside an seed's prefix
    pub fn seed_run_executable(
        &self,
        name: &str,
        args: &Vec<&str>,
        quiet: bool,
        wait: bool,
    ) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound(name.to_string()));
        }

        let seed = self.read_seed(name)?;

        // Determine arguments to be passed to runner
        let new_args = args;

        let runner = runners::create_runner(name, &self.flora_dirs, &self.config, &seed)?;
        runner.run_executable(new_args, quiet, wait)
    }

    /// Creates a desktop entry for seed
    pub fn create_desktop_entries(
        &self,
        seed_name: Option<&str>,
        app_name: Option<&str>,
    ) -> Result<(), FloraError> {
        // Initialize menus
        desktop::initialize_desktop_entries(&self.flora_dirs)?;

        let seed_dir = self.flora_dirs.get_seed_root();

        let mut files = read_dir(&seed_dir)?
            .map(|seed_config_path| {
                let path = seed_config_path?
                    .path();
                let file_stem = path.file_stem().unwrap_or_default();
                let name = file_stem.to_string_lossy();
                let seed = self.read_seed(&name)?;

                Ok((String::from(name), seed))
            })
            .collect::<Result<Vec<_>, FloraError>>()?;

        if let Some(seed_name) = seed_name {
            files.retain(|(name, _)| name == seed_name);
        }

        for (name, seed) in files {
            debug!("Generating menu entries for seed {}", name);

            let runner = runners::create_runner(&name, &self.flora_dirs, &self.config, &seed)?;
            let mut apps: Vec<_> = seed.get_apps();
            if let Some(app_name) = app_name {
                apps.retain(|app| app.application_name == app_name);
            }

            for app in apps {
                runner.create_desktop_entry(&app)?;
            }
        }

        Ok(())
    }
}

// Static functions
impl FloraManager {
    /// Creates a new FloraManager instance
    pub fn new() -> Result<Self, FloraError> {
        let proj_dirs =
            ProjectDirs::from("com", "Damillora", "Flora").ok_or(FloraError::NoValidHome)?;
        let flora_root = proj_dirs.data_dir().to_path_buf();

        let dirs = FloraDirs::new(flora_root)?;
        dirs.create_dirs()?;

        // Read config
        let config = FloraConfig::read_config(&dirs)?;

        Ok(Self {
            flora_dirs: Box::new(dirs),
            config: Box::new(config),
        })
    }
}

// List models
pub struct FloraSeedListItem {
    pub seed_name: String,
    pub seed_type: String,
}
