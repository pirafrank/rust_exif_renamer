use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
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
    
    for file in files {
        let entry = file?;
        let path = entry.path();
        
        if let Some(extension) = path.extension() {
            if extension == "jpg" || extension == "jpeg" {
                // Read the EXIF data from the file using the exif crate
                let filex = std::fs::File::open(&path)?;
                let mut bufreader = std::io::BufReader::new(&filex);
                let exifreader = exif::Reader::new();
                let exif = exifreader.read_from_container(&mut bufreader)?;
                println!("filename: {}", path.display());
                for f in exif.fields() {
                    if f.tag.to_string().to_lowercase().starts_with("date") {
                      println!("{} {} {}",
                              f.tag, f.ifd_num, f.display_value().with_unit(&exif));
                    }
                }
                println!("")
            }
        }
    }
    
    Ok(())
}
