use std::{
    collections::BTreeMap,
    fs,
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
    seed::{FloraSeedApp, FloraSeedSettings, FloraWineSeed},
    winepath,
};

pub struct FloraWineRunner<'a> {
    name: &'a str,
    dirs: &'a FloraDirs,
    settings: &'a Option<Box<FloraSeedSettings>>,
    env: &'a Option<BTreeMap<String, String>>,

    prefix: PathBuf,
    runtime: PathBuf,
}

impl<'a> FloraWineRunner<'a> {
    pub fn new(
        name: &'a str,
        dirs: &'a FloraDirs,
        config: &'a FloraConfig,
        settings: &'a Option<Box<FloraSeedSettings>>,
        env: &'a Option<BTreeMap<String, String>>,
        wine_seed: &'a FloraWineSeed,
    ) -> Result<Self, FloraError> {
        let wine_prefix = if let Some(path) = &wine_seed.wine_prefix {
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
        };

        let wine_runtime = if let Some(runner) = &wine_seed.wine_runtime {
            // Wine runtime is defined in seed.
            // Use Wine runtime defined in seed.
            let mut wine_path = dirs.get_wine_root();
            wine_path.push(runner);
            PathBuf::from(&wine_path)
        } else if let Some(wine_config) = &config.wine {
            if let Some(default_wine_runtime) = &wine_config.default_wine_runtime
                && !default_wine_runtime.is_empty()
            {
                // Wine runtime is not defined in seed, but defined globally.
                // Use Wine runtime defined in global configuration
                let mut wine_path = dirs.get_wine_root();
                wine_path.push(default_wine_runtime.clone());
                PathBuf::from(&wine_path)
            } else {
                PathBuf::from("/usr")
            }
        } else {
            // Wine runtime is not defined in seed and globally.
            // Use system wine in /usr
            PathBuf::from("/usr")
        };
        debug!("Wine dir: {}", &wine_runtime.to_string_lossy());

        if !fs::exists(&wine_runtime)? {
            return Err(FloraError::MissingRunner);
        }
        debug!("Wine prefix: {}", wine_prefix.to_string_lossy());

        if !fs::exists(&wine_prefix)? {
            info!("Prefix not found, but will be created at launch");
        }

        Ok(Self {
            name,
            dirs,
            settings,
            env,

            prefix: wine_prefix,
            runtime: wine_runtime,
        })
    }
}

impl<'a> FloraWineRunner<'a> {
    fn get_start_menu_dir(&self) -> PathBuf {
        let mut wine_prefix = self.prefix.clone();
        wine_prefix.push("drive_c/users");
        wine_prefix.push(whoami::username());
        wine_prefix.push("AppData/Roaming/Microsoft/Windows/Start Menu");

        wine_prefix
    }

    fn get_system_start_menu_dir(&self) -> PathBuf {
        let mut wine_prefix = self.prefix.clone();
        wine_prefix.push("drive_c/ProgramData/Microsoft/Windows/Start Menu");

        wine_prefix
    }

    fn gather_command_info(&self) -> Result<(PathBuf, PathBuf), FloraError> {
        Ok((self.runtime.clone(), self.prefix.clone()))
    }
    fn generate_command(&self, args: &[&str]) -> Result<Command, FloraError> {
        let (wine_runtime, wine_prefix) = self.gather_command_info()?;

        let mut wine_exe = wine_runtime.clone();
        if !wine_runtime.to_string_lossy().is_empty() {
            wine_exe.push("bin/wine");
        } else {
            // Use system wine
            wine_exe.push("/usr/bin/wine");
        }

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
            command.arg(&wine_exe);

            command
        } else {
            Command::new(&wine_exe)
        };

        if let Some(envs) = self.env {
            for (env_name, env_val) in envs {
                command.env(env_name, env_val);
            }
        }
        command.env("WINEPREFIX", wine_prefix).args(args);

        debug!(
            "Using {} to launch {}",
            wine_exe.to_string_lossy(),
            args.join(" ")
        );

        Ok(command)
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
        let (wine_dir, wine_prefix) = self.gather_command_info()?;

        let mut wine_exe = wine_dir.clone();
        if !wine_dir.as_os_str().is_empty() {
            wine_exe.push("bin/wine");
        } else {
            // Use system wine
            wine_exe.push("/usr/bin/wine");
        }

        debug!("Using {} for winetricks", wine_exe.to_string_lossy());

        use std::process::Command;
        let mut command = Command::new("winetricks");
        if let Some(envs) = self.env {
            for (env_name, env_val) in envs {
                command.env(env_name, env_val);
            }
        }
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
                icon_name = String::from(icon_path.to_string_lossy());
            };
        }

        // Create desktop entry files
        let desktop_entry = format!(
            "[Desktop Entry]
Type=Application
Categories={}
Name={}
Icon={}
Exec=flora run -a -w {} \"{}\"
Comment=Run {} with Flora (Wine seed {})
Terminal=false",
            app.category.clone().unwrap_or(String::from("X-Flora")),
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
