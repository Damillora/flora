use std::{
    fs,
    io::Cursor,
    path::{Path, PathBuf},
};

use lnk::encoding::WINDOWS_1252;
use log::debug;
use pelite::{FileMap, pe32, pe64, resources::FindError};
use thiserror::Error;
use xdg_mime::SharedMimeInfo;

pub enum FloraLink {
    LinuxExe(PathBuf),
    WindowsIco(String),
    WindowsExe(String),
    Other(PathBuf),
}

pub fn get_icon_name_from_path(lnk_location: &Path) -> Result<String, FloraLinkError> {
    let mime_db = SharedMimeInfo::new();
    let mut guess_builder = mime_db.guess_mime_type();
    let guess = guess_builder
        .file_name(&lnk_location.to_string_lossy())
        .guess();

    Ok(mime_db
        .lookup_generic_icon_name(guess.mime_type())
        .unwrap_or(String::from("applications-other")))
}

pub fn find_lnk_exe_location(lnk_location: &Path) -> Result<FloraLink, FloraLinkError> {
    debug!("Inferring exe or lnk: {}", lnk_location.to_string_lossy());

    let is_exe = infer::app::is_exe(&fs::read(lnk_location)?);

    if is_exe {
        Ok(FloraLink::LinuxExe(lnk_location.to_owned()))
    } else if let Ok(shortcut) = lnk::ShellLink::open(lnk_location, WINDOWS_1252) {
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
                    .ok_or(FloraLinkError::LinkNoTarget(lnk_location.to_path_buf()))?
                    .trim_matches(char::from(0)) // Clean up null values
                    .to_string(),
            ))
        }
    } else {
        Ok(FloraLink::Other(lnk_location.to_owned()))
    }
}

pub fn extract_icon_from_ico(
    icon_path: &Path,
    ico_location: &Path,
) -> Result<(), FloraLinkError> {
    debug!("ICO location: {}", ico_location.to_string_lossy());

    let file = std::fs::File::open(ico_location)?;
    let icon_dir = ico::IconDir::read(file)?;

    // Get highest icon size
    let image = icon_dir
        .entries()
        .iter()
        .max_by(|a, b| a.width().cmp(&b.width()))
        .ok_or(FloraLinkError::IconNotInIcoFile(ico_location.to_path_buf()))?
        .decode()?;
    let file = std::fs::File::create(icon_path)?;
    image.write_png(file)?;

    Ok(())
}
pub fn extract_icon_from_exe(
    icon_path: &Path,
    exe_location: &Path,
) -> Result<bool, FloraLinkError> {
    // Extract main icon from executable

    if let Ok(map) = FileMap::open(exe_location) {
        if let Ok(pe64_exe) = pe64::PeFile::from_bytes(&map) {
            debug!("PE64 executable: {}", exe_location.to_string_lossy());

            use pe64::Pe;
            // PE64 executable

            // Get first icon resource on the EXE
            let icon_group = pe64_exe
                .resources()?
                .icons()
                .filter_map(|e| e.ok())
                .next()
                .ok_or(FloraLinkError::IconNotInExecutable(exe_location.to_path_buf()))?;

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
                .ok_or(FloraLinkError::IconNotInExecutable(exe_location.to_path_buf()))?
                .decode()?;
            let file = std::fs::File::create(icon_path)?;
            image.write_png(file)?;

            return Ok(true);
        } else if let Ok(pe32_exe) = pe32::PeFile::from_bytes(&map) {
            debug!("PE32 executable: {}", exe_location.to_string_lossy());

            use pe32::Pe;
            // PE32 executable

            // Get first icon resource on the EXE
            let icon_group = pe32_exe
                .resources()?
                .icons()
                .filter_map(|e| e.ok())
                .next()
                .ok_or(FloraLinkError::IconNotInExecutable(exe_location.to_path_buf()))?;

            // Get ICO resource
            let mut ico_file = vec![];
            icon_group.1.write(&mut ico_file)?;

            let ico_cursor = Cursor::new(ico_file);

            let icon_dir = ico::IconDir::read(ico_cursor)?;

            let image = icon_dir.entries()[0].decode()?;
            let file = std::fs::File::create(icon_path)?;
            image.write_png(file)?;

            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    Ok(false)
}

#[derive(Error, Debug)]
pub enum FloraLinkError {
    #[error("Cannot parse executable file: {0}")]
    ExeParseError(#[from] pelite::Error),
    #[error("Cannot find executable resource: {0}")]
    ExeResourceError(#[from] FindError),
    #[error("Unable to find target of link {0}")]
    LinkNoTarget(PathBuf),
    #[error("Unable to find icon in executable {0}")]
    IconNotInExecutable(PathBuf),
    #[error("Unable to find icon in ico file {0}")]
    IconNotInIcoFile(PathBuf),
    #[error("Unable to edit file: {0}")]
    FileError(#[from] std::io::Error),
}
