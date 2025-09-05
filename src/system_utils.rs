use std::{path::PathBuf, process::Command, str::FromStr};

use crate::errors::SetupError;

/// Checks if a program is installed and in the PATH.
pub(crate) fn command(program: &str) -> Result<PathBuf, SetupError> {
    let output = Command::new("command").args(["-v", program]).output()?;

    if output.status.success() {
        println!("✅ {program} found");
        let path = String::from_utf8(output.stdout)?;

        Ok(PathBuf::from_str(path.trim())?)
    } else {
        Err(SetupError::ProgramFileNotFound(program.to_string()))
    }
}
