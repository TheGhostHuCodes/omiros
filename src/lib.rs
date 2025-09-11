//! The main library crate for the environment setup tool.
//!
//! This crate contains the core logic for checking and installing packages from
//! various package managers.

/// Contains the logic for interacting with Homebrew.
pub mod brew;
/// Contains the logic for working with dotfiles.
pub mod dotfiles;
/// Defines the custom error types for the application.
pub mod errors;
/// Contains the logic for interacting with the Mac App Store commandline tool.
pub mod mas;
/// Logic for setting up `rustup`.
pub mod rustup;
/// Defines the data structures for the system configuration file.
pub mod system;
/// Contains utility functions for interacting with the system.
mod system_utils;
/// Contains logic for interacting with vscode extensions through the `code`
/// commandline tool.
pub mod vscode;
