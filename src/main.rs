use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

// --------------------------------------------------

use anyhow::{Context, Result};
use clap::Parser;
use once_cell::sync::Lazy;
use regex::Regex;

// --------------------------------------------------

// Lazily initialized regular expression that matches one or more whitespace characters.
static RE_WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());

// --------------------------------------------------

/// CLI tool to rename all files and folders in a given directory to snake_case.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the directory to rename folders and files.
    path: PathBuf,

    /// Dry run mode: display changes without applying them to the filesystem.
    #[arg(short, long)]
    dry_run: bool,

    /// Include hidden files and directories.
    #[arg(long, default_value_t = false)]
    include_hidden: bool,

    /// Verbose mode: show detailed renaming information.
    #[arg(short, long)]
    verbose: bool,
}

// --------------------------------------------------

// Determines whether a file is hidden.
fn is_hidden(file_name: &OsStr) -> bool {
    file_name.to_str().is_some_and(|name| name.starts_with('.'))
}

// --------------------------------------------------

// Renames the provided file/folder.
fn rename_file_or_folder(
    path: &Path,
    original_name: &str,
    new_name: &str,
    args: &Args,
) -> Result<()> {
    let parent_dir = path.parent().context("Failed to get parent directory")?;
    let new_path = parent_dir.join(new_name);

    if args.dry_run {
        println!("[Dry Run] Would rename '{original_name}' to '{new_name}'");
    } else {
        fs::rename(path, &new_path)
            .with_context(|| format!("Failed to rename '{:?}' to '{:?}'", path, new_path))?;
        if args.verbose {
            println!("Renamed '{original_name}' to '{new_name}'")
        }
    }
    Ok(())
}

// --------------------------------------------------

fn to_snake_case(name: &str) -> String {
    RE_WHITESPACE.replace_all(name.trim(), "_").to_lowercase()
}

// --------------------------------------------------

fn process_dir(path: &Path, args: &Args) -> Result<()> {
    let entries =
        fs::read_dir(path).with_context(|| format!("Failed to read directory {:?}", path))?;

    for entry in entries {
        let entry = entry.with_context(|| format!("Failed to access entry in {:?}", path))?;
        let path = entry.path();
        let file_name = path
            .file_name()
            .context(format!("Failed to get file name from {:?}", path))?;

        // Skip hidden files when applicable
        if !args.include_hidden && is_hidden(file_name) {
            continue;
        }

        // Skip symlinks
        let metadata = fs::symlink_metadata(&path)
            .with_context(|| format!("Failed to get metadata for: {:?}", path))?;
        if metadata.file_type().is_symlink() {
            if args.verbose {
                println!("Skipping symbolic link: {:?}", path);
            }
            continue;
        }

        if path.is_dir() {
            process_dir(&path, args)?;
        }

        let original_name = file_name
            .to_str()
            .context(format!("Failed to call to_str on {:?}", file_name))?;
        let new_name = to_snake_case(original_name);

        if original_name != new_name {
            rename_file_or_folder(&path, original_name, &new_name, args)?;
        }
    }

    Ok(())
}

// --------------------------------------------------

fn main() -> Result<()> {
    let args = Args::parse();
    let path = &args.path;

    if !path.exists() {
        anyhow::bail!("Error: Path {:?} does not exist", path);
    }

    // Do not process symlinks
    let metadata = fs::symlink_metadata(path)
        .with_context(|| format!("Failed to get metadata for: {:?}", path))?;
    if !metadata.is_dir() {
        anyhow::bail!("Error: Path {:?} is not a directory", path);
    }

    process_dir(path, &args)?;

    Ok(())
}
