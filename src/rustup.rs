use std::process::Command;

use crate::{errors::SetupError, system_utils::command};

pub fn install_rustup() -> Result<(), SetupError> {
    println!("🦀 Installing rustup...");
    let rustup_path = command("rustup")?;

    if rustup_path.exists() {
        println!(
            "ℹ️  rustup is already installed at: {}",
            rustup_path.display()
        );
        return Ok(());
    }

    // Download and execute the rustup installer.
    // curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    let status = Command::new("curl")
        .args([
            "--proto",
            "'=https'",
            "--tlsv1.2",
            "-sSf",
            "https://sh.rustup.rs",
            "|",
            "sh",
            "-s",
            "--",
            "-y",
        ])
        .status()?;

    if status.success() {
        println!("✅ rustup installed successfully");
        println!("💡 You may need to restart your shell or run: source ~/.cargo/env");
        Ok(())
    } else {
        Err(SetupError::InstallFailed(
            "rustup installation failed".to_string(),
        ))
    }
}
