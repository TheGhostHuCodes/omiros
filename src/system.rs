use serde::Deserialize;

use crate::{brew::Brew, dotfiles::Dotfiles, mas::Mas, vscode::Vscode};

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
}
