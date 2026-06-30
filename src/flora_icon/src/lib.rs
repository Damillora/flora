use std::{
    fmt::Display,
    fs,
    io::Cursor,
    path::{Path, PathBuf},
};

use lnk::encoding::WINDOWS_1252;
use log::debug;
use pelite::{FileMap, pe32, pe64, resources::FindError};
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

pub fn find_lnk_exe_location(lnk_location: &PathBuf) -> Result<FloraLink, FloraLinkError> {
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
                    .ok_or(FloraLinkError::LinkNoTarget)?
                    .trim_matches(char::from(0)) // Clean up null values
                    .to_string(),
            ))
        }
    } else {
        Ok(FloraLink::Other(lnk_location.to_owned()))
    }
}

pub fn extract_icon_from_ico(
    icon_path: &PathBuf,
    ico_location: &PathBuf,
) -> Result<(), FloraLinkError> {
    debug!("ICO location: {}", ico_location.to_string_lossy());

    let file = std::fs::File::open(ico_location)?;
    let icon_dir = ico::IconDir::read(file)?;

    // Get highest icon size
    let image = icon_dir
        .entries()
        .iter()
        .max_by(|a, b| a.width().cmp(&b.width()))
        .ok_or(FloraLinkError::NoIcons)?
        .decode()?;
    let file = std::fs::File::create(icon_path)?;
    image.write_png(file)?;

    Ok(())
}
pub fn extract_icon_from_exe(
    icon_path: &PathBuf,
    exe_location: &PathBuf,
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
                .ok_or(FloraLinkError::IconNotFound)?;

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
                .ok_or(FloraLinkError::IconNotFound)?
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
                .ok_or(FloraLinkError::IconNotFound)?;

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

#[derive(Debug)]
pub enum FloraLinkError {
    ExeParseError(pelite::Error),
    ExeResourceError(FindError),
    LinkNoTarget,
    IconNotFound,
    NoIcons,
    FileError(std::io::Error),
}
impl From<pelite::Error> for FloraLinkError {
    fn from(value: pelite::Error) -> Self {
        Self::ExeParseError(value)
    }
}
impl From<FindError> for FloraLinkError {
    fn from(value: FindError) -> Self {
        Self::ExeResourceError(value)
    }
}
impl From<std::io::Error> for FloraLinkError {
    fn from(value: std::io::Error) -> Self {
        Self::FileError(value)
    }
}

impl Display for FloraLinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            FloraLinkError::ExeParseError(err) => {
                write!(f, "Unable to inspect executable: {}", err)
            }
            FloraLinkError::ExeResourceError(err) => {
                write!(f, "Unable to inspect resources: {}", err)
            }
            FloraLinkError::LinkNoTarget => write!(f, "Cannot find target of shortcut"),
            FloraLinkError::IconNotFound => write!(f, "Unable to find icon"),
            FloraLinkError::NoIcons => write!(f, "No icons are found"),
            FloraLinkError::FileError(err) => write!(f, "Unable to complete the action: {}", err),
        }
    }
}
