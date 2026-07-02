use std::path::PathBuf;

use flora_icon::FloraLinkError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FloraError {
    // Start Menu related errors
    #[error("Unable to find start menu location for {0}")]
    StartMenuLocationNotFound(String),
    #[error("Unable to write desktop entry {0} to {1}")]
    DesktopEntryWriteError(PathBuf, std::io::Error),
    #[error("Unable to extract icon: {0}")]
    IconExtractionError(#[from] FloraLinkError),
    #[error("Unable to execute in runner: {0}")]
    RunnerExecError(std::io::Error),
    #[error("Unable to parse launcher command: {0}")]
    IncorrectLauncherCommand(String),

    #[error("Unable to access file: {0}")]
    FileAccessError(#[from] std::io::Error),
    #[error("User does not have a valid home directory")]
    NoValidHome,

    #[error("This runner cannot run applications")]
    RunnerNone,
    #[error("Unable to find runner {0}")]
    MissingRunner(PathBuf),
    #[error("Seed runner-specific options not found")]
    MissingRunnerConfig,

    #[error("Seed not found: {0}")]
    SeedNotFound(String),
    #[error("Seed already exists: {0}")]
    SeedExists(String),
    #[error("Seed not of the correct type")]
    SeedWrongType(String),

    #[error("Application already exists: {0}")]
    AppExists(String),
    #[error("Application not found: {0}")]
    AppNotFound(String),

    #[error("Error parsing configuration: {0}")]
    ConfigError(#[from] toml::de::Error),
    #[error("Error saving configuration: {0}")]
    ConfigSaveError(#[from] toml::ser::Error),
}
