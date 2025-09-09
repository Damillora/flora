use crate::{
    config::FloraConfig,
    dirs::FloraDirs,
    errors::FloraError,
    responses::FloraSeedStartMenuItem,
    runners::{proton::FloraProtonRunner, wine::FloraWineRunner},
    seed::{FloraSeed, FloraSeedType},
};

/// Proton runner
pub mod proton;
/// Wine runner
pub mod wine;

pub trait FloraRunner {
    fn run_config(
        &self,
        args: &Option<Vec<&str>>,
        quiet: bool,
        wait: bool,
    ) -> Result<(), FloraError>;
    fn run_tricks(
        &self,
        args: &Option<Vec<&str>>,
        quiet: bool,
        wait: bool,
    ) -> Result<(), FloraError>;
    fn run_executable(&self, args: &[&str], quiet: bool, wait: bool) -> Result<(), FloraError>;
    fn create_desktop_entries(&self) -> Result<(), FloraError>;
    fn get_start_menu_entry_location(&self, menu_name: &str) -> Result<String, FloraError>;
    fn list_start_menu_entries(&self) -> Result<Vec<FloraSeedStartMenuItem>, FloraError>;
}

pub fn create_runner<'a>(
    name: &'a str,
    dirs: &'a FloraDirs,
    config: &'a FloraConfig,
    seed: &'a FloraSeed,
) -> Box<dyn FloraRunner + 'a> {
    match &seed.seed_type {
        FloraSeedType::Wine(wine_seed) => {
            let runner = FloraWineRunner::new(name, dirs, config, wine_seed, &seed.apps);

            Box::new(runner)
        }
        FloraSeedType::Proton(proton_seed) => {
            let runner = FloraProtonRunner::new(name, dirs, config, proton_seed, &seed.apps);

            Box::new(runner)
        }
    }
}
