use std::{fs, io, path::PathBuf};

use clap::{CommandFactory, Parser};
use clap_complete::{Shell, generate};

use omiros::{
    brew::{
        check_brew_installed, find_missing_packages, get_installed_brew_packages,
        install_missing_packages,
    },
    dotfiles::setup_dotfiles,
    macos,
    mas::{check_mas_installed, find_missing_apps, get_installed_apps, install_missing_apps},
    system::System,
};

/// A home manager for normies.
#[derive(Parser)]
#[command(name = "omiros", version, about, long_about = None)]
enum Cli {
    /// Run system synchronization operation
    Run {
        /// Path to the directory containing the system.toml file.
        #[arg(short, long)]
        system_config_dir: PathBuf,
        /// Path to the dotfiles directory.
        #[arg(short, long)]
        dotfiles_dir: PathBuf,
    },
    /// Generate shell completions
    Completions {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Completions { shell } => {
            // This is needed by the generator below to walk over the CLI spec
            // so that it can emit completions.
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "omiros", &mut io::stdout());
        }
        Cli::Run {
            system_config_dir,
            dotfiles_dir,
        } => {
            let system_config_path = system_config_dir.join("system.toml");
            let system_config = fs::read_to_string(system_config_path)?;
            let system: System = toml::from_str(&system_config)?;

            // TODO: There's a chicken and egg problem here, some shell installers
            // require curl or wget, or some other tooling, but at least for brew, we'll
            // need to install that first before we have a macOS package manager. We
            // might have to special-case the installation of brew first if requested
            // for install.
            if let Some(shell_installers) = system.shell_installers {
                for installer in shell_installers.install {
                    installer.install()?;
                }
            } else {
                println!("ℹ️  No `[shell-installers]` block in configuration file");
            }

            if let Some(brew) = system.brew {
                check_brew_installed()?;
                let installed_packages = get_installed_brew_packages()?;
                let missing_packages = find_missing_packages(&brew, &installed_packages);
                install_missing_packages(&missing_packages)?;
            } else {
                println!("ℹ️  No `[brew]` block in configuration file");
            }

            if let Some(mas) = system.mas {
                check_mas_installed()?;
                let installed_apps = get_installed_apps()?;
                let missing_apps = find_missing_apps(&mas, &installed_apps);
                install_missing_apps(&missing_apps)?;
            } else {
                println!("ℹ️  No `[mas]` block in configuration file");
            }

            if let Some(dotfiles) = system.dotfiles {
                setup_dotfiles(&dotfiles, &dotfiles_dir.canonicalize()?)?;
            } else {
                println!("ℹ️  No `[dotfiles]` block in configuration file");
            }

            if let Some(vscode) = system.vscode {
                vscode.install_missing_extensions()?;
            } else {
                println!("ℹ️  No `[vscode]` block in configuration file");
            }

            if let Some(macos) = system.macos {
                let mut dock_changed = false;
                if let Some(dock) = &macos.dock {
                    dock_changed |= macos::apply_dock_settings(dock)?;
                }
                if let Some(mission_control) = &macos.mission_control {
                    dock_changed |= macos::apply_mission_control_settings(mission_control)?;
                }

                if dock_changed {
                    macos::restart_dock()?;
                }

                if let Some(safari) = macos.safari {
                    macos::apply_safari_settings(&safari)?;
                }
                if let Some(system) = macos.system {
                    macos::apply_system_settings(&system)?;
                }
                if let Some(magic_mouse) = macos.magic_mouse {
                    macos::apply_magic_mouse_settings(&magic_mouse)?;
                }
                if let Some(finder) = macos.finder {
                    macos::apply_finder_settings(&finder)?;
                }
            } else {
                println!("ℹ️  No `[macos]` block in configuration file");
            }
        }
    }
    Ok(())
}
