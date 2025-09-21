use serde::Deserialize;

use crate::{
    brew::Brew,
    dotfiles::Dotfiles,
    macos::{Dock, Finder, MagicMouse, MissionControl, Safari, SystemSettings},
    mas::Mas,
    vscode::Vscode,
};

/// Represents the entire system configuration, including all package managers,
/// and dotfiles.
#[derive(Deserialize, Debug)]
pub struct System {
    /// The Homebrew configuration.
    pub brew: Option<Brew>,
    /// The Mac App Store configuration.
    pub mas: Option<Mas>,
    /// The Dotfiles configuration.
    pub dotfiles: Option<Dotfiles>,
    /// The VS Code configuration.
    pub vscode: Option<Vscode>,
    /// The macOS configuration.
    pub macos: Option<MacOS>,
}

/// Represents all macOS-specific configuration.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct MacOS {
    pub dock: Option<Dock>,
    pub safari: Option<Safari>,
    pub system: Option<SystemSettings>,
    pub mission_control: Option<MissionControl>,
    pub magic_mouse: Option<MagicMouse>,
    pub finder: Option<Finder>,
}
