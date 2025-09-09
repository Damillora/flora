use std::{fs, path::PathBuf, process::Stdio};

use flora_icon::FloraLink;
use log::{debug, info};
use walkdir::WalkDir;

use crate::{
    config::FloraConfig,
    desktop,
    dirs::FloraDirs,
    errors::FloraError,
    runners::FloraRunner,
    seed::{FloraSeedApp, FloraWineSeed},
    winepath,
};

pub struct FloraWineRunner<'a> {
    name: &'a str,
    dirs: &'a FloraDirs,
    config: &'a FloraConfig,
    wine_seed: &'a FloraWineSeed,
    apps: &'a Vec<FloraSeedApp>,
}

impl<'a> FloraWineRunner<'a> {
    pub fn new(
        name: &'a str,
        dirs: &'a FloraDirs,
        config: &'a FloraConfig,
        wine_seed: &'a FloraWineSeed,
        apps: &'a Vec<FloraSeedApp>,
    ) -> Self {
        Self {
            name,
            dirs,
            config,
            wine_seed,
            apps,
        }
    }
}

impl<'a> FloraWineRunner<'a> {
    fn get_wine_dir(&self) -> PathBuf {
        if let Some(runner) = &self.wine_seed.wine_runtime {
            // Wine runtime is defined in seed.
            // Use Wine runtime defined in seed.
            let mut wine_path = self.dirs.get_wine_root();
            wine_path.push(runner);
            PathBuf::from(&wine_path)
        } else if let Some(wine_config) = &self.config.wine {
            if let Some(default_wine_runtime) = &wine_config.default_wine_runtime
                && !default_wine_runtime.is_empty()
            {
                // Wine runtime is not defined in seed, but defined globally.
                // Use Wine runtime defined in global configuration
                let mut wine_path = self.dirs.get_wine_root();
                wine_path.push(default_wine_runtime.clone());
                PathBuf::from(&wine_path)
            } else {
                PathBuf::from("/usr")
            }
        } else {
            // Wine runtime is not defined in seed and globally.
            // Use system wine in /usr
            PathBuf::from("/usr")
        }
    }

    fn get_wine_prefix(&self) -> PathBuf {
        if let Some(path) = &self.wine_seed.wine_prefix {
            // Prefix is defined in seed
            // Use prefix defined in seed.
            PathBuf::from(path.clone())
        } else if let Some(wine_config) = &self.config.wine {
            // Prefix is not defined in seed, but there is a default prefix defined globally.
            // Use default prefix from global configuration.
            PathBuf::from(&wine_config.default_wine_prefix)
        } else {
            // Prefix is not defined in seed and default prefix is not set.
            // Use a well-known fallback prefix directory.
            self.dirs.get_fallback_prefix()
        }
    }

    fn ensure_wine_dir(&self, wine_dir: &PathBuf) -> Result<(), FloraError> {
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

    fn ensure_wine_prefix(&self, wine_prefix: &PathBuf) -> Result<(), FloraError> {
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

    fn get_start_menu_dir(&self) -> PathBuf {
        let mut wine_prefix = self.get_wine_prefix();
        wine_prefix.push("drive_c/users");
        wine_prefix.push(whoami::username());
        wine_prefix.push("AppData/Roaming/Microsoft/Windows/Start Menu");

        wine_prefix
    }

    fn get_system_start_menu_dir(&self) -> PathBuf {
        let mut wine_prefix = self.get_wine_prefix();
        wine_prefix.push("drive_c/ProgramData/Microsoft/Windows/Start Menu");

        wine_prefix
    }
}

impl<'a> FloraRunner for FloraWineRunner<'a> {
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
        let wine_dir = self.get_wine_dir();
        let wine_prefix = self.get_wine_prefix();

        self.ensure_wine_dir(&wine_dir)?;
        self.ensure_wine_prefix(&wine_prefix)?;

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
            let log_out = self.dirs.get_log_file(self.name)?;
            let log_err = self.dirs.get_log_file(self.name)?;
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
    }

    fn run_executable(&self, args: &[&str], quiet: bool, wait: bool) -> Result<(), FloraError> {
        let wine_dir = self.get_wine_dir();
        let wine_prefix = self.get_wine_prefix();

        self.ensure_wine_dir(&wine_dir)?;
        self.ensure_wine_prefix(&wine_prefix)?;

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
        let wine_prefix = self.get_wine_prefix();

        // Initialize menus
        desktop::initialize_desktop_entries(self.dirs)?;

        for app in self.apps.iter() {
            // Get link path
            let target_linux_path =
                winepath::windows_to_unix(&wine_prefix, &app.application_location);

            let exe_find = flora_icon::find_lnk_exe_location(&target_linux_path)?;

            let icon_path = self.dirs.get_icon_file(self.name, &app.application_name);
            let mut icon_name = String::from("applications-other");

            if let FloraLink::WindowsIco(ico_path) = exe_find {
                let windows_ico_path = winepath::windows_to_unix(&wine_prefix, &ico_path);
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
                    FloraLink::WindowsExe(path) => winepath::windows_to_unix(&wine_prefix, &path),
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
                Comment=Run {} with Flora (Wine seed {})
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
        let wine_prefix = self.get_wine_prefix();

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

                    let winepath = winepath::unix_to_windows(&wine_prefix, &PathBuf::from(path));

                    debug!("Winepath: {}", winepath);
                    return Ok(winepath);
                }
            }
        }

        Err(FloraError::StartMenuNotFound)
    }
}
