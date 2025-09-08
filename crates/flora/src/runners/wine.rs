use std::{fs, path::PathBuf, process::Stdio};

use log::{debug, info};

use crate::{
    config::FloraConfig,
    desktop,
    dirs::FloraDirs,
    errors::FloraError,
    seed::{FloraSeed, FloraSeedType, FloraWineSeed},
};

fn get_wine_dir(dirs: &FloraDirs, config: &FloraConfig, wine_seed: &FloraWineSeed) -> PathBuf {
    if let Some(runner) = &wine_seed.wine_runtime {
        // Wine runtime is defined in seed.
        // Use Wine runtime defined in seed.
        let mut wine_path = dirs.get_wine_root();
        wine_path.push(runner);
        PathBuf::from(&wine_path)
    } else if let Some(wine_config) = &config.wine {
        // Wine runtime is not defined in seed, but defined globally.
        // Use Wine runtime defined in global configuration
        let mut wine_path = dirs.get_wine_root();
        wine_path.push(wine_config.default_wine_runtime.clone());
        PathBuf::from(&wine_path)
    } else {
        // Wine runtime is not defined in seed and globally.
        // Use system wine in /usr
        PathBuf::from("/usr")
    }
}

fn get_wine_prefix(dirs: &FloraDirs, config: &FloraConfig, wine_seed: &FloraWineSeed) -> PathBuf {
    if let Some(path) = &wine_seed.wine_prefix {
        // Prefix is defined in seed
        // Use prefix defined in seed.
        PathBuf::from(path.clone())
    } else if let Some(wine_config) = &config.wine {
        // Prefix is not defined in seed, but there is a default prefix defined globally.
        // Use default prefix from global configuration.
        PathBuf::from(&wine_config.default_wine_prefix)
    } else {
        // Prefix is not defined in seed and default prefix is not set.
        // Use a well-known fallback prefix directory.
        dirs.get_fallback_prefix()
    }
}

fn ensure_wine_dir(wine_dir: &PathBuf) -> Result<(), FloraError> {
    debug!(
        "Wine dir: {}",
        wine_dir
            .clone()
            .into_os_string()
            .into_string()
            .map_err(|_| FloraError::InternalError)?
    );

    if !fs::exists(wine_dir)? {
        return Err(FloraError::MissingRunner);
    }

    Ok(())
}

fn ensure_wine_prefix(wine_prefix: &PathBuf) -> Result<(), FloraError> {
    debug!(
        "Wine prefix: {}",
        wine_prefix
            .clone()
            .into_os_string()
            .into_string()
            .map_err(|_| FloraError::InternalError)?
    );

    if !fs::exists(wine_prefix)? {
        info!("Prefix not found, but will be created at launch");
    }

    Ok(())
}

/// Run something in wine
pub fn run_wine_executable(
    name: &str,
    dirs: &FloraDirs,
    config: &FloraConfig,
    seed: &FloraSeed,
    args: &Vec<String>,
    quiet: bool,
    wait: bool,
) -> Result<(), FloraError> {
    if let FloraSeedType::Wine(wine_seed) = &seed.seed_type {
        let wine_dir = get_wine_dir(dirs, config, wine_seed);
        let wine_prefix = get_wine_prefix(dirs, config, wine_seed);

        ensure_wine_dir(&wine_dir)?;
        ensure_wine_prefix(&wine_prefix)?;

        let mut wine_exe = wine_dir.clone();
        if !wine_dir.as_os_str().is_empty() {
            wine_exe.push("bin/wine");
        } else {
            // Use system wine
            wine_exe.push("/usr/bin/wine");
        }

        debug!(
            "Using {} to launch {}",
            wine_exe
                .clone()
                .into_os_string()
                .into_string()
                .map_err(|_| FloraError::InternalError)?,
            args.join(" ")
        );

        use std::process::Command;
        let mut command = Command::new(wine_exe);
        command.env("WINEPREFIX", wine_prefix).args(args);
        if quiet {
            let log_out = dirs.get_log_file(name)?;
            let log_err = dirs.get_log_file(name)?;
            command.stdin(Stdio::null()).stdout(log_out).stderr(log_err);
        }

        let mut handle = command.spawn()?;
        if wait {
            handle.wait()?;
        }

        Ok(())
    } else {
        Err(FloraError::IncorrectRunner)
    }
}

/// Run tricks
pub fn run_wine_tricks(
    name: &str,
    dirs: &FloraDirs,
    config: &FloraConfig,
    seed: &FloraSeed,
    args: &Option<Vec<String>>,
    quiet: bool,
    wait: bool,
) -> Result<(), FloraError> {
    if let FloraSeedType::Wine(wine_seed) = &seed.seed_type {
        let wine_dir = get_wine_dir(dirs, config, wine_seed);
        let wine_prefix = get_wine_prefix(dirs, config, wine_seed);

        ensure_wine_dir(&wine_dir)?;
        ensure_wine_prefix(&wine_prefix)?;

        let mut wine_exe = wine_dir.clone();
        if !wine_dir.as_os_str().is_empty() {
            wine_exe.push("bin/wine");
        } else {
            // Use system wine
            wine_exe.push("/usr/bin/wine");
        }

        debug!(
            "Using {} for winetricks",
            wine_exe
                .clone()
                .into_os_string()
                .into_string()
                .map_err(|_| FloraError::InternalError)?
        );

        use std::process::Command;
        let mut command = Command::new("winetricks");
        command
            .env("WINEPREFIX", wine_prefix)
            .env("WINE", wine_exe)
            .arg("-q");

        if quiet {
            let log_out = dirs.get_log_file(name)?;
            let log_err = dirs.get_log_file(name)?;
            command.stdin(Stdio::null()).stdout(log_out).stderr(log_err);
        }

        if let Some(args) = args {
            command.args(args);
        }
        let mut handle = command.spawn()?;
        if wait {
            handle.wait()?;
        }

        Ok(())
    } else {
        Err(FloraError::IncorrectRunner)
    }
}

pub fn run_wine_config(
    name: &str,
    dirs: &FloraDirs,
    config: &FloraConfig,
    seed: &FloraSeed,
    args: &Option<Vec<String>>,
    quiet: bool,
    wait: bool,
) -> Result<(), FloraError> {
    let mut winecfg_path = vec![String::from("winecfg")];

    if let Some(additional_args) = args {
        winecfg_path.extend(additional_args.iter().cloned());
    }

    run_wine_executable(name, dirs, config, seed, &winecfg_path, quiet, wait)
}

pub fn create_desktop_entry(
    name: &String,
    dirs: &FloraDirs,
    seed: &FloraSeed,
) -> Result<(), FloraError> {
    // Initialize menus
    desktop::initialize_desktop_entries(dirs)?;

    for app in seed.apps.iter() {
        // Create desktop entry files
        let desktop_entry = format!(
            "[Desktop Entry]
Type=Application
Categories=X-Flora
Name={}
Icon={}
Exec=flora run -a -w {} \"{}\"
Comment=Run {} with Flora
Terminal=false",
            app.application_name,
            "applications-other",
            name,
            app.application_name,
            app.application_name
        );

        let desktop_entry_location = dirs.get_desktop_entry_file(name, &app.application_name);

        debug!(
            "Writing {} desktop entry to {}",
            name,
            desktop_entry_location
                .clone()
                .into_os_string()
                .into_string()
                .map_err(|_| FloraError::InternalError)?
        );

        fs::write(desktop_entry_location, desktop_entry)?;
    }
    Ok(())
}
