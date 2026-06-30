use std::fmt;

use flora_icon::FloraLinkError;

/// Every error that Flora can throw
pub enum FloraError {
    /// Unable to find home directory to initialize configuration
    NoValidHome,
    /// Seed already exists
    SeedExists,
    /// Seed is not found
    SeedNotFound,

    /// Seed has no default app
    SeedNoDefaultApp,
    /// Seed has no specified app
    SeedNoApp,
    /// Seed app already exists
    SeedAppExists,

    /// Incorrect update type on seed
    SeedUpdateMismatch,

    /// Start Menu entry not found
    StartMenuNotFound,

    /// Incorrect runner is invoked
    IncorrectRunner,
    /// Runner is not found
    MissingRunner,
    /// Config for runner is not found
    MissingRunnerConfig,

    /// Cannot parse launcher command
    IncorrectLauncher,

    /// Something wrong when processing icons
    IconError(FloraLinkError),

    /// Error parsing config
    ConfigError(toml::de::Error),
    /// Error saving config
    ConfigSaveError(toml::ser::Error),
    /// Error parsing config
    IoError(std::io::Error),
}

impl fmt::Debug for FloraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Initializations errors
            Self::NoValidHome => write!(
                f,
                "Cannot read configuration because of invalid home directory"
            ),
            // See derrors
            Self::SeedExists => write!(f, "Seed already exists"),
            Self::SeedNotFound => write!(f, "Seed does not exist"),
            Self::SeedNoDefaultApp => write!(f, "Seed does not have a default app"),
            Self::SeedNoApp => write!(f, "Seed does not have this app"),
            Self::SeedAppExists => write!(f, "App already exists"),
            Self::SeedUpdateMismatch => write!(f, "Attempting to update the wrong seed type"),
            // Start menu errors
            Self::StartMenuNotFound => write!(f, "Cannot find Start Menu entry"),
            // Runner errors
            Self::IncorrectRunner => write!(f, "Incorrect runner has been invoked"),
            Self::MissingRunner => write!(f, "Cannot find runner"),
            Self::MissingRunnerConfig => write!(f, "Cannot find config for runner"),
            // Launcher errors
            Self::IncorrectLauncher => write!(f, "Cannot parse launcher command"),
            // Misc errors
            Self::IconError(err) => write!(f, "There was a problem processing icons: {}", err),
            Self::ConfigError(err) => write!(f, "Config read error: {}", err),
            Self::ConfigSaveError(err) => write!(f, "Config save error: {}", err),
            Self::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl From<toml::de::Error> for FloraError {
    fn from(value: toml::de::Error) -> Self {
        Self::ConfigError(value)
    }
}

impl From<toml::ser::Error> for FloraError {
    fn from(value: toml::ser::Error) -> Self {
        Self::ConfigSaveError(value)
    }
}

impl From<std::io::Error> for FloraError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
impl From<FloraLinkError> for FloraError {
    fn from(value: FloraLinkError) -> Self {
        Self::IconError(value)
    }
}
