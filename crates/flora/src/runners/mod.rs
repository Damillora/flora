use crate::{
    app::{FloraApp, FloraAppType},
    config::FloraConfig,
    dirs::FloraDirs,
};

/// Wine runner
pub mod wine;
/// Proton runner
pub mod proton;


pub fn run_app_config(
    name: &String,
    dirs: &FloraDirs,
    config: &FloraConfig,
    app: &FloraApp,
    args: &Option<Vec<String>>,
    quiet: bool,
    wait: bool,
) -> Result<(), crate::errors::FloraError> {
    match app.app_type {
        FloraAppType::Wine(_) => wine::run_wine_config(name, dirs, config, app, args, quiet, wait),
        FloraAppType::Proton(_) => proton::run_proton_config(name, dirs, config, app, args, quiet, wait),
        FloraAppType::Other => Ok(()),
    }
}

pub fn run_app_tricks(
    name: &String,
    dirs: &FloraDirs,
    config: &FloraConfig,
    app: &FloraApp,
    args: &Option<Vec<String>>,
    quiet: bool,
    wait: bool,
) -> Result<(), crate::errors::FloraError> {
    match app.app_type {
        FloraAppType::Wine(_) => wine::run_wine_tricks(name, dirs, config, app, args, quiet, wait),
        FloraAppType::Proton(_) => proton::run_proton_tricks(name, dirs, config, app, args, quiet, wait),
        FloraAppType::Other => Ok(()),
    }
}

pub fn run_app_executable(
    name: &String,
    dirs: &FloraDirs,
    config: &FloraConfig,
    app: &FloraApp,
    args: &Vec<String>,
    quiet: bool,
    wait: bool,
) -> Result<(), crate::errors::FloraError> {
    match app.app_type {
        FloraAppType::Wine(_) => {
            wine::run_wine_executable(name, dirs, config, app, args, quiet, wait)
        },
        FloraAppType::Proton(_) => {
            proton::run_proton_executable(name, dirs, config, app, args, quiet, wait)
        }
        FloraAppType::Other => Ok(()),
    }
}


pub fn create_desktop_entry(
    name: &String,
    dirs: &FloraDirs,
    app: &FloraApp,
) -> Result<(), crate::errors::FloraError> {
    match app.app_type {
        FloraAppType::Wine(_) => {
            wine::create_desktop_entry(name, dirs, app)
        },
        FloraAppType::Proton(_) => {
            proton::create_desktop_entry(name, dirs, app)
        },
        FloraAppType::Other => Ok(()),
    }
}
