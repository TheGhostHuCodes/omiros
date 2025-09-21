use serde::Deserialize;
use std::process::Command;
use thiserror::Error;

use crate::defaults::{DefaultsError, DockOrientation, MouseButtonMode, write_defaults};

/// Represents the Dock configuration.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Dock {
    pub orientation: Option<DockOrientation>,
    pub autohide: Option<bool>,
    pub icon_size: Option<i32>,
    pub transparent_hidden_app_icons: Option<bool>,
}

/// Represents the Mission Control configuration.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MissionControl {
    pub automatically_rearrange_spaces: Option<bool>,
    pub group_apps: Option<bool>,
}

/// Represents the Safari configuration.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Safari {
    pub show_full_url: Option<bool>,
}

/// System-wide configuration.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemSettings {
    pub show_file_extensions: Option<bool>,
    /// Never have I experienced a more unnatural scrolling direction as Apple's
    /// "natural" scrolling direction.
    pub natural_scrolling: Option<bool>,
    /// Apple Press&Hold allows you to select alternative characters on long
    /// presses. I've never used this feature, and it causes issues with vim
    /// navigation.
    pub key_press_and_hold: Option<bool>,
    /// Delay before repetition starts. Lower value means shorter wait time
    /// before repeat starts.
    pub initial_key_repeat_wait: Option<i32>,
    /// Rate at which keys are repeated once repetition starts. Lower value
    /// means faster rate... for some reason.
    pub key_repeat_rate: Option<i32>,
    /// Automatically capitalizes the first letter of a new sentence and proper
    /// nouns as you type. How annoying.
    pub automatic_capitalization: Option<bool>,
}

/// Magic Mouse configuration.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MagicMouse {
    pub mouse_button_mode: Option<MouseButtonMode>,
}

/// Finder configuration.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Finder {
    /// Display directory breadcrumbs at the bottom of the finder window.
    pub show_pathbar: Option<bool>,
    pub show_full_posix_path_in_title_bar: Option<bool>,
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

    // TODO: we might want to move this over to the finder section, even though
    // this is a global configuration, because it mainly affects Finder.
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
        // Logout, login, or System restart required. TODO: somehow signify that this needs to happen in the output.
    }

    if let Some(key_press_and_hold) = system.key_press_and_hold {
        write_defaults(
            "NSGlobalDomain",
            "ApplePressAndHoldEnabled",
            key_press_and_hold,
        )?;
        // Logout, login, or System restart required. TODO: somehow signify that this needs to happen in the output.
    }

    if let Some(initial_key_repeat_wait) = system.initial_key_repeat_wait {
        write_defaults(
            "NSGlobalDomain",
            "InitialKeyRepeat",
            initial_key_repeat_wait,
        )?;
        // Logout, login, or System restart required. TODO: somehow signify that this needs to happen in the output.
    }

    if let Some(key_repeat_rate) = system.key_repeat_rate {
        write_defaults("NSGlobalDomain", "KeyRepeat", key_repeat_rate)?;
        // Logout, login, or System restart required. TODO: somehow signify that this needs to happen in the output.
    }

    if let Some(automatic_capitalization) = system.automatic_capitalization {
        write_defaults(
            "NSGlobalDomain",
            "NSAutomaticCapitalizationEnabled",
            automatic_capitalization,
        )?;
        // No logout or restart needed, update happens immediately.
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

pub fn apply_magic_mouse_settings(magic_mouse: &MagicMouse) -> Result<(), DefaultsError> {
    if let Some(mouse_button_mode) = magic_mouse.mouse_button_mode {
        write_defaults(
            "com.apple.AppleMultitouchMouse",
            "MouseButtonMode",
            mouse_button_mode,
        )?;
    }

    Ok(())
}

pub fn apply_finder_settings(finder: &Finder) -> Result<(), DefaultsError> {
    let mut changed = false;

    if let Some(show_pathbar) = finder.show_pathbar {
        changed |= write_defaults("com.apple.finder", "ShowPathbar", show_pathbar)?;
    }

    if let Some(show_full_posix_path_in_title_bar) = finder.show_full_posix_path_in_title_bar {
        changed |= write_defaults(
            "com.apple.finder",
            "_FXShowPosixPathInTitle",
            show_full_posix_path_in_title_bar,
        )?;
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
