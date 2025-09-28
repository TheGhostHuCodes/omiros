use serde::Deserialize;

use crate::{
    brew::Brew,
    dotfiles::Dotfiles,
    macos::{Dock, Finder, MagicMouse, MissionControl, Safari, SystemSettings},
    mas::Mas,
    shell_installers::ShellInstaller,
    vscode::Vscode,
};

/// Represents the entire system configuration, including all package managers,
/// and dotfiles.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
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
    /// The shell installers configuration.
    pub shell_installers: Option<ShellInstallers>,
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

/// Represents all shell installers.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ShellInstallers {
    pub install: Vec<ShellInstaller>,
}
