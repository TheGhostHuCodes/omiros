/// Represents the possible errors that can occur during the setup process.
#[derive(Debug, thiserror::Error)]
pub enum SetupError {
    /// Indicates that Homebrew is not installed or not found in the system's PATH.
    #[error("Brew not found")]
    BrewNotFound,
    /// Indicates that a required program is not installed or not found in the system's PATH.
    #[error("Program not found")]
    ProgramFileNotFound(String),
    /// Indicates that a Homebrew package installation failed.
    #[error("Failed to install brew package")]
    BrewInstallFailed,
    /// Indicates that a Mac App Store package installation failed.
    #[error("Failed to install mas package")]
    MasInstallFailed,
    /// Generic installation failed.
    #[error("Installation failed: {0}")]
    InstallFailed(String),
    /// Generic error setting up Dotfiles.
    #[error("Error setting up dotfiles")]
    DotfileError(String),
    /// IO error.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    /// Toml deserialization error.
    #[error("TOML parse error: {0}")]
    TomlError(#[from] toml::de::Error),
    /// utf-8 error.
    #[error("From UTF-8 error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    /// Infallible error, should never happen.
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] core::str::Utf8Error),
    /// Infallible error, should never happen.
    #[error("Infallible error: {0}")]
    Infallible(#[from] std::convert::Infallible),
}
