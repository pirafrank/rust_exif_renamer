use clap::{Parser, ValueEnum};
use std::fs;
use std::io;
use std::{path::Path, path::PathBuf};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

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

const SUPPORTED_EXTENSIONS: [&str; 2] = ["jpg", "jpeg"];
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

    let files: Vec<_> = fs::read_dir(directory)?
        .filter_map(|entry: Result<fs::DirEntry, io::Error>| {
            let entry: fs::DirEntry = entry.ok()?;
            // let entry.path live long enough to be used
            let binding: PathBuf = entry.path();
            let file_ext: &str = binding.extension()?.to_str()?;
            // go lowercase for case-insensitive comparison
            if SUPPORTED_EXTENSIONS.contains(&file_ext.to_lowercase().as_str()) {
                Some(entry)
            } else {
                None
            }
        })
        .collect();

    let total: usize = files.len();
    let pattern: String = cli.pattern.unwrap_or_else(|| DEFAULT_PATTERN.to_string());
    let mutex: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let process_files = |process_fn: fn(&Path, &str)| {
        files.into_par_iter().for_each(|file: fs::DirEntry| {
            // Update the progress
            let mut progress = mutex.lock().unwrap();
            *progress += 1;
            println!("\nProcessing: {}/{}\n", *progress, total);
            // Process file
            let path: PathBuf = file.path();
            println!("Processing file: {}", file.path().display());
            process_fn(&path, &pattern);
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
