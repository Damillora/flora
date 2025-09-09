use std::{fs, io::Cursor, path::PathBuf};

use lnk::encoding::WINDOWS_1252;
use log::debug;
use pelite::{FileMap, pe32, pe64, resources::FindError};
use png::EncodingError;

pub enum FloraLink {
    LinuxExe(PathBuf),
    WindowsIco(String),
    WindowsExe(String),
    Other,
}

pub fn find_lnk_exe_location(lnk_location: &PathBuf) -> Result<FloraLink, FloraLinkError> {
    debug!(
        "Inferring exe or lnk: {}",
        lnk_location
            .clone()
            .into_os_string()
            .into_string()
            .unwrap_or_default()
    );

    let is_exe = infer::app::is_exe(&fs::read(lnk_location).unwrap());

    if is_exe {
        Ok(FloraLink::LinuxExe(lnk_location.to_owned()))
    } else {
        if let Ok(shortcut) = lnk::ShellLink::open(lnk_location, WINDOWS_1252) {
            if let Some(icon_location) = shortcut.string_data().icon_location() {
                Ok(FloraLink::WindowsIco(
                    icon_location
                        .to_owned()
                        .trim_matches(char::from(0)) // Clean up null values
                        .to_string(),
                ))
            } else {
                Ok(FloraLink::WindowsExe(
                    shortcut
                        .link_target()
                        .unwrap_or_default()
                        .trim_matches(char::from(0)) // Clean up null values
                        .to_string(),
                ))
            }
        } else {
            Ok(FloraLink::Other)
        }
    }
}

pub fn extract_icon_from_ico(
    icon_path: &PathBuf,
    ico_location: &PathBuf,
) -> Result<(), FloraLinkError> {
    debug!(
        "ICO location: {}",
        ico_location
            .clone()
            .into_os_string()
            .into_string()
            .unwrap_or_default()
    );

    let file = std::fs::File::open(ico_location)?;
    let icon_dir = ico::IconDir::read(file)?;

    // Get highest icon size
    let image = icon_dir
        .entries()
        .iter()
        .max_by(|a, b| a.width().cmp(&b.width()))
        .unwrap()
        .decode()?;
    let file = std::fs::File::create(icon_path)?;
    image.write_png(file).unwrap();

    Ok(())
}
pub fn extract_icon_from_exe(
    icon_path: &PathBuf,
    exe_location: &PathBuf,
) -> Result<bool, FloraLinkError> {
    // Extract main icon from executable

    if let Ok(map) = FileMap::open(exe_location) {
        if let Ok(pe64_exe) = pe64::PeFile::from_bytes(&map) {
            debug!(
                "PE64 executable: {}",
                exe_location
                    .clone()
                    .into_os_string()
                    .into_string()
                    .unwrap_or_default()
            );

            use pe64::Pe;
            // PE64 executable

            // Get first icon resource on the EXE
            let icon_group = pe64_exe
                .resources()?
                .icons()
                .filter_map(|e| e.ok())
                .next()
                .ok_or(FloraLinkError::IconError)?;

            // Get ICO resource
            let mut ico_file = vec![];
            icon_group.1.write(&mut ico_file)?;

            let ico_cursor = Cursor::new(ico_file);

            let icon_dir = ico::IconDir::read(ico_cursor)?;

            // Get highest icon size
            let image = icon_dir
                .entries()
                .iter()
                .max_by(|a, b| a.width().cmp(&b.width()))
                .unwrap()
                .decode()?;
            let file = std::fs::File::create(icon_path)?;
            image.write_png(file).unwrap();

            return Ok(true);
        } else if let Ok(pe32_exe) = pe32::PeFile::from_bytes(&map) {
            debug!(
                "PE32 executable: {}",
                exe_location
                    .clone()
                    .into_os_string()
                    .into_string()
                    .unwrap_or_default()
            );

            use pe32::Pe;
            // PE32 executable

            // Get first icon resource on the EXE
            let icon_group = pe32_exe
                .resources()?
                .icons()
                .filter_map(|e| e.ok())
                .next()
                .ok_or(FloraLinkError::IconError)?;

            // Get ICO resource
            let mut ico_file = vec![];
            icon_group.1.write(&mut ico_file)?;

            let ico_cursor = Cursor::new(ico_file);

            let icon_dir = ico::IconDir::read(ico_cursor)?;

            let image = icon_dir.entries()[0].decode()?;
            let file = std::fs::File::create(icon_path)?;
            image.write_png(file).unwrap();

            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    Ok(false)
}

#[derive(Debug)]
pub enum FloraLinkError {
    InvalidFormat,
    ExeError(String),
    ExeResourceError(String),
    IconError,
    PngError,
    FileError(std::io::Error),
}
impl From<pelite::Error> for FloraLinkError {
    fn from(value: pelite::Error) -> Self {
        Self::ExeError(value.to_string())
    }
}
impl From<FindError> for FloraLinkError {
    fn from(value: FindError) -> Self {
        Self::ExeResourceError(value.to_string())
    }
}
impl From<EncodingError> for FloraLinkError {
    fn from(value: EncodingError) -> Self {
        debug!("{:?}", value);
        Self::PngError
    }
}
impl From<std::io::Error> for FloraLinkError {
    fn from(value: std::io::Error) -> Self {
        Self::FileError(value)
    }
}
