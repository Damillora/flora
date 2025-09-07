use std::{
    fs::{self, File},
    path::PathBuf,
};

use crate::errors::FloraError;

pub struct FloraDirs {
    pub flora_root: PathBuf,
    pub applications_entry_dir: PathBuf,
}

impl FloraDirs {
    pub fn get_app_root(&self) -> PathBuf {
        let mut app_root = self.flora_root.clone();
        app_root.push("apps");

        app_root
    }
    pub fn get_wine_root(&self) -> PathBuf {
        let mut wine_root = self.flora_root.clone();
        wine_root.push("wine");

        wine_root
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
    pub fn get_log_file(&self, name: &String) -> Result<File, FloraError> {
        let mut log_file = self.get_log_root();
        log_file.push(format!("{}.log", name.as_str()));
        log::debug!(
            "Logging outputs to {}",
            log_file
                .clone()
                .into_os_string()
                .into_string()
                .map_err(|_| FloraError::InternalError)?
        );

        let log_file = File::options().append(true).create(true).open(log_file)?;

        Ok(log_file)
    }

    pub fn create_dirs(&self) {
        fs::create_dir_all(&self.flora_root).unwrap();
        fs::create_dir_all(&self.applications_entry_dir).unwrap();
        fs::create_dir_all(&self.get_app_root()).unwrap();
        fs::create_dir_all(&self.get_wine_root()).unwrap();
        fs::create_dir_all(&self.get_log_root()).unwrap();
        fs::create_dir_all(&self.get_prefixes_root()).unwrap();
        fs::create_dir_all(&self.get_icons_root()).unwrap();
    }
}

impl FloraDirs {
    pub fn new(flora_root: PathBuf, applications_entry_dir: PathBuf) -> Self {
        Self {
            flora_root: flora_root,
            applications_entry_dir: applications_entry_dir,
        }
    }
}
