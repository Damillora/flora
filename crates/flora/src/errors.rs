use std::fmt;

/// Every error that Flora can throw
pub enum FloraError {
    /// Internal Flora error
    InternalError,

    /// App already exists
    AppExists,

    /// App is not found
    AppNotFound,

    /// Incorrect runner is invoked
    IncorrectRunner,

    /// Runner is not found
    MissingRunner,
    /// Config for runner is not found
    MissingRunnerConfig,

    /// Unable to launch app
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
            Self::AppExists => write!(f, "App already exists"),
            Self::AppNotFound => write!(f, "App does not exist"),
            Self::IncorrectRunner => write!(f, "Incorrect runner has been invoked"),
            Self::MissingRunner => write!(f, "Cannot find runner"),
            Self::MissingRunnerConfig => write!(f, "Cannot find config for runner"),
            Self::LaunchError => write!(f, "Unable to launch app"),
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
