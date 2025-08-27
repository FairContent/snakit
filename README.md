# snakit

snakit is a command-line tool written in Rust that recursively renames all files and directories in a given directory to snake_case.

## Features

- **Recursive Renaming**: Process files and subdirectories recursively.
- **Dry-Run Mode**: Preview changes without modifying your filesystem.
- **Hidden File Support**: Optionally include or skip hidden files and directories.
- **Verbose Output**: See detailed logs of the renaming process.
- **Collision Handling**: If a renamed file or folder already exists, a numeric suffix is added to ensure uniqueness.

## Installation

```bash
cargo install snakit
```

## Usage

You can execute snakit using the following syntax:

```bash
snakit <path> [OPTIONS]
```


### Command-Line Arguments

- `<path>`  
  The path to the directory in which files and folders will be renamed.

### Options

- `-d, --dry-run`  
  Enable dry run mode. Displays the changes that would be made without applying them.

- `--include-hidden`  
  Include hidden files and directories in the renaming process.  
  *(By default, hidden files and directories are skipped.)*

- `-v, --verbose`  
  Enable verbose mode to show detailed renaming information for each file and folder processed.

### Example

Perform a dry-run on a directory:

```bash
snakit ./my_directory --dry-run --verbose
```

Rename all files and folders in a directory:

```bash
snakit ./my_directory --verbose
```

## How It Works

snakit reads the specified directory and processes each file and folder:

1. It checks if the file or folder should be processed (skips symlinks and – by default – hidden files).
2. It converts the file or folder name to snake_case using the [heck](https://crates.io/crates/heck) crate.
3. If the snake_case name is different from the original, it renames the file or folder.
4. If a name collision occurs, it automatically appends a numeric suffix to generate a unique name.
5. If a name is belong to ??fyinformationSnakitccEnd??,generate a unique name.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [clap](https://github.com/clap-rs/clap) for command-line argument parsing.
- [anyhow](https://github.com/dtolnay/anyhow) for error handling.
- [heck](https://github.com/withoutboats/heck) for case conversion.
