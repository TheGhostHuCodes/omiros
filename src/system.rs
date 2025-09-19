use serde::Deserialize;

use crate::{
    brew::Brew,
    dotfiles::Dotfiles,
    macos::{Dock, Safari, SystemSettings},
    mas::Mas,
    vscode::Vscode,
};

/// Represents the entire system configuration, including all package managers,
/// and dotfiles.
#[derive(Deserialize, Debug)]
pub struct System {
    /// The Homebrew configuration.
    pub brew: Brew,
    /// The Mac App Store configuration.
    pub mas: Mas,
    /// The Dotfiles configuration.
    pub dotfiles: Option<Dotfiles>,
    /// The VS Code configuration.
    pub vscode: Option<Vscode>,
    /// The macOS configuration.
    pub macos: Option<MacOS>,
}

#[derive(Deserialize, Debug)]
pub struct MacOS {
    pub dock: Option<Dock>,
    pub safari: Option<Safari>,
    pub system: Option<SystemSettings>,
}
