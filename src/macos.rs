use serde::Deserialize;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Dock {
    pub orientation: Option<DockOrientation>,
    pub autohide: Option<bool>,
    #[serde(rename = "icon-size")]
    pub icon_size: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Safari {
    #[serde(rename = "show-full-url")]
    pub show_full_url: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DockOrientation {
    Left,
    Bottom,
    Right,
}

impl std::fmt::Display for DockOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DockOrientation::Left => write!(f, "left"),
            DockOrientation::Bottom => write!(f, "bottom"),
            DockOrientation::Right => write!(f, "right"),
        }
    }
}

#[derive(Debug, Error)]
pub enum MacOSError {
    #[error("Failed to read setting")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse setting")]
    ParseError,
    #[error("Failed to write setting")]
    WriteError,
}

pub fn apply_dock_settings(dock: &Dock) -> Result<(), MacOSError> {
    let mut changed = false;

    if let Some(orientation) = &dock.orientation {
        if set_dock_orientation(orientation)? {
            changed = true;
        }
    }

    if let Some(autohide) = dock.autohide {
        if set_dock_autohide(autohide)? {
            changed = true;
        }
    }

    if let Some(icon_size) = dock.icon_size {
        if set_dock_icon_size(icon_size)? {
            changed = true;
        }
    }

    if changed {
        println!("Restarting Dock to apply changes...");
        Command::new("killall").arg("Dock").status()?;
    }

    Ok(())
}

pub fn apply_safari_settings(safari: &Safari) -> Result<(), MacOSError> {
    let mut changed = false;

    if let Some(show_full_url) = safari.show_full_url {
        if set_safari_show_full_url(show_full_url)? {
            changed = true;
        }
    }

    if changed {
        println!("Restarting Safari to apply changes...");
        Command::new("killall").arg("Safari").status()?;
    }

    Ok(())
}

fn get_dock_orientation() -> Result<DockOrientation, MacOSError> {
    let output = Command::new("defaults")
        .args(["read", "com.apple.dock", "orientation"])
        .output()?;
    let orientation_str = String::from_utf8_lossy(&output.stdout).trim().to_lowercase();
    match orientation_str.as_str() {
        "left" => Ok(DockOrientation::Left),
        "bottom" => Ok(DockOrientation::Bottom),
        "right" => Ok(DockOrientation::Right),
        _ => Err(MacOSError::ParseError),
    }
}

fn set_dock_orientation(orientation: &DockOrientation) -> Result<bool, MacOSError> {
    let current_orientation = get_dock_orientation()?;
    if current_orientation != *orientation {
        println!("Setting dock orientation to '{}'", orientation);
        let status = Command::new("defaults")
            .args([
                "write",
                "com.apple.dock",
                "orientation",
                "-string",
                &orientation.to_string(),
            ])
            .status()?;

        if status.success() {
            println!("Dock orientation set to '{}'", orientation);
            Ok(true)
        } else {
            Err(MacOSError::WriteError)
        }
    } else {
        println!("Dock orientation is already set to '{}'", orientation);
        Ok(false)
    }
}

fn get_dock_autohide() -> Result<bool, MacOSError> {
    let output = Command::new("defaults")
        .args(["read", "com.apple.dock", "autohide"])
        .output()?;
    let autohide_str = String::from_utf8_lossy(&output.stdout).trim().to_lowercase();
    match autohide_str.as_str() {
        "1" | "true" => Ok(true),
        "0" | "false" => Ok(false),
        _ => Err(MacOSError::ParseError),
    }
}

fn set_dock_autohide(autohide: bool) -> Result<bool, MacOSError> {
    let current_autohide = get_dock_autohide()?;
    if current_autohide != autohide {
        println!("Setting dock autohide to '{}'", autohide);
        let status = Command::new("defaults")
            .args([
                "write",
                "com.apple.dock",
                "autohide",
                "-bool",
                &autohide.to_string(),
            ])
            .status()?;

        if status.success() {
            println!("Dock autohide set to '{}'", autohide);
            Ok(true)
        } else {
            Err(MacOSError::WriteError)
        }
    } else {
        println!("Dock autohide is already set to '{}'", autohide);
        Ok(false)
    }
}

fn get_dock_icon_size() -> Result<u32, MacOSError> {
    let output = Command::new("defaults")
        .args(["read", "com.apple.dock", "tilesize"])
        .output()?;
    let icon_size_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    icon_size_str
        .parse::<u32>()
        .map_err(|_| MacOSError::ParseError)
}

fn set_dock_icon_size(icon_size: u32) -> Result<bool, MacOSError> {
    let current_icon_size = get_dock_icon_size()?;
    if current_icon_size != icon_size {
        println!("Setting dock icon size to '{}'", icon_size);
        let status = Command::new("defaults")
            .args([
                "write",
                "com.apple.dock",
                "tilesize",
                "-int",
                &icon_size.to_string(),
            ])
            .status()?;

        if status.success() {
            println!("Dock icon size set to '{}'", icon_size);
            Ok(true)
        } else {
            Err(MacOSError::WriteError)
        }
    } else {
        println!("Dock icon size is already set to '{}'", icon_size);
        Ok(false)
    }
}

fn get_safari_show_full_url() -> Result<bool, MacOSError> {
    let output = Command::new("defaults")
        .args(["read", "com.apple.Safari", "ShowFullURLInSmartSearchField"])
        .output()?;
    let show_full_url_str = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_lowercase();
    match show_full_url_str.as_str() {
        "1" | "true" => Ok(true),
        "0" | "false" => Ok(false),
        _ => Err(MacOSError::ParseError),
    }
}

fn set_safari_show_full_url(show_full_url: bool) -> Result<bool, MacOSError> {
    let current_show_full_url = get_safari_show_full_url()?;
    if current_show_full_url != show_full_url {
        println!("Setting Safari show full URL to '{}'", show_full_url);
        let status = Command::new("defaults")
            .args([
                "write",
                "com.apple.Safari",
                "ShowFullURLInSmartSearchField",
                "-bool",
                &show_full_url.to_string(),
            ])
            .status()?;

        if status.success() {
            println!("Safari show full URL set to '{}'", show_full_url);
            Ok(true)
        } else {
            Err(MacOSError::WriteError)
        }
    } else {
        println!(
            "Safari show full URL is already set to '{}'",
            show_full_url
        );
        Ok(false)
    }
}
