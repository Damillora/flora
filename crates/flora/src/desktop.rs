use crate::{dirs::FloraDirs, errors::FloraError};

pub(crate) fn initialize_desktop_entries(dirs: &FloraDirs) -> Result<(), FloraError> {
    dirs.create_desktop_dirs()?;

    // let flora_directory = "[Desktop Entry]
    // Type=Directory
    // Name=Flora
    // Icon=flower-shape
    // ";
    // let flora_menu = r#"<!DOCTYPE Menu PUBLIC "-//freedesktop//DTD Menu 1.0//EN"
    // "http://www.freedesktop.org/standards/menu-spec/menu-1.0.dtd">
    // <Menu>
    // <Name>Applications</Name>
    // <Menu>
    // <Name>flora</Name>
    // <Directory>flora.directory</Directory>
    // <Include>
    // <Category>X-Flora</Category>
    // </Include>
    // </Menu>
    // </Menu>
    // "#;
    // let directory_file = dirs.get_desktop_directory_file();
    // debug!(
    //     "Writing directory entry to {}",
    //     directory_file.to_string_lossy()
    // );

    // if !fs::exists(&directory_file)? {
    //     fs::write(directory_file, flora_directory).map(|_| ())?;
    // }

    // let menu_file = dirs.get_desktop_menu_file();
    // debug!("Writing menu entry to {}", menu_file.to_string_lossy());

    // if !fs::exists(&menu_file)? {
    //     fs::write(menu_file, flora_menu).map(|_| ())?;
    // }

    Ok(())
}
