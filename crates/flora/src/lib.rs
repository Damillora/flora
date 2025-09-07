/// Flora directories
mod dirs;

/// Flora configuration
mod config;

/// Desktop entry management
mod desktop;

/// Flora errors
pub mod errors;

/// Flora core manager
///
/// Handles creation and deletion of apps
pub mod manager;

/// Flora apps configuration
///
/// Contains functions related to app configuration
pub mod app;

/// Flora response items
///
/// Contain response structs
pub mod responses;

/// Flora runners
/// Contains functionality to launch apps
mod runners;
