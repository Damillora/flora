use std::{
    collections::BTreeMap,
    env, fs,
    path::PathBuf,
    process::{Command, Stdio},
};

use flora_icon::FloraLink;
use log::{debug, info};
use walkdir::WalkDir;

use crate::{
    config::FloraConfig,
    dirs::FloraDirs,
    errors::FloraError,
    responses::FloraSeedStartMenuItem,
    runners::FloraRunner,
    seed::{FloraProtonSeed, FloraSeedApp, FloraSeedSettings},
    winepath,
};

pub struct FloraProtonRunner<'a> {
    name: &'a str,
    dirs: &'a FloraDirs,
    settings: &'a Option<Box<FloraSeedSettings>>,
    env: &'a Option<BTreeMap<String, String>>,
    proton_seed: &'a FloraProtonSeed,

    prefix: PathBuf,
    runtime: PathBuf,
    umu: String,
}

fn find_proton_tool(dirs: &FloraDirs, name: &String) -> Result<PathBuf, FloraError> {
    // Flora Proton path
    let mut flora_proton_path = dirs.get_proton_root();
    flora_proton_path.push(name);

    // Local Steam Proton path
    let mut steam_proton_path = dirs.get_proton_root_steam();
    steam_proton_path.push(name);
    let mut flatpak_steam_proton_path = PathBuf::new();

    if let Some(home_path) = env::home_dir() {
        flatpak_steam_proton_path = home_path.clone();
        flatpak_steam_proton_path
            .push(".var/app/com.valvesoftware.Steam/.steam/root/compatibilitytools.d");
        flatpak_steam_proton_path.push(name);
    }

    // System Steam Proton Path
    let mut steam_proton_path_system = PathBuf::from("/usr/share/steam/compatibilitytools.d");
    steam_proton_path_system.push(name);

    if fs::exists(&flora_proton_path)? {
        Ok(flora_proton_path)
    } else if fs::exists(&steam_proton_path)? {
        Ok(steam_proton_path)
    } else if fs::exists(&flatpak_steam_proton_path)? {
        Ok(steam_proton_path)
    } else if fs::exists(&steam_proton_path_system)? {
        Ok(steam_proton_path_system)
    } else {
        Ok(PathBuf::from(&name))
    }
}
impl<'a> FloraProtonRunner<'a> {
    pub fn new(
        name: &'a str,
        dirs: &'a FloraDirs,
        config: &'a FloraConfig,
        settings: &'a Option<Box<FloraSeedSettings>>,
        env: &'a Option<BTreeMap<String, String>>,
        proton_seed: &'a FloraProtonSeed,
    ) -> Result<Self, FloraError> {
        let proton_prefix = if let Some(path) = &proton_seed.proton_prefix {
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
        };

        let proton_runtime = if let Some(runner) = &proton_seed.proton_runtime {
            // Proton runtime is defined in seed.
            // Use Proton runtime defined in seed.
            find_proton_tool(&dirs, &runner)?
        } else if let Some(proton_config) = &config.proton {
            // Proton runtime is not defined in seed, but defined globally.
            // Use Proton runtime defined in global configuration.
            find_proton_tool(&dirs, &proton_config.default_proton_runtime)?
        } else {
            // Proton runtime is not defined in seed nor global.
            // Define an empty runtime, and let umu-launcher decide.
            PathBuf::from("")
        };

        let mut local_umu_path = dirs.get_umu_root();
        local_umu_path.push("umu-run");

        let umu = if fs::exists(&local_umu_path)? {
            // Use local installed umu
            String::from(local_umu_path.to_string_lossy())
        } else {
            // Use system installed umu
            String::from("umu-run")
        };

        // Check proton runtime folder
        debug!("Proton runtime dir: {}", proton_runtime.to_string_lossy());

        if !fs::exists(&proton_runtime)? {
            return Err(FloraError::MissingRunner);
        }

        // Check proton prefix folder
        debug!("Proton prefix: {}", proton_prefix.to_string_lossy());

        if !fs::exists(&proton_prefix)? {
            info!("Prefix not found, but will be created at launch");
        }

        Ok(Self {
            name,
            dirs,
            settings,
            proton_seed,
            env,

            prefix: proton_prefix,
            runtime: proton_runtime,
            umu,
        })
    }
}

impl<'a> FloraProtonRunner<'a> {
    fn get_system_start_menu_dir(&self) -> PathBuf {
        let mut proton_prefix = self.prefix.clone();
        proton_prefix.push("drive_c/ProgramData/Microsoft/Windows/Start Menu");

        proton_prefix
    }

    fn get_start_menu_dir(&self) -> PathBuf {
        let mut proton_prefix = self.prefix.clone();
        proton_prefix.push("drive_c/users");
        proton_prefix.push("steamuser");
        proton_prefix.push("AppData/Roaming/Microsoft/Windows/Start Menu");

        proton_prefix
    }
    fn gather_command_info(&self) -> Result<(PathBuf, PathBuf), FloraError> {
        Ok((self.runtime.clone(), self.prefix.clone()))
    }
    fn generate_command(&self, args: &[&str]) -> Result<Command, FloraError> {
        let (proton_tool, proton_prefix) = self.gather_command_info()?;
        let mut command = if let Some(settings) = self.settings
            && let Some(launcher) = &settings.launcher_command
        {
            let command_param = shlex::split(launcher).ok_or(FloraError::IncorrectLauncher)?;
            let (launch_command, launch_args) = (
                &command_param.first().ok_or(FloraError::IncorrectLauncher)?,
                &command_param[1..],
            );

            let mut command = Command::new(launch_command);
            command.args(launch_args);
            command.arg(&self.umu);

            command
        } else {
            Command::new(&self.umu)
        };

        if let Some(envs) = self.env {
            for (env_name, env_val) in envs {
                command.env(env_name, env_val);
            }
        }
        command
            .env("WINEPREFIX", proton_prefix)
            .env("PROTONPATH", proton_tool)
            .args(args);

        if let Some(game_id) = &self.proton_seed.game_id {
            command.env("GAMEID", game_id);
        }
        if let Some(store) = &self.proton_seed.store {
            command.env("STORE", store);
        }

        debug!("Using {} to launch {}", &self.umu, args.join(" "));

        Ok(command)
    }
}
impl<'a> FloraRunner for FloraProtonRunner<'a> {
    fn run_config(
        &self,
        args: &Option<Vec<&str>>,
        quiet: bool,
        wait: bool,
    ) -> Result<(), FloraError> {
        let mut winecfg_path = vec!["winecfg"];

        if let Some(additional_args) = args {
            winecfg_path.extend(additional_args.iter().cloned());
        }

        self.run_executable(&winecfg_path, quiet, wait)
    }

