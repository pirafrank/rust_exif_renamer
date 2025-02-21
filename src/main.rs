use clap::{Parser, ValueEnum};
use std::path::Path;
use rayon::prelude::*;
use std::fs;

mod exif;
mod exif_error;

// Version constants

const VERSION: &str = env!("CARGO_PKG_VERSION");
const COMMIT: &str = env!("GIT_COMMIT_HASH");
const BUILD_DATE: &str = env!("BUILD_DATE");
const AUTHOR : &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn long_version() -> &'static str {
    Box::leak(
        format!(
            "Version: {}\nCommit: {}\nBuild Date: {}",
            VERSION, COMMIT, BUILD_DATE
        )
        .into_boxed_str()
    )
}

// Constants

const DEFAULT_PATTERN: &str = "%Y%m%d_%H%M%S";

// Command line interface

#[derive(Clone, ValueEnum)]
enum Cmd {
    /// Rename files based on EXIF data
    ExifToFilename,
    /// Update EXIF data based on filename
    FilenameToExif,
}

#[derive(Parser)]
#[command(
  author = AUTHOR,
  version = VERSION,
  about = DESCRIPTION,
  long_version = long_version()
)]
struct Cli {
    /// Command to execute
    command: Cmd,

    /// Directory containing the images to process
    #[arg(short = 'p', long, required = true)]
    path: String,

    /// Date format pattern to parse from or set to filename
    #[arg(short = 'P', long, default_value = DEFAULT_PATTERN)]
    pattern: Option<String>,
}

// main

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let directory = Path::new(&cli.path);

    if !directory.is_dir() {
        println!("Error: Invalid directory path");
        return Ok(());
    }

    let files = fs::read_dir(directory)?;
    let pattern = cli.pattern.unwrap_or_else(|| DEFAULT_PATTERN.to_string());

    let process_files = |process_fn: fn(&Path, &str)| {
        files.par_bridge().for_each(|file| {
            if let Ok(entry) = file {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "jpg" || extension == "jpeg" {
                        println!("Processing: {}", entry.path().display());
                        process_fn(&path, &pattern);
                    }
                }
            }
        });
    };

    match cli.command {
        Cmd::ExifToFilename => process_files(
            |path: &Path, pattern: &str| exif::exif_to_filename(path, pattern, path.extension().unwrap())
        ),
        Cmd::FilenameToExif => process_files(
            |path: &Path, pattern: &str| exif::filename_to_exif(path, pattern)
        ),
    }

    Ok(())
}
