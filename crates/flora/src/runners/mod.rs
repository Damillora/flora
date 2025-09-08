use crate::{
    config::FloraConfig,
    dirs::FloraDirs,
    seed::{FloraSeed, FloraSeedType},
};

/// Proton runner
pub mod proton;
/// Wine runner
pub mod wine;

pub fn run_seed_config(
    name: &String,
    dirs: &FloraDirs,
    config: &FloraConfig,
    seed: &FloraSeed,
    args: &Option<Vec<String>>,
    quiet: bool,
    wait: bool,
) -> Result<(), crate::errors::FloraError> {
    match seed.seed_type {
        FloraSeedType::Wine(_) => {
            wine::run_wine_config(name, dirs, config, seed, args, quiet, wait)
        }
        FloraSeedType::Proton(_) => {
            proton::run_proton_config(name, dirs, config, seed, args, quiet, wait)
        }
    }
}

pub fn run_seed_tricks(
    name: &String,
    dirs: &FloraDirs,
    config: &FloraConfig,
    seed: &FloraSeed,
    args: &Option<Vec<String>>,
    quiet: bool,
    wait: bool,
) -> Result<(), crate::errors::FloraError> {
    match seed.seed_type {
        FloraSeedType::Wine(_) => {
            wine::run_wine_tricks(name, dirs, config, seed, args, quiet, wait)
        }
        FloraSeedType::Proton(_) => {
            proton::run_proton_tricks(name, dirs, config, seed, args, quiet, wait)
        }
    }
}

pub fn run_seed_executable(
    name: &String,
    dirs: &FloraDirs,
    config: &FloraConfig,
    seed: &FloraSeed,
    args: &Vec<String>,
    quiet: bool,
    wait: bool,
) -> Result<(), crate::errors::FloraError> {
    match seed.seed_type {
        FloraSeedType::Wine(_) => {
            wine::run_wine_executable(name, dirs, config, seed, args, quiet, wait)
        }
        FloraSeedType::Proton(_) => {
            proton::run_proton_executable(name, dirs, config, seed, args, quiet, wait)
        }
    }
}

pub fn create_desktop_entry(
    name: &String,
    dirs: &FloraDirs,
    seed: &FloraSeed,
) -> Result<(), crate::errors::FloraError> {
    match seed.seed_type {
        FloraSeedType::Wine(_) => wine::create_desktop_entry(name, dirs, seed),
        FloraSeedType::Proton(_) => proton::create_desktop_entry(name, dirs, seed),
    }
}
