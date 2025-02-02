use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

// --------------------------------------------------

use anyhow::{Context, Result};
use clap::Parser;
use heck::ToSnakeCase;

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
// TODO: On Windows, hidden files might be determined by file attributes.
fn is_hidden(file_name: &OsStr) -> bool {
    file_name.to_str().is_some_and(|name| name.starts_with('.'))
}

// --------------------------------------------------

// Renames the provided file/folder.
fn rename_file_or_folder(
    path: &Path,
    original_name: &str,
    hidden: bool,
    args: &Args,
) -> Result<()> {
    let parent_dir = path
        .parent()
        .context(format!("Failed to get parent directory of {:?}", path))?;

    let original_path = Path::new(original_name);
    let original_file_stem = original_path.file_stem().context(format!(
        "Failed to extract the stem from {:?}",
        original_name
    ))?;
    let original_ext = original_path.extension();

    let stem_str = original_file_stem.to_string_lossy();
    let stem_snake = if hidden {
        format!(".{}", stem_str.to_snake_case())
    } else {
        stem_str.to_snake_case()
    };

    let new_name = if let Some(ext) = original_ext {
        format!("{}.{}", stem_snake, ext.to_string_lossy())
    } else {
        stem_snake.clone()
    };

    // Return if the name is already in snake_case
    if original_name == new_name {
        return Ok(());
    }

    let mut candidate = parent_dir.join(&new_name);

    if candidate.exists() {
        let new_file_stem = Path::new(&new_name)
            .file_stem()
            .context(format!("Failed to extract the stem from {:?}", new_name))?;
        let mut counter = 1;

        while candidate.exists() {
            let new_stem = format!("{}_{}", new_file_stem.to_string_lossy(), counter);

            if let Some(ext) = original_ext {
                candidate = parent_dir.join(format!("{}.{}", new_stem, ext.to_string_lossy()));
            } else {
                candidate = parent_dir.join(&new_stem);
            }

            counter += 1;
        }
    }

    if args.dry_run {
        println!("[Dry Run] Would rename '{original_name}' to '{new_name}'");
    } else {
        fs::rename(path, &candidate)
            .with_context(|| format!("Failed to rename '{:?}' to '{:?}'", path, candidate))?;
        if args.verbose {
            println!("Renamed '{original_name}' to '{new_name}'")
        }
    }
    Ok(())
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

        let hidden = is_hidden(file_name);

        // Skip hidden files when applicable
        if !args.include_hidden && hidden {
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
        rename_file_or_folder(&path, original_name, hidden, args)?;
    }

    Ok(())
}

// --------------------------------------------------

fn main() -> Result<()> {
    let args = Args::parse();
    let path = &args.path;

    // Check if the provided path exists
    if !path.exists() {
        anyhow::bail!("Error: Path {:?} does not exist", path);
    }

    // Check if the provided path is a directory
    let metadata = fs::symlink_metadata(path)
        .with_context(|| format!("Failed to get metadata for: {:?}", path))?;
    if !metadata.is_dir() {
        anyhow::bail!("Error: Path {:?} is not a directory", path);
    }

    process_dir(path, &args)?;

    Ok(())
}
