use std::{fs, path::PathBuf, process::Stdio};

use flora_icon::FloraLink;
use log::{debug, info};
use walkdir::WalkDir;

use crate::{
    config::FloraConfig,
    desktop,
    dirs::FloraDirs,
    errors::FloraError,
    seed::{FloraProtonSeed, FloraSeed, FloraSeedType},
    winepath,
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
        Ok(flora_proton_path)
    } else if fs::exists(&steam_proton_path)? {
        Ok(steam_proton_path)
    } else if fs::exists(&steam_proton_path_system)? {
        Ok(steam_proton_path_system)
    } else {
        Ok(PathBuf::from(&name))
    }
}
fn get_proton_tool(
    dirs: &FloraDirs,
    config: &FloraConfig,
    proton_seed: &FloraProtonSeed,
) -> Result<PathBuf, FloraError> {
    if let Some(runner) = &proton_seed.proton_runtime {
        // Proton runtime is defined in seed.
        // Use Proton runtime defined in seed.
        Ok(find_proton_tool(dirs, runner)?)
    } else if let Some(proton_config) = &config.proton {
        // Proton runtime is not defined in seed, but defined globally.
        // Use Proton runtime defined in global configuration.
        Ok(find_proton_tool(
            dirs,
            &proton_config.default_proton_runtime,
        )?)
    } else {
        // Proton runtime is not defined in seed nor global.
        // Define an empty runtime, and let umu-launcher decide.
        Ok(PathBuf::from(""))
    }
}

