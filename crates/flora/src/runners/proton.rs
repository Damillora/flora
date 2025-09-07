use std::{fs, path::PathBuf, process::Stdio};

use log::{debug, info};

use crate::{
    app::{FloraApp, FloraAppProtonConfig, FloraAppType},
    config::FloraConfig,
    desktop,
    dirs::FloraDirs,
    errors::FloraError,
};

fn find_proton_tool(dirs: &FloraDirs, name: &String) -> Result<PathBuf, FloraError> {
    // Flora Proton path
    let mut flora_proton_path = dirs.get_proton_root();
    flora_proton_path.push(name);

    // Local Steam Proton path
    let mut steam_proton_path = dirs.get_proton_root_steam();
    steam_proton_path.push(name);

    // System Steam Proton Path
    let mut steam_proton_path_system = PathBuf::from("/usr/share/steam/compatibilitytools.d");
    steam_proton_path_system.push(name);

    if fs::exists(&flora_proton_path)? {
        return Ok(flora_proton_path);
    } else if fs::exists(&steam_proton_path)? {
        return Ok(steam_proton_path);
    } else if fs::exists(&steam_proton_path_system)? {
        return Ok(steam_proton_path_system);
    } else {
        return Ok(PathBuf::from(&name));
    }
}
fn get_proton_tool(
    dirs: &FloraDirs,
    config: &FloraConfig,
    proton_app_config: &FloraAppProtonConfig,
) -> Result<PathBuf, FloraError> {
    let wine_dir = if let Some(runner) = &proton_app_config.proton_runtime {
        // Proton runtime is defined in app config
        Ok(find_proton_tool(&dirs, &runner)?)
    } else if let Some(proton_config) = &config.proton {
        // Proton runtime is not defined in app config, but defined in global config
        Ok(find_proton_tool(
            &dirs,
            &proton_config.default_proton_runtime,
        )?)
    } else {
        // Proton runtime is not defined in app config and global config, keep empty.
        Ok(PathBuf::from(""))
    };

    wine_dir
}

fn get_proton_prefix(
    dirs: &FloraDirs,
    config: &FloraConfig,
    proton_app_config: &FloraAppProtonConfig,
) -> PathBuf {
    let wine_prefix = if let Some(path) = &proton_app_config.proton_prefix {
        // Prefix is defined in app config
        PathBuf::from(path.clone())
    } else if let Some(proton_config) = &config.proton {
        // Prefix is not defined in app config, but there is a default prefix in global config
        PathBuf::from(&proton_config.default_proton_prefix)
    } else {
        // Prefix is not defined in app config and global config
        PathBuf::from(dirs.get_fallback_prefix())
    };

    wine_prefix
}

fn ensure_proton_tool(proton_tool: &PathBuf) -> Result<(), FloraError> {
    debug!(
        "Proton tool dir: {}",
        proton_tool
            .clone()
            .into_os_string()
            .into_string()
            .map_err(|_| FloraError::InternalError)?
    );

    if !fs::exists(&proton_tool)? {
        return Err(FloraError::MissingRunner);
    }

    Ok(())
}

fn ensure_proton_prefix(proton_prefix: &PathBuf) -> Result<(), FloraError> {
    debug!(
        "Proton prefix: {}",
        proton_prefix
            .clone()
            .into_os_string()
            .into_string()
            .map_err(|_| FloraError::InternalError)?
    );

    if !fs::exists(&proton_prefix)? {
        info!("Prefix not found, but will be created at launch");
    }

    Ok(())
}

/// Run something in wine
pub fn run_proton_executable(
    name: &String,
    dirs: &FloraDirs,
    config: &FloraConfig,
    app: &FloraApp,
    args: &Vec<String>,
    quiet: bool,
    wait: bool,
) -> Result<(), FloraError> {
    if let FloraAppType::Proton(proton_config) = &app.app_type {
        let proton_tool = get_proton_tool(&dirs, &config, &proton_config)?;
        let proton_prefix = get_proton_prefix(&dirs, &config, &proton_config);

        ensure_proton_tool(&proton_tool)?;
        ensure_proton_prefix(&proton_prefix)?;

        debug!("Using {} to launch {}", "umu-run", args.join(" "));

        use std::process::Command;
        let mut command = Command::new("umu-run");
        command
            .env("WINEPREFIX", proton_prefix)
            .env("PROTONPATH", proton_tool)
            .args(args);

        if let Some(game_id) = &proton_config.game_id {
            command.env("GAMEID", &game_id);
        }
        if let Some(store) = &proton_config.store {
            command.env("STORE", &store);
        }
        if quiet {
            let log_out = dirs.get_log_file(&name)?;
            let log_err = dirs.get_log_file(&name)?;
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
pub fn run_proton_tricks(
    name: &String,
    dirs: &FloraDirs,
    config: &FloraConfig,
    app: &FloraApp,
    args: &Option<Vec<String>>,
    quiet: bool,
    wait: bool,
) -> Result<(), FloraError> {
    let mut winetricks_path = vec![String::from("winetricks")];

    if let Some(additional_args) = args {
        winetricks_path.extend(additional_args.iter().cloned());
    }

    run_proton_executable(name, dirs, config, app, &winetricks_path, quiet, wait)
}

pub fn run_proton_config(
    name: &String,
    dirs: &FloraDirs,
    config: &FloraConfig,
    app: &FloraApp,
    args: &Option<Vec<String>>,
    quiet: bool,
    wait: bool,
) -> Result<(), FloraError> {
    let mut winecfg_path = vec![String::from("winecfg")];

    if let Some(additional_args) = args {
        winecfg_path.extend(additional_args.iter().cloned());
    }

    run_proton_executable(name, dirs, config, app, &winecfg_path, quiet, wait)
}

pub fn create_desktop_entry(
    name: &String,
    dirs: &FloraDirs,
    app: &FloraApp,
) -> Result<(), FloraError> {
    // Initialize menus
    desktop::initialize_desktop_entries(&dirs)?;

    // Create desktop entry files
    let desktop_entry = format!(
        "[Desktop Entry]
Type=Application
Name={}
Icon={}
Exec=flora run -w {}
Comment=Run {} with Flora using Proton
Terminal=false",
        app.pretty_name, "applications-other", name, app.pretty_name
    );

    let desktop_entry_location = dirs.get_desktop_entry_file(&name);

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

    Ok(())
}
