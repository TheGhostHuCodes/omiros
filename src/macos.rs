use serde::Deserialize;
use std::process::Command;
use thiserror::Error;

use crate::defaults::{DefaultsError, DockOrientation, write_defaults};

/// Represents the Dock configuration.
#[derive(Debug, Deserialize)]
pub struct Dock {
    pub orientation: Option<DockOrientation>,
    pub autohide: Option<bool>,
    #[serde(rename = "icon-size")]
    pub icon_size: Option<i32>,
    #[serde(rename = "transparent-hidden-app-icons")]
    pub transparent_hidden_app_icons: Option<bool>,
}

/// Represents the Mission Control configuration.
#[derive(Debug, Deserialize)]
pub struct MissionControl {
    #[serde(rename = "automatically-rearrange-spaces")]
    pub automatically_rearrange_spaces: Option<bool>,
    #[serde(rename = "group-apps")]
    pub group_apps: Option<bool>,
}

/// Represents the Safari configuration.
#[derive(Debug, Deserialize)]
pub struct Safari {
    #[serde(rename = "show-full-url")]
    pub show_full_url: Option<bool>,
}

/// Represents the system-wide configuration.
#[derive(Debug, Deserialize)]
pub struct SystemSettings {
    #[serde(rename = "show-file-extensions")]
    pub show_file_extensions: Option<bool>,
    #[serde(rename = "weird-mac-scrolling")]
    pub natural_scrolling: Option<bool>,
}

/// Represents the possible errors that can occur when applying macOS settings.
#[derive(Debug, Error)]
pub enum MacOSError {
    #[error("Failed to read setting")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse setting")]
    ParseError,
    #[error("Failed to write setting")]
    WriteError,
}

/// Applies the Dock settings.
pub fn apply_dock_settings(dock: &Dock) -> Result<bool, DefaultsError> {
    let mut changed = false;

    if let Some(orientation) = dock.orientation {
        changed |= write_defaults("com.apple.dock", "orientation", orientation)?;
    }

    if let Some(autohide) = dock.autohide {
        changed |= write_defaults("com.apple.dock", "autohide", autohide)?;
    }

    if let Some(icon_size) = dock.icon_size {
        changed |= write_defaults("com.apple.dock", "tilesize", icon_size)?;
    }

    if let Some(showhidden) = dock.transparent_hidden_app_icons {
        changed |= write_defaults("com.apple.dock", "showhidden", showhidden)?;
    }

    Ok(changed)
}

/// Applies the Mission Control settings.
pub fn apply_mission_control_settings(
    mission_control: &MissionControl,
) -> Result<bool, DefaultsError> {
    let mut changed = false;

    if let Some(rearrange) = mission_control.automatically_rearrange_spaces {
        changed |= write_defaults("com.apple.dock", "mru-spaces", rearrange)?;
    }

    if let Some(group_apps) = mission_control.group_apps {
        changed |= write_defaults("com.apple.dock", "expose-group-apps", group_apps)?;
    }

    Ok(changed)
}

/// Restarts the Dock.
pub fn restart_dock() -> Result<(), DefaultsError> {
    println!("Restarting Dock to apply changes...");
    Command::new("killall")
        .arg("Dock")
        .status()
        .map_err(|e| DefaultsError::CommandFailed(format!("failed to kill Dock {e}")))?;
    Ok(())
}

/// Applies the Safari settings.
pub fn apply_safari_settings(safari: &Safari) -> Result<(), DefaultsError> {
    let mut changed = false;

    if let Some(show_full_url) = safari.show_full_url {
        changed |= write_defaults(
            "com.apple.Safari",
            "ShowFullURLInSmartSearchField",
            show_full_url,
        )?;
    }

    if changed {
        println!("Restarting Safari to apply changes...");
        Command::new("killall")
            .arg("Safari")
            .status()
            .map_err(|e| DefaultsError::CommandFailed(format!("failed to kill Safari {e}")))?;
    }

    Ok(())
}

/// Applies the system-wide settings.
pub fn apply_system_settings(system: &SystemSettings) -> Result<(), DefaultsError> {
    let mut changed = false;

    if let Some(show_file_extensions) = system.show_file_extensions {
        changed |= write_defaults(
            "NSGlobalDomain",
            "AppleShowAllExtensions",
            show_file_extensions,
        )?;
    }

    if let Some(natural_scrolling) = system.natural_scrolling {
        write_defaults(
            "NSGlobalDomain",
            "com.apple.swipescrolldirection",
            natural_scrolling,
        )?;
        // System restart required. TODO: somehow signify that this needs to happen in the output.
    }

    if changed {
        println!("Restarting Finder to apply changes...");
        Command::new("killall")
            .arg("Finder")
            .status()
            .map_err(|e| DefaultsError::CommandFailed(format!("failed to kill Finder {e}")))?;
    }

    Ok(())
}
