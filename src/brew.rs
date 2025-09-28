use std::{collections::HashSet, process::Command, str::from_utf8};

use serde::Deserialize;

use crate::{errors::SetupError, system_utils::command};

const BREW_PROGRAM_NAME: &str = "brew";

/// Represents the Homebrew configuration, specifying which formulae and casks to install.
#[derive(Deserialize, Debug)]
pub struct Brew {
    formulae: Option<Vec<String>>,
    casks: Option<Vec<String>>,
}

/// Represents the set of currently installed Homebrew packages.
#[derive(Debug)]
pub struct InstalledBrewPackages {
    formulae: HashSet<String>,
    casks: HashSet<String>,
}

/// Represents the set of missing Homebrew packages that need to be installed.
#[derive(Debug)]
pub struct MissingBrewPackages<'a> {
    /// The list of missing formulae.
    pub formulae: Vec<&'a str>,
    /// The list of missing casks.
    pub casks: Vec<&'a str>,
}

/// Compares the desired Homebrew packages with the installed packages to determine which ones are missing.
pub fn find_missing_packages<'a>(
    desired: &'a Brew,
    installed: &InstalledBrewPackages,
) -> MissingBrewPackages<'a> {
    let mut missing = MissingBrewPackages {
        formulae: Vec::new(),
        casks: Vec::new(),
    };

    if let Some(formulae) = &desired.formulae {
        for formula in formulae {
            if !installed.formulae.contains(formula) {
                missing.formulae.push(formula);
            }
        }
    }

    if let Some(casks) = &desired.casks {
        for cask in casks {
            if !installed.casks.contains(cask) {
                missing.casks.push(cask);
            }
        }
    }

    missing
}

/// Retrieves the list of currently installed Homebrew packages.
pub fn get_installed_brew_packages() -> Result<InstalledBrewPackages, SetupError> {
    let formulae_output = Command::new(BREW_PROGRAM_NAME).args(["leaves"]).output()?;
    let formulae = from_utf8(&formulae_output.stdout)?
        .lines()
        .map(String::from)
        .collect();

    let casks_output = Command::new(BREW_PROGRAM_NAME)
        .args(["list", "--casks"])
        .output()?;
    let casks = from_utf8(&casks_output.stdout)?
        .lines()
        .map(String::from)
        .collect();

    Ok(InstalledBrewPackages { formulae, casks })
}

/// Checks if Homebrew is installed and available in the system's PATH.
pub fn check_brew_installed() -> Result<(), SetupError> {
    command(BREW_PROGRAM_NAME).map_err(|_| SetupError::BrewNotFound)?;
    Ok(())
}

/// Installs the missing Homebrew packages.
pub fn install_missing_packages(missing: &MissingBrewPackages) -> Result<(), SetupError> {
    for formula in &missing.formulae {
        println!("Installing formula: {formula}");
        let status = Command::new("brew").args(["install", formula]).status()?;
        if !status.success() {
            return Err(SetupError::BrewInstallFailed);
        }
    }

    for cask in &missing.casks {
        println!("Installing cask: {cask}");
        let status = Command::new("brew")
            .args(["install", "--cask", cask])
            .status()?;
        if !status.success() {
            return Err(SetupError::BrewInstallFailed);
        }
    }

    Ok(())
}
