use std::{fs, path::PathBuf, process::Stdio};

use flora_icon::FloraLink;
use log::{debug, info};
use walkdir::WalkDir;

use crate::{
    config::FloraConfig,
    desktop,
    dirs::FloraDirs,
    errors::FloraError,
    responses::FloraSeedStartMenuItem,
    runners::FloraRunner,
    seed::{FloraProtonSeed, FloraSeedApp},
    winepath,
};

pub struct FloraProtonRunner<'a> {
    name: &'a str,
    dirs: &'a FloraDirs,
    config: &'a FloraConfig,
    proton_seed: &'a FloraProtonSeed,
    apps: &'a Vec<FloraSeedApp>,
}

impl<'a> FloraProtonRunner<'a> {
    pub fn new(
        name: &'a str,
        dirs: &'a FloraDirs,
        config: &'a FloraConfig,
        proton_seed: &'a FloraProtonSeed,
        apps: &'a Vec<FloraSeedApp>,
    ) -> Self {
        Self {
            name,
            dirs,
            config,
            proton_seed,
            apps,
        }
    }
}

impl<'a> FloraProtonRunner<'a> {
    fn find_proton_tool(&self, name: &String) -> Result<PathBuf, FloraError> {
        // Flora Proton path
        let mut flora_proton_path = self.dirs.get_proton_root();
        flora_proton_path.push(name);

        // Local Steam Proton path
        let mut steam_proton_path = self.dirs.get_proton_root_steam();
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
    fn get_proton_tool(&self) -> Result<PathBuf, FloraError> {
        if let Some(runner) = &self.proton_seed.proton_runtime {
            // Proton runtime is defined in seed.
            // Use Proton runtime defined in seed.
            Ok(self.find_proton_tool(runner)?)
        } else if let Some(proton_config) = &self.config.proton {
            // Proton runtime is not defined in seed, but defined globally.
            // Use Proton runtime defined in global configuration.
            Ok(self.find_proton_tool(&proton_config.default_proton_runtime)?)
        } else {
            // Proton runtime is not defined in seed nor global.
            // Define an empty runtime, and let umu-launcher decide.
            Ok(PathBuf::from(""))
        }
    }

    fn get_proton_prefix(&self) -> PathBuf {
        if let Some(path) = &self.proton_seed.proton_prefix {
            // Prefix is defined in seed
            // Use prefix defined by seed.
            PathBuf::from(path.clone())
        } else if let Some(proton_config) = &self.config.proton {
            // Prefix is not defined in seed, but there is a default prefix defined globally.
            // Use default prefix from global configuration.
            PathBuf::from(&proton_config.default_proton_prefix)
        } else {
            // Prefix is not defined in seed and default prefix is not set.
            // Use a well-known fallback prefix directory.
            self.dirs.get_fallback_prefix_proton()
        }
    }

    fn ensure_proton_tool(&self, proton_tool: &PathBuf) -> Result<(), FloraError> {
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

    fn ensure_proton_prefix(&self, proton_prefix: &PathBuf) -> Result<(), FloraError> {
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

    fn get_system_start_menu_dir(&self) -> PathBuf {
        let mut proton_prefix = self.get_proton_prefix();
        proton_prefix.push("drive_c/ProgramData/Microsoft/Windows/Start Menu");

        proton_prefix
    }

    fn get_start_menu_dir(&self) -> PathBuf {
        let mut proton_prefix = self.get_proton_prefix();
        proton_prefix.push("drive_c/users");
        proton_prefix.push("steamuser");
        proton_prefix.push("AppData/Roaming/Microsoft/Windows/Start Menu");

        proton_prefix
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
        let proton_tool = self.get_proton_tool()?;
        let proton_prefix = self.get_proton_prefix();

        self.ensure_proton_tool(&proton_tool)?;
        self.ensure_proton_prefix(&proton_prefix)?;

        debug!("Using {} to launch {}", "umu-run", args.join(" "));

        use std::process::Command;
        let mut command = Command::new("umu-run");
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

    fn create_desktop_entries(&self) -> Result<(), FloraError> {
        let proton_prefix = self.get_proton_prefix();
        // Initialize menus
        desktop::initialize_desktop_entries(self.dirs)?;

        for app in self.apps.iter() {
            // Get link path
            let target_linux_path =
                winepath::windows_to_unix(&proton_prefix, &app.application_location);

            let exe_find = flora_icon::find_lnk_exe_location(&target_linux_path)?;

            let icon_path = self.dirs.get_icon_file(self.name, &app.application_name);
            let mut icon_name = String::from("applications-other");

            if let FloraLink::Other = exe_find {
                // Not an EXE or LNK, use other icon
            } else if let FloraLink::WindowsIco(ico_path) = exe_find {
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

    fn get_start_menu_entry_location(&self, menu_name: &str) -> Result<String, FloraError> {
        let proton_prefix = self.get_proton_prefix();

        for start_menu_dir in [self.get_start_menu_dir(), self.get_system_start_menu_dir()] {
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
    }

    fn list_start_menu_entries(&self) -> Result<Vec<FloraSeedStartMenuItem>, FloraError> {
        let proton_prefix = self.get_proton_prefix();
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
                        start_menu_name: String::from(file_stem.to_str().unwrap()),
                        start_menu_location: String::from(winepath::unix_to_windows(
                            &proton_prefix,
                            entry.path(),
                        )),
                    });
                }
            }
        }

        Ok(start_menu_entries)
    }
}
