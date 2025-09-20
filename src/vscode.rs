use serde::Deserialize;

use std::{collections::HashSet, ops::Deref, process::Command};

use crate::{errors::SetupError, system_utils::command};

/// Represents the VS Code configuration, specifying which extensions to
/// install.
#[derive(Deserialize, Debug)]
pub struct Vscode {
    pub extensions: Vec<ExtensionIdentifier>,
}

/// A VSCode extension unique identifier. Has the form `{publisher}.{name}``,
/// but we don't bother parsing it, just passing it directly to the `code`
/// commandline for installation.
#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct ExtensionIdentifier(String);

impl ExtensionIdentifier {
    fn to_lowercase(&self) -> Self {
        ExtensionIdentifier(self.as_str().to_lowercase())
    }
}

impl Deref for ExtensionIdentifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Vscode {
    pub fn install_missing_extensions(&self) -> Result<(), SetupError> {
        command("code")?;

        println!("Checking VS Code extensions...");
        let installed_extensions = get_installed_extensions()?;
        let missing_extensions = self
            .extensions
            .iter()
            .filter(|&e| !installed_extensions.contains(&e.to_lowercase()))
            .collect::<Vec<_>>();

        if missing_extensions.is_empty() {
            println!("All VS Code extensions are installed.");
        } else {
            println!("Installing missing VS Code extensions...");
            for extension in missing_extensions {
                println!("Installing vscode extension: {extension:?}");
                let status = Command::new("code")
                    .args(["--install-extension", extension])
                    .status()?;
                if !status.success() {
                    return Err(SetupError::InstallFailed(format!(
                        "vscode extension install failed: {extension:?}"
                    )));
                }
            }
        }

        Ok(())
    }
}

/// Gets all installed VSCode extensions. Note VSCode extension identifiers are
/// case sensitive IDs. However, using the command line to get a list of these
/// identifiers returns all lower-case list of extension identifiers.
fn get_installed_extensions() -> Result<HashSet<ExtensionIdentifier>, SetupError> {
    let output = Command::new("code").arg("--list-extensions").output()?;
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        let extensions = stdout
            .lines()
            .map(|extension| ExtensionIdentifier(extension.trim().to_string()))
            .collect();
        Ok(extensions)
    } else {
        Err(SetupError::InstallFailed(format!(
            "Failed to get installed VS Code extensions: {}",
            String::from_utf8(output.stderr)?
        )))
    }
}
