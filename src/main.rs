use clap::{Parser, ValueEnum};
use std::path::Path;
use rayon::prelude::*;
use std::fs;

mod helpers;

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
    #[arg(short = 'P', long, default_value = "%Y%m%d_%H%M%S")]
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

    match cli.command {
        Command::ExifToFilename => {
            let pattern = cli.pattern.unwrap_or_else(|| String::from("%Y%m%d_%H%M%S"));
            files.par_bridge().for_each(|file| {
                if let Ok(entry) = file {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == "jpg" || extension == "jpeg" {
                            helpers::exif_to_filename(&path, &pattern, &extension)
                        }
                    }
                }
            });
        }
        Command::FilenameToExif => {
            files.par_bridge().for_each(|file| {
                if let Ok(entry) = file {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == "jpg" || extension == "jpeg" {
                            helpers::filename_to_exif(&path)
                        }
                    }
                }
            });
        }
    }

    Ok(())
}
