use std::{
    collections::BTreeMap,
    fs::{self, read_dir},
    path::PathBuf,
};

use directories::ProjectDirs;
use log::debug;

use crate::{
    config::FloraConfig,
    desktop,
    dirs::FloraDirs,
    errors::FloraError,
    requests::{FloraCreateSeed, FloraCreateSeedApp, FloraSeedAppOperations, FloraUpdateSeed},
    responses::{FloraSeedItem, FloraSeedStartMenuItem},
    runners,
    seed::FloraSeed,
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

    fn write_seed_config(&self, name: &str, seed: &FloraSeed) -> Result<(), FloraError> {
        let seed_toml = toml::to_string(seed)?;

        let seed_path = self.seed_path(name);
        fs::write(seed_path, seed_toml)?;

        Ok(())
    }

    /// Creates a new Flora seed
    pub fn create_seed(&self, name: &str, seed_opts: &FloraCreateSeed) -> Result<(), FloraError> {
        if self.is_seed_exists(name)? {
            return Err(FloraError::SeedExists);
        }

        let new_seed_location = self.seed_path(name);

        debug!("Creating seed at {}", &new_seed_location.to_string_lossy());

        let new_seed = FloraSeed::from_options(&self.config, seed_opts)?;
        let new_toml = toml::to_string(&new_seed).map_err(FloraError::from)?;

        // Write the content to the file
        fs::write(&new_seed_location, new_toml.as_bytes())?;

        Ok(())
    }
    /// Edit seed
    pub fn update_seed(&self, name: &str, upd_data: &FloraUpdateSeed) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound);
        }

        let seed_location = self.seed_path(name);

        debug!("Updating seed at {}", &seed_location.to_string_lossy());

        let mut seed_config = self.read_seed(name)?;
        seed_config.merge_options(upd_data)?;

        self.write_seed_config(name, &seed_config)?;

        Ok(())
    }
    /// Edit seed env
    pub fn update_seed_env(
        &self,
        name: &str,
        env_name: &str,
        env_value: &str,
    ) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound);
        }

        let seed_location = self.seed_path(name);

        debug!(
            "Setting env variable {} at {}",
            &env_name,
            &seed_location.to_string_lossy()
        );
        let mut seed_config = self.read_seed(name)?;

        // Edit environment
        let mut seed_env = seed_config.env.unwrap_or(BTreeMap::new());
        seed_env.insert(String::from(env_name), String::from(env_value));

        // Move back edited env
        seed_config.env = Some(seed_env);
        self.write_seed_config(name, &seed_config)?;

        Ok(())
    }
    /// Edit seed env
    pub fn delete_seed_env(&self, name: &str, env_name: &str) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound);
        }

        let seed_location = self.seed_path(name);

        debug!(
            "Deleting env variable {} at {}",
            &env_name,
            &seed_location.to_string_lossy()
        );
        let mut seed_config = self.read_seed(name)?;

        // Edit environment
        let mut seed_env = seed_config.env.unwrap_or(BTreeMap::new());
        seed_env.remove(env_name);

        // Move back edited env
        seed_config.env = Some(seed_env);
        self.write_seed_config(name, &seed_config)?;

        Ok(())
    }
    /// Edit seed apps
    pub fn update_seed_apps(
        &self,
        name: &str,
        upd_data: &Vec<FloraSeedAppOperations>,
    ) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound);
        }

        let seed_location = self.seed_path(name);

        debug!("Updating seed at {}", &seed_location.to_string_lossy());

        let mut seed_config = self.read_seed(name)?;
        seed_config.update_apps(upd_data)?;

        self.write_seed_config(name, &seed_config)?;

        Ok(())
    }

    pub fn list_start_menu_entries(
        &self,
        name: &str,
    ) -> Result<Vec<FloraSeedStartMenuItem>, FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound);
        }

        let seed = self.read_seed(name)?;

        let runner = runners::create_runner(name, &self.flora_dirs, &self.config, &seed)?;
        runner.list_start_menu_entries()
    }

    /// Creates an app for seed from Start Menu item
    pub fn create_start_menu_app(&self, name: &str, menu_name: &str) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound);
        }

        let seed = self.read_seed(name)?;

        let runner = runners::create_runner(name, &self.flora_dirs, &self.config, &seed)?;
        let start_menu_location = runner.get_start_menu_entry_location(menu_name)?;
        let update_seed_operation = vec![FloraSeedAppOperations::Add(FloraCreateSeedApp {
            application_name: menu_name,
            application_location: start_menu_location.as_str(),
            category: None,
        })];

        self.update_seed_apps(name, &update_seed_operation)
    }

    /// Deletes new Flora seed
    pub fn delete_seed(&self, name: &str) -> Result<(), FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound);
        }

        let seed_location = self.seed_path(name);

        debug!("Deleting seed at {}", &seed_location.to_string_lossy());

        fs::remove_file(&seed_location)?;

        Ok(())
    }

    pub fn list_seed(&self) -> Result<Vec<FloraSeedItem>, FloraError> {
        let seed_dir = self.flora_dirs.get_seed_root();

        let files = read_dir(&seed_dir)?;

        files
            .map(|file_path| -> Result<PathBuf, FloraError> { Ok(file_path?.path()) })
            .map(|seed_config_path| -> Result<FloraSeedItem, FloraError> {
                let file_path = seed_config_path?;
                let file_stem = file_path.file_stem().ok_or(FloraError::SeedNotFound)?;
                let name = String::from(file_stem.to_string_lossy());

                let config = self.read_seed(&name)?;

                Ok(FloraSeedItem::from_config(&name, &config))
            })
            .collect()
    }

    /// Deletes new Flora seed
    pub fn show_seed(&self, name: &str) -> Result<FloraSeedItem, FloraError> {
        if !self.is_seed_exists(name)? {
            return Err(FloraError::SeedNotFound);
        }

        let seed_config = self.read_seed(name)?;

        Ok(FloraSeedItem::from_config(name, &seed_config))
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
            return Err(FloraError::SeedNotFound);
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
            return Err(FloraError::SeedNotFound);
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
            return Err(FloraError::SeedNotFound);
        }
        let seed = self.read_seed(name)?;

        let app_entry = match &app_name {
            Some(app_name) => seed
                .apps
                .iter()
                .find(|item| &item.application_name == app_name),
            None => seed.apps.first(),
        };

        if let Some(app_entry) = app_entry {
            // Determine arguments to be passed to runner
            let new_args = [&*app_entry.application_location];

            let runner = runners::create_runner(name, &self.flora_dirs, &self.config, &seed)?;
            runner.run_executable(&new_args, quiet, wait)
        } else {
            Err(FloraError::SeedNoApp)
        }
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
            return Err(FloraError::SeedNotFound);
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
                let path = seed_config_path
                    .map_err(|_| FloraError::SeedNotFound)?
                    .path();
                let file_stem = path.file_stem().ok_or(FloraError::SeedNotFound)?;
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
            let mut apps: Vec<_> = seed.apps.iter().collect();
            if let Some(app_name) = app_name {
                apps.retain(|app| app.application_name == app_name);
            }

            for app in apps {
                runner.create_desktop_entry(app)?;
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
