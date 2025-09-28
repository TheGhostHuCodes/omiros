use serde::Deserialize;

pub mod rustup;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ShellInstaller {
    Rustup,
}

impl ShellInstaller {
    pub fn install(&self) -> anyhow::Result<()> {
        match self {
            ShellInstaller::Rustup => Ok(rustup::install_rustup()?),
        }
    }
}
