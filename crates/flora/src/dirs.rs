use std::{
    fs::{self, File, create_dir},
    path::PathBuf,
};

use directories::BaseDirs;

use crate::errors::FloraError;

pub struct FloraDirs {
    pub flora_root: PathBuf,
    applications_entry_dir: PathBuf,
    config_menu_dir: PathBuf,
    applications_directory_dir: PathBuf,
    steam_compat_dir: PathBuf,
}

impl FloraDirs {
    pub fn get_seed_root(&self) -> PathBuf {
        let mut seed_root = self.flora_root.clone();
        seed_root.push("seeds");

        seed_root
    }
    pub fn get_wine_root(&self) -> PathBuf {
        let mut wine_root = self.flora_root.clone();
        wine_root.push("wine");

        wine_root
    }
    pub fn get_proton_root(&self) -> PathBuf {
        let mut wine_root = self.flora_root.clone();
        wine_root.push("proton");

        wine_root
    }
    pub fn get_proton_root_steam(&self) -> PathBuf {
        self.steam_compat_dir.clone()
    }
    pub fn get_log_root(&self) -> PathBuf {
        let mut log_root = self.flora_root.clone();
        log_root.push("logs");

        log_root
    }
    pub fn get_prefixes_root(&self) -> PathBuf {
        let mut wine_root = self.flora_root.clone();
        wine_root.push("prefixes");

        wine_root
    }
    pub fn get_icons_root(&self) -> PathBuf {
        let mut wine_root = self.flora_root.clone();
        wine_root.push("icons");

        wine_root
    }

    pub fn get_fallback_prefix(&self) -> PathBuf {
        let mut wine_root = self.flora_root.clone();
        wine_root.push("prefixes/default");

        wine_root
    }
    pub fn get_fallback_prefix_proton(&self) -> PathBuf {
        let mut wine_root = self.flora_root.clone();
        wine_root.push("prefixes/proton");

        wine_root
    }
    pub fn get_log_file(&self, name: &str) -> Result<File, FloraError> {
        let mut log_file = self.get_log_root();
        log_file.push(format!("{}.log", name));
        log::debug!("Logging outputs to {}", log_file.clone().to_string_lossy(),);

        let log_file = File::options().append(true).create(true).open(log_file)?;

        Ok(log_file)
    }
    pub fn get_desktop_entry_file(&self, name: &str, application_name: &str) -> PathBuf {
        let mut desktop_entry_location = self.applications_entry_dir.clone();
        desktop_entry_location.push(format!("{}_{}.desktop", name, application_name));

        desktop_entry_location
    }
    pub fn get_desktop_directory_file(&self) -> PathBuf {
        let mut desktop_entry_location = self.applications_directory_dir.clone();
        desktop_entry_location.push("flora.directory");

        desktop_entry_location
    }

    pub fn get_desktop_menu_file(&self) -> PathBuf {
        let mut desktop_entry_location = self.config_menu_dir.clone();
        desktop_entry_location.push("flora.menu");

        desktop_entry_location
    }

    pub fn get_icon_file(&self, name: &str, app_name: &str) -> PathBuf {
        let mut icon_path = self.get_icons_root();
        icon_path.push(format!("{}_{}.png", name, app_name));

        icon_path
    }

    pub fn create_dirs(&self) -> Result<(), FloraError> {
        fs::create_dir_all(&self.flora_root)?;
        fs::create_dir_all(&self.applications_entry_dir)?;
        fs::create_dir_all(self.get_seed_root())?;
        fs::create_dir_all(self.get_wine_root())?;
        fs::create_dir_all(self.get_proton_root())?;
        fs::create_dir_all(self.get_log_root())?;
        fs::create_dir_all(self.get_prefixes_root())?;
        fs::create_dir_all(self.get_icons_root())?;

        Ok(())
    }

    pub fn create_desktop_dirs(&self) -> Result<(), FloraError> {
        fs::create_dir_all(&self.applications_directory_dir)?;
        fs::create_dir_all(&self.applications_entry_dir)?;
        fs::create_dir_all(&self.config_menu_dir)?;

        Ok(())
    }
}

impl FloraDirs {
    pub fn new(flora_root: PathBuf) -> Result<Self, FloraError> {
        let base_dirs = BaseDirs::new().ok_or(FloraError::NoValidHome)?;

        let mut applications_entry_dir = base_dirs.data_dir().to_path_buf();
        applications_entry_dir.push("applications/flora");

        let mut applications_directory_dir = base_dirs.data_dir().to_path_buf();
        applications_directory_dir.push("desktop-directories");

        let mut config_menu_dir = base_dirs.config_dir().to_path_buf();
        config_menu_dir.push("menus/applications-merged");

        let mut steam_compat_dir = base_dirs.data_dir().to_path_buf();
        steam_compat_dir.push("Steam/compatibilitytools.d");

        Ok(Self {
            flora_root,
            applications_entry_dir,
            applications_directory_dir,
            config_menu_dir,
            steam_compat_dir,
        })
    }
}
