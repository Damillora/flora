use std::path::{Component, Path, PathBuf};

use typed_path::{Utf8WindowsComponent, Utf8WindowsPathBuf, Utf8WindowsPrefix};

pub fn windows_to_unix(wine_prefix: &Path, windows_path: &String) -> PathBuf {
    let windows_path = Utf8WindowsPathBuf::from(windows_path);
    let mut unix_path = wine_prefix.to_path_buf();

    for component in windows_path.components() {
        if let Utf8WindowsComponent::Prefix(drive_letter) = component {
            unix_path.push("dosdevices");
            if let Utf8WindowsPrefix::Disk(disk) = drive_letter.kind() {
                unix_path.push(format!("{}:", disk.to_string().to_lowercase()));
            }
        } else if let Utf8WindowsComponent::Normal(normal) = component {
            unix_path.push(normal);
        }
    }

    unix_path
}

pub fn unix_to_windows(prefix: &PathBuf, unix_path: &Path) -> String {
    let mut dosdevices = prefix.clone();
    dosdevices.push("dosdevices");

    let mut windows_path = Utf8WindowsPathBuf::from("Z:\\");

    if unix_path.starts_with(&dosdevices) {
        let unix_path = unix_path.strip_prefix(&dosdevices).unwrap();

        let mut first_dir = true;
        for component in unix_path.components() {
            if let Component::Normal(normal) = component {
                if first_dir {
                    windows_path = Utf8WindowsPathBuf::from(format!(
                        "{}\\",
                        normal.to_str().unwrap().to_uppercase()
                    ));
                    first_dir = false;
                } else {
                    windows_path.push(normal.to_str().unwrap());
                }
            }
        }
    } else if unix_path.starts_with(prefix) && {
        let all_path = unix_path.strip_prefix(prefix).unwrap();
        let all_path: Vec<&str> = all_path
            .components()
            .filter_map(|i| match i {
                Component::Normal(normal) => Some(normal.to_str().unwrap()),
                _ => None,
            })
            .collect();
        all_path.first().unwrap().starts_with("drive_")
    } {
        let unix_path = unix_path.strip_prefix(prefix).unwrap();

        let mut first_dir = true;
        for component in unix_path.components() {
            if let Component::Normal(normal) = component {
                if first_dir {
                    windows_path = Utf8WindowsPathBuf::from(format!(
                        "{}:\\",
                        normal
                            .to_str()
                            .unwrap()
                            .strip_prefix("drive_")
                            .unwrap()
                            .to_uppercase()
                    ));
                    first_dir = false;
                } else {
                    windows_path.push(normal.to_str().unwrap());
                }
            }
        }
    } else {
        for component in unix_path.components() {
            if let Component::Normal(normal) = component {
                windows_path.push(normal.to_str().unwrap());
            }
        }
    };
    windows_path.to_string()
}
