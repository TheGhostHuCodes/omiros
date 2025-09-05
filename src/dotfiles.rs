use std::{
    env::home_dir,
    fs,
    path::{Component, Path, PathBuf},
};

use serde::Deserialize;

use crate::errors::SetupError;

#[derive(Deserialize, Debug)]
pub struct Dotfiles {
    files: Vec<DotfileEntry>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum DotfileEntry {
    Implicit(PathBuf),
    Explicit { original: PathBuf, link: PathBuf },
}

/// Takes a path, if it stats with `~/`, expand the home path by prepending the
/// home path and removing the tilde. Effectively expanding the tilde path to
/// home. This is usually done by the shell, but here we have to do it by hand
/// because there is no shell to do the expansion.
fn tilde_expand_path(path: &Path, home: &Path) -> Result<PathBuf, SetupError> {
    let expanded = if path.starts_with("~/") {
        path.components()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    Component::Normal(home.as_os_str())
                } else {
                    c
                }
            })
            .collect()
    } else {
        PathBuf::from(path)
    };

    Ok(expanded)
}

pub fn setup_dotfiles(dotfiles: &Dotfiles, dotfiles_dir: &Path) -> Result<(), SetupError> {
    println!("🔗 Setting up dotfiles...");

    if !dotfiles_dir.exists() {
        return Err(SetupError::DotfileError(format!(
            "Dotfiles directory not found: {}",
            dotfiles_dir.display()
        )));
    }

    let home = home_dir().ok_or_else(|| {
        SetupError::DotfileError("Could not determine home directory.".to_string())
    })?;

    for entry in &dotfiles.files {
        let (original, link) = match entry {
            DotfileEntry::Implicit(path_buf) => {
                let original = dotfiles_dir.join(path_buf);
                let link = home.join(path_buf);
                (original, link)
            }
            DotfileEntry::Explicit { original, link } => {
                let original = dotfiles_dir.join(original);
                let link = tilde_expand_path(link, &home)?;
                (original, link)
            }
        };

        // Verify original file exists
        if !original.exists() {
            return Err(SetupError::DotfileError(format!(
                "Original dotfile not found: {}",
                original.display()
            )));
        }

        // Create parent directory if it doesn't exist
        if let Some(link_parent) = link.parent()
            && !link_parent.exists()
        {
            fs::create_dir_all(link_parent)?;
            println!("📁 Created directory: {}", link_parent.display());
        }

        // Check what exists at the link location.
        match fs::symlink_metadata(&link) {
            Ok(metadata) => {
                if metadata.is_symlink() {
                    // It's a symlink, check if it points to the correct location
                    match fs::read_link(&link) {
                        Ok(link_target) if link_target == original => {
                            println!("✅ {} already correctly linked", link.display());
                            continue;
                        }
                        Ok(_) => {
                            // It's a symlink, but it points to the wrong place
                            fs::remove_file(&link)?;
                            println!("🔄 Removed incorrect symlink: {}", link.display());
                        }
                        Err(_) => {
                            // It's a broken symlink
                            fs::remove_file(&link)?;
                            println!("🗑️  Removed broken symlink: {}", link.display());
                        }
                    }
                } else {
                    // It's a regular file or directory - error out and have the user
                    // manually remove it.
                    return Err(SetupError::DotfileError(format!(
                        "Link path already exists as a file/directory:{}\n\
                            Please manually backup and remove this file before running omiros again.",
                        link.display()
                    )));
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // The link does not exist, which is what we want.
            }
            Err(e) => {
                // Some other error, bubble it up.
                return Err(SetupError::IoError(e));
            }
        }

        // Create symlink
        std::os::unix::fs::symlink(&original, &link)?;
        println!("🔗 Linked {} -> {}", link.display(), original.display());
    }

    println!("✅ Dotfiles setup complete");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn tilde_expand_path_works() {
        let home = Path::new("/User/me/");
        let path = Path::new("~/.config/thing");

        let x = tilde_expand_path(path, home).unwrap();

        assert_eq!(PathBuf::from_str("/User/me/.config/thing").unwrap(), x)
    }
}
