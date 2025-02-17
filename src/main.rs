use std::env;
use std::fs;
use std::path::Path;
use rayon::prelude::*;

mod helpers;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let pattern = "%Y%m%d_%H%M%S"; // Example pattern: YYYYMMDD_hh24mmss
    
    if args.len() != 2 {
        println!("Usage: exif_renamer <directory>");
        return Ok(());
    }
    
    let directory = Path::new(&args[1]);
    
    if !directory.is_dir() {
        println!("Invalid directory path");
        return Ok(());
    }
    
    let files = fs::read_dir(directory)?;
    
    files.par_bridge().for_each(|file| {
        if let Ok(entry) = file {
            let path = entry.path();
            
            if let Some(extension) = path.extension() {
                if extension == "jpg" || extension == "jpeg" {
                    // Read the EXIF data from the file using the exif crate
                    helpers::process_image_file(&path, &pattern, &extension)
                }
            }
        }
    });
    
    Ok(())
}
