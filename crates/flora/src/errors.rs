use std::fmt;

/// Every error that Flora can throw
pub enum FloraError {
    /// Internal Flora error
    InternalError,

    /// Seed already exists
    SeedExists,

    /// Seed is not found
    SeedNotFound,

    /// Seed has no default app
    SeedNoDefaultApp,
    /// Seed has no specified app
    SeedNoApp,
    /// Incorrect runner is invoked
    IncorrectRunner,

    /// Runner is not found
    MissingRunner,
    /// Config for runner is not found
    MissingRunnerConfig,

    /// Unable to launch seed
    LaunchError,

    /// Error parsing config
    ConfigError(toml::de::Error),
    /// Error parsing config
    IoError(std::io::Error),
}

impl fmt::Debug for FloraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalError => write!(f, "An internal error has occured"),
            Self::SeedExists => write!(f, "Seed already exists"),
            Self::SeedNotFound => write!(f, "Seed does not exist"),
            Self::SeedNoDefaultApp => write!(f, "Seed does not have a default app"),
            Self::SeedNoApp => write!(f, "Seed does not have this app"),
            Self::IncorrectRunner => write!(f, "Incorrect runner has been invoked"),
            Self::MissingRunner => write!(f, "Cannot find runner"),
            Self::MissingRunnerConfig => write!(f, "Cannot find config for runner"),
            Self::LaunchError => write!(f, "Unable to launch seed"),
            Self::ConfigError(err) => write!(f, "Config read error: {}", err),
            Self::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl From<toml::de::Error> for FloraError {
    fn from(value: toml::de::Error) -> Self {
        Self::ConfigError(value)
    }
}

impl From<std::io::Error> for FloraError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
