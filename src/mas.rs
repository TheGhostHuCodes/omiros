use pest::Parser;
use pest_derive::Parser;
use serde::Deserialize;

use std::{collections::HashSet, process::Command, str::from_utf8};

use crate::{errors::SetupError, system_utils::command};

const MAS_PROGRAM_NAME: &str = "mas";

/// Represents the Mac App Store configuration, specifying which apps to install.
#[derive(Deserialize, Debug)]
pub struct Mas {
    /// The list of apps to install.
    pub apps: Vec<App>,
}

/// Represents a single Mac App Store application.
#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct App {
    /// The name of the app.
    pub name: String,
    /// The ID of the app in the Mac App Store.
    pub id: String,
}

/// Represents the set of currently installed Mac App Store apps.
#[derive(Debug)]
pub struct InstalledMasApps {
    /// The set of installed apps.
    pub apps: HashSet<App>,
}

/// Represents the set of missing Mac App Store apps that need to be installed.
#[derive(Debug)]
pub struct MissingMasApps<'a> {
    /// The list of missing apps.
    pub apps: Vec<&'a App>,
}

/// Checks if `mas` is installed and available in the system's PATH.
pub fn check_mas_installed() -> Result<(), SetupError> {
    let _ = command(MAS_PROGRAM_NAME)?;

    Ok(())
}

/// Retrieves the list of currently installed Mac App Store apps.
pub fn get_installed_apps() -> anyhow::Result<InstalledMasApps> {
    let mas_output = Command::new("mas").args(["list"]).output()?;

    let apps = from_utf8(&mas_output.stdout)?
        .lines()
        .map(parse_mas_list_record)
        .collect();

    Ok(InstalledMasApps { apps })
}

fn parse_mas_list_record(record: &str) -> App {
    let record = record.trim();
    let record = MasListParser::parse(Rule::record, record)
        .expect("unsuccessful mas list parse")
        .next()
        .unwrap();

    let mut id: String = Default::default();
    let mut name: String = Default::default();

    for field in record.into_inner() {
        match field.as_rule() {
            Rule::app_id => id = field.as_str().to_string(),
            Rule::app_name => name = field.as_str().trim().to_string(),
            // We're ignoring the app_version for now, even though we parse it.
            Rule::app_version => (),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    App { id, name }
}

#[derive(Parser)]
#[grammar = "grammars/mas_list.pest"]
pub struct MasListParser;

/// Compares the desired Mac App Store apps with the installed apps to determine which ones are missing.
pub fn find_missing_apps<'a>(desired: &'a Mas, installed: &InstalledMasApps) -> MissingMasApps<'a> {
    let mut missing = MissingMasApps { apps: Vec::new() };

    for app in &desired.apps {
        if !installed.apps.contains(app) {
            missing.apps.push(app);
        }
    }

    missing
}

/// Installs the missing Mac App Store apps.
pub fn install_missing_apps(missing: &MissingMasApps) -> Result<(), SetupError> {
    for app in &missing.apps {
        println!("Installing app: {}", app.name);
        let status = Command::new("mas").args(["install", &app.id]).status()?;
        if !status.success() {
            return Err(SetupError::MasInstallFailed);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn parse_mas_list_record_parses_single_word_app_name_correctly() {
        let input = "937984704   Amphetamine  (5.3.2)";
        let expected = App {
            name: "Amphetamine".to_string(),
            id: "937984704".to_string(),
        };
        let actual = parse_mas_list_record(input);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(
        "946798523  Sleep Control Centre            (2.27)",
        App {
            name: "Sleep Control Centre".to_string(),
            id: "946798523".to_string(),
        }
    )]
    #[case(
        "1352211125  Tide Alert (NOAA) - Tide Chart  (3.2)",
        App {
            name: "Tide Alert (NOAA) - Tide Chart".to_string(),
            id: "1352211125".to_string(),
        }
    )]
    #[case(
        "  1491074310  Tetris®                         (7.3.3)  ",
        App {
            name: "Tetris®".to_string(),
            id: "1491074310".to_string(),
        }
    )]
    #[case(
        "   381471023  Flashlight Ⓞ                    (2.3.5) ",
        App {
            name: "Flashlight Ⓞ".to_string(),
            id: "381471023".to_string(),
        }
    )]
    #[case(
        "   890378044  Toy Blast                       (21004) ",
        App {
            name: "Toy Blast".to_string(),
            id: "890378044".to_string(),
        }
    )]
    fn parse_mas_list_record_parses_app_name_correctly(#[case] input: &str, #[case] expected: App) {
        let actual = parse_mas_list_record(input);

        assert_eq!(expected, actual);
    }
}