fn get_proton_prefix(
    dirs: &FloraDirs,
    config: &FloraConfig,
    proton_seed: &FloraProtonSeed,
) -> PathBuf {
    if let Some(path) = &proton_seed.proton_prefix {
        // Prefix is defined in seed
        // Use prefix defined by seed.
        PathBuf::from(path.clone())
    } else if let Some(proton_config) = &config.proton {
        // Prefix is not defined in seed, but there is a default prefix defined globally.
        // Use default prefix from global configuration.
        PathBuf::from(&proton_config.default_proton_prefix)
    } else {
        // Prefix is not defined in seed and default prefix is not set.
        // Use a well-known fallback prefix directory.
        dirs.get_fallback_prefix_proton()
    }
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

    if !fs::exists(proton_tool)? {
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

    if !fs::exists(proton_prefix)? {
        info!("Prefix not found, but will be created at launch");
    }

    Ok(())
}

fn get_system_start_menu_dir(
    dirs: &FloraDirs,
    config: &FloraConfig,
    proton_seed: &FloraProtonSeed,
) -> PathBuf {
    let mut proton_prefix = get_proton_prefix(dirs, config, proton_seed);
    proton_prefix.push("drive_c/ProgramData/Microsoft/Windows/Start Menu");

    proton_prefix
}

fn get_start_menu_dir(
    dirs: &FloraDirs,
    config: &FloraConfig,
    proton_seed: &FloraProtonSeed,
) -> PathBuf {
    let mut proton_prefix = get_proton_prefix(dirs, config, proton_seed);
    proton_prefix.push("drive_c/users");
    proton_prefix.push("steamuser");
    proton_prefix.push("AppData/Roaming/Microsoft/Windows/Start Menu");

    proton_prefix
}
pub fn find_start_menu_entry_location(
    dirs: &FloraDirs,
    config: &FloraConfig,
    seed: &FloraSeed,
    menu_name: &String,
) -> Result<String, FloraError> {
    if let FloraSeedType::Proton(proton_seed) = &seed.seed_type {
        let proton_prefix = get_proton_prefix(dirs, config, proton_seed);

        for start_menu_dir in vec![
            get_start_menu_dir(dirs, config, proton_seed),
            get_system_start_menu_dir(dirs, config, proton_seed),
        ] {
            for entry in WalkDir::new(start_menu_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if let Some(file_name) = entry.path().file_name()
                    && file_name.eq_ignore_ascii_case(format!("{}.lnk", menu_name))
                {
                    debug!("Found Start Menu item: {}", entry.path().display());
                    let path = String::from(entry.path().to_str().unwrap_or_default());

                    let winepath = winepath::unix_to_windows(&proton_prefix, &PathBuf::from(path));

                    debug!("Winepath: {}", winepath);
                    return Ok(winepath);
                }
            }
        }

        Err(FloraError::StartMenuNotFound)
    } else {
        Err(FloraError::IncorrectRunner)
    }
}

/// Run something in wine
pub fn run_proton_executable(
    name: &str,
    dirs: &FloraDirs,
    config: &FloraConfig,
    seed: &FloraSeed,
    args: &Vec<String>,
    quiet: bool,
    wait: bool,
) -> Result<(), FloraError> {
    if let FloraSeedType::Proton(proton_seed) = &seed.seed_type {
        let proton_tool = get_proton_tool(dirs, config, proton_seed)?;
        let proton_prefix = get_proton_prefix(dirs, config, proton_seed);

        ensure_proton_tool(&proton_tool)?;
        ensure_proton_prefix(&proton_prefix)?;

        debug!("Using {} to launch {}", "umu-run", args.join(" "));

        use std::process::Command;
        let mut command = Command::new("umu-run");
        command
            .env("WINEPREFIX", proton_prefix)
            .env("PROTONPATH", proton_tool)
            .args(args);

        if let Some(game_id) = &proton_seed.game_id {
            command.env("GAMEID", game_id);
        }
        if let Some(store) = &proton_seed.store {
            command.env("STORE", store);
        }
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
pub fn run_proton_tricks(
    name: &str,
    dirs: &FloraDirs,
    config: &FloraConfig,
    seed: &FloraSeed,
    args: &Option<Vec<String>>,
    quiet: bool,
    wait: bool,
) -> Result<(), FloraError> {
    let mut winetricks_path = vec![String::from("winetricks")];

    if let Some(additional_args) = args {
        winetricks_path.extend(additional_args.iter().cloned());
    }

    run_proton_executable(name, dirs, config, seed, &winetricks_path, quiet, wait)
}

pub fn run_proton_config(
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

    run_proton_executable(name, dirs, config, seed, &winecfg_path, quiet, wait)
}

pub fn create_desktop_entry(
    name: &String,
    dirs: &FloraDirs,
    config: &FloraConfig,
    seed: &FloraSeed,
) -> Result<(), FloraError> {
    if let FloraSeedType::Proton(proton_seed) = &seed.seed_type {
        let proton_prefix = get_proton_prefix(dirs, config, proton_seed);
        // Initialize menus
        desktop::initialize_desktop_entries(dirs)?;

        for app in seed.apps.iter() {
            // Get link path
            let target_linux_path =
                winepath::windows_to_unix(&proton_prefix, &app.application_location);

            let exe_find = flora_icon::find_lnk_exe_location(&target_linux_path)?;

            let icon_path = dirs.get_icon_file(name, &app.application_name);
            let mut icon_name = String::from("applications-other");

            if let FloraLink::WindowsIco(ico_path) = exe_find {
                let windows_ico_path = winepath::windows_to_unix(&proton_prefix, &ico_path);
                debug!(
                    "We got icon from {}",
                    &windows_ico_path
                        .clone()
                        .into_os_string()
                        .into_string()
                        .unwrap()
                );

                flora_icon::extract_icon_from_ico(&icon_path, &PathBuf::from(&windows_ico_path))?;
                icon_name = icon_path.into_os_string().into_string().unwrap_or_default()
            } else {
                debug!("No icon location, search exe for icons");
                let exe_location = match exe_find {
                    FloraLink::LinuxExe(path) => path,
                    FloraLink::WindowsExe(path) => winepath::windows_to_unix(&proton_prefix, &path),
                    _ => panic!("Windows ICO should be handled in the former case!"),
                };

                if flora_icon::extract_icon_from_exe(&icon_path, &exe_location)? {
                    debug!(
                        "We got icon from {}",
                        exe_location
                            .clone()
                            .into_os_string()
                            .into_string()
                            .unwrap_or_default()
                    );
                    icon_name = icon_path.into_os_string().into_string().unwrap_or_default()
                };
            }

            // Create desktop entry files
            let desktop_entry = format!(
                r#"[Desktop Entry]
Type=Application
Categories=X-Flora
Name={}
Icon={}
Exec=flora run -a -w {} "{}"
Comment=Run {} with Flora (Proton seed {})
Terminal=false"#,
                app.application_name,
                icon_name,
                name,
                app.application_name,
                app.application_name,
                name
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
    } else {
        Err(FloraError::IncorrectRunner)
    }
}
