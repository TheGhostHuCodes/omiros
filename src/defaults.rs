use std::{
    fmt::Display,
    process::{Command, Stdio},
    str,
};

use serde::Deserialize;

pub(crate) trait DefaultsType: Sized {
    /// The type flag used when writing values to the `defaults` command. For
    /// example, booleans are written with `-bool`
    const TYPE_FLAG: &'static str;

    /// Parses the output from the `defaults` command, and returns back a
    /// instance of Self.
    fn parse_output(s: &str) -> Result<Self, DefaultsError>;
}

impl DefaultsType for bool {
    const TYPE_FLAG: &'static str = "-bool";

    fn parse_output(s: &str) -> Result<Self, DefaultsError> {
        match s {
            "0" | "false" => Ok(false),
            "1" | "true" => Ok(true),
            s => Err(DefaultsError::ParseError(format!(
                "Unable to parse {s} as {}",
                Self::TYPE_FLAG
            ))),
        }
    }
}

impl DefaultsType for i32 {
    const TYPE_FLAG: &'static str = "-int";

    fn parse_output(s: &str) -> Result<Self, DefaultsError> {
        s.parse::<i32>()
            .map_err(|_| DefaultsError::ParseError(format!("Could not parse: {s}")))
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum DockOrientation {
    Left,
    Bottom,
    Right,
}

impl std::fmt::Display for DockOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DockOrientation::Left => write!(f, "left"),
            DockOrientation::Bottom => write!(f, "bottom"),
            DockOrientation::Right => write!(f, "right"),
        }
    }
}

impl DefaultsType for DockOrientation {
    const TYPE_FLAG: &'static str = "-string";

    fn parse_output(s: &str) -> Result<Self, DefaultsError> {
        match s {
            "left" => Ok(DockOrientation::Left),
            "bottom" => Ok(DockOrientation::Bottom),
            "right" => Ok(DockOrientation::Right),
            s => Err(DefaultsError::ParseError(format!(
                "Could not parse output: {s}"
            ))),
        }
    }
}

/// Reads the configuration value stored by macOS by using the `defaults` CLI
/// for particular `domain` and `key`.
fn read_defaults<T>(domain: &str, key: &str) -> Result<T, DefaultsError>
where
    T: DefaultsType,
{
    let output = Command::new("defaults")
        .args(["read", domain, key])
        .output()
        .map_err(|e| {
            DefaultsError::CommandFailed(format!("Failed to execute defaults write: {}", e))
        })?;

    if !output.status.success() {
        return Err(DefaultsError::CommandFailed("sadness".to_string()));
    }

    let s = str::from_utf8(output.stdout.trim_ascii())?;

    T::parse_output(s)
}

/// returns a bool telling you if a change had to occur, or if the setting was
/// already the same as the given `value`, this lets you do things like add a
/// follow-on step such as restarting the application that this setting affects.
pub(crate) fn write_defaults<T>(
    domain: &str,
    key: &str,
    new_value: T,
) -> Result<bool, DefaultsError>
where
    T: Display + DefaultsType + PartialEq,
{
    match read_defaults::<T>(domain, key) {
        Ok(current_value) => {
            if current_value == new_value {
                println!("ℹ️  {}.{} already set to {}", domain, key, new_value);
                return Ok(false);
            }
        }
        Err(_) => todo!(),
    }

    println!(
        "🔧 Setting {}.{} = {} ({})",
        domain,
        key,
        new_value,
        T::TYPE_FLAG
    );

    let status = Command::new("defaults")
        .args(["write", domain, key, T::TYPE_FLAG, &new_value.to_string()])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()
        .map_err(|e| {
            DefaultsError::CommandFailed(format!("Failed to execute defaults write: {}", e))
        })?;

    if !status.success() {
        return Err(DefaultsError::CommandFailed(format!(
            "defaults write failed for {}.{}",
            domain, key
        )));
    }

    Ok(true)
}

#[derive(Debug, thiserror::Error)]
pub enum DefaultsError {
    /// `default` command failed.
    #[error("Defaults command failed {0}")]
    CommandFailed(String),
    #[error("Defaults output parsing failed {0}")]
    ParseError(String),
    /// Error when converting a &[u8] to a utf-8 &str
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] core::str::Utf8Error),
}
