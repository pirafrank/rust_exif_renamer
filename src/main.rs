use clap::{Parser, ValueEnum};
use std::path::Path;
use rayon::prelude::*;
use std::fs;

mod exif;
mod exif_error;

const DEFAULT_PATTERN: &str = "%Y%m%d_%H%M%S";

#[derive(Clone, ValueEnum)]
enum Command {
    /// Rename files based on EXIF data
    ExifToFilename,
    /// Update EXIF data based on filename
    FilenameToExif,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Command to execute
    command: Command,

    /// Directory containing the images to process
    #[arg(short = 'p', long, required = true)]
    path: String,

    /// Date format pattern (only used with exif-to-filename)
    #[arg(short = 'P', long, default_value = DEFAULT_PATTERN)]
    pattern: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let directory = Path::new(&cli.path);

    if !directory.is_dir() {
        println!("Invalid directory path");
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
        Command::ExifToFilename => process_files(
            |path: &Path, pattern: &str| exif::exif_to_filename(path, pattern, path.extension().unwrap())
        ),
        Command::FilenameToExif => process_files(
            |path: &Path, pattern: &str| exif::filename_to_exif(path, pattern)
        ),
    }

    Ok(())
}
