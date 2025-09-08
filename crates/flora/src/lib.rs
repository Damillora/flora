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
/// Handles creation and deletion of seeds
pub mod manager;

/// Flora seed configuration
///
/// Contains functions related to Flora seeds. Flora seeds are a single Wine or Proton configuration, with its own WINEPREFIX.
pub mod seed;

/// Flora response items
///
/// Contain response structs
pub mod responses;

/// Flora runners
/// Contains functionality to launch seeds
mod runners;
