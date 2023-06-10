use std::env;
use std::fs;
use std::path::Path;
use chrono::{DateTime, NaiveDateTime, Local, TimeZone};
use rayon::prelude::*;

fn create_datetime_from_string(date_string: &str) -> Option<DateTime<Local>> {
    let format = "%Y-%m-%d %H:%M:%S";
    let parsed_date = NaiveDateTime::parse_from_str(date_string, format);

    match parsed_date {
        Ok(parsed) => Some(Local.from_local_datetime(&parsed).single().unwrap()),
        Err(_) => None,
    }
}

fn format_date(date: DateTime<Local>, pattern: &str) -> String {
    date.format(pattern).to_string()
}

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
                    if let Ok(filex) = std::fs::File::open(&path) {
                        let mut bufreader = std::io::BufReader::new(&filex);
                        let exifreader = exif::Reader::new();
                        if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
                            println!("filename: {}", path.display());
                            for f in exif.fields() {
                                if f.tag.to_string().to_lowercase().starts_with("date") {
                                    let date_string = f.display_value().to_string();
                                    println!("{} {} {}",
                                        f.tag, f.ifd_num, date_string);
                                    let current_date = create_datetime_from_string(&date_string);
                                    let formatted_date = format_date(current_date.unwrap(), pattern);
                                    println!("Formatted date: {}", formatted_date);
                                }
                            }
                            println!("");
                        }
                    }
                }
            }
        }
    });
    
    Ok(())
}