    fn run_tricks(
        &self,
        args: &Option<Vec<&str>>,
        quiet: bool,
        wait: bool,
    ) -> Result<(), FloraError> {
        let mut winetricks_path = vec!["winetricks"];

        if let Some(additional_args) = args {
            winetricks_path.extend(additional_args.iter().cloned());
        }

        self.run_executable(&winetricks_path, quiet, wait)
    }

    fn run_executable(&self, args: &[&str], quiet: bool, wait: bool) -> Result<(), FloraError> {
        let mut command = self.generate_command(args)?;

        if quiet {
            let log_out = self.dirs.get_log_file(self.name)?;
            let log_err = self.dirs.get_log_file(self.name)?;
            command.stdin(Stdio::null()).stdout(log_out).stderr(log_err);
        }

        let mut handle = command.spawn()?;
        if wait {
            handle.wait()?;
        }

        Ok(())
    }
    fn create_desktop_entry(&self, app: &FloraSeedApp) -> Result<(), FloraError> {
        // Get link path
        let target_linux_path = winepath::windows_to_unix(&self.prefix, &app.application_location);

        let exe_find = flora_icon::find_lnk_exe_location(&target_linux_path)?;

        let icon_path = self.dirs.get_icon_file(self.name, &app.application_name);
        let mut icon_name = String::from("applications-other");

        if let FloraLink::Other(location) = exe_find {
            // Not an EXE or LNK, use other icon
            icon_name = flora_icon::get_icon_name_from_path(&location)?;
        } else if let FloraLink::WindowsIco(ico_path) = exe_find {
            let windows_ico_path = winepath::windows_to_unix(&self.prefix, &ico_path);
            debug!("We got icon from {}", &windows_ico_path.to_string_lossy());

            flora_icon::extract_icon_from_ico(&icon_path, &PathBuf::from(&windows_ico_path))?;
            icon_name = String::from(icon_path.to_string_lossy())
        } else {
            debug!("No icon location, search exe for icons");
            let exe_location = match exe_find {
                FloraLink::LinuxExe(path) => path,
                FloraLink::WindowsExe(path) => winepath::windows_to_unix(&self.prefix, &path),
                _ => panic!("Windows ICO should be handled in the former case!"),
            };

            if flora_icon::extract_icon_from_exe(&icon_path, &exe_location)? {
                debug!("We got icon from {}", exe_location.to_string_lossy());
                icon_name = String::from(icon_path.to_string_lossy())
            };
        }

        // Create desktop entry files
        let desktop_entry = format!(
            "[Desktop Entry]
Type=Application
Categories=X-Flora
Name={}
Icon={}
Exec=flora run -a -w {} \"{}\"
Comment=Run {} with Flora (Proton seed {})
Terminal=false",
            app.application_name,
            icon_name,
            self.name,
            app.application_name,
            app.application_name,
            self.name
        );

        let desktop_entry_location = self
            .dirs
            .get_desktop_entry_file(self.name, &app.application_name);

        debug!(
            "Writing {} desktop entry to {}",
            self.name,
            desktop_entry_location.to_string_lossy()
        );

        fs::write(desktop_entry_location, desktop_entry)?;

        Ok(())
    }

    fn get_start_menu_entry_location(&self, menu_name: &str) -> Result<String, FloraError> {
        for start_menu_dir in [self.get_start_menu_dir(), self.get_system_start_menu_dir()] {
            for entry in WalkDir::new(start_menu_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if let Some(file_name) = entry.path().file_name()
                    && file_name.eq_ignore_ascii_case(format!("{}.lnk", menu_name))
                {
                    debug!("Found Start Menu item: {}", entry.path().display());
                    let path = String::from(entry.path().to_string_lossy());

                    let winepath = winepath::unix_to_windows(&self.prefix, &PathBuf::from(path));

                    debug!("Winepath: {}", winepath);
                    return Ok(winepath);
                }
            }
        }

        Err(FloraError::StartMenuNotFound)
    }

    fn list_start_menu_entries(&self) -> Result<Vec<FloraSeedStartMenuItem>, FloraError> {
        let mut start_menu_entries = Vec::new();

        for start_menu_dir in [self.get_start_menu_dir(), self.get_system_start_menu_dir()] {
            for entry in WalkDir::new(start_menu_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if let Some(extension) = entry.path().extension()
                    && extension == "lnk"
                    && let Some(file_stem) = entry.path().file_stem()
                {
                    debug!("Found Start Menu item: {}", entry.path().display());

                    start_menu_entries.push(FloraSeedStartMenuItem {
                        start_menu_name: String::from(file_stem.to_string_lossy()),
                        start_menu_location: winepath::unix_to_windows(&self.prefix, entry.path()),
                    });
                }
            }
        }

        Ok(start_menu_entries)
    }
}
