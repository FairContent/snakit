use std::path::PathBuf;

// --------------------------------------------------

use anyhow::Result;
use clap::Parser;

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

fn main() -> Result<()> {
    let args = Args::parse();

    let path = &args.path;
    if !path.exists() || !path.is_dir() {
        anyhow::bail!("Error: Path {:?} is invalid or not a directory", path);
    }

    Ok(())
}
