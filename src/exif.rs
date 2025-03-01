use std::ffi::OsStr;
use std::path::Path;
use chrono::{DateTime, NaiveDateTime, Local, TimeZone};
use little_exif::metadata::Metadata;
use little_exif::exif_tag::ExifTag;
use crate::exif_error::ExifError;

fn create_datetime_from_pattern(date_string: &str, format: &str) -> Option<DateTime<Local>> {
    let parsed_date = NaiveDateTime::parse_from_str(date_string, format);

    match parsed_date {
        Ok(parsed) => Some(Local.from_local_datetime(&parsed).single().unwrap()),
        Err(_) => None,
    }
}

fn create_datetime_from_string(date_string: &str) -> Option<DateTime<Local>> {
    let format = "%Y-%m-%d %H:%M:%S";
    create_datetime_from_pattern(date_string, format)
}

fn format_date(date: DateTime<Local>, pattern: &str) -> String {
    date.format(pattern).to_string()
}

fn rename_file(formatted_date: &str, path: &Path, extension: &OsStr) {
    let new_filename = format!("{}.{}", formatted_date, extension.to_string_lossy());
    let new_path = path.with_file_name(new_filename);
    if let Err(err) = std::fs::rename(&path, &new_path) {
        println!("Failed to rename file: {}", err);
    } else {
        println!("{} renamed to: {}", path.display(), new_path.display());
    }
}

pub fn exif_to_filename(path: &Path, pattern: &str, extension: &OsStr) {
  if let Ok(filex) = std::fs::File::open(&path) {
      let mut bufreader = std::io::BufReader::new(&filex);
      let exifreader = exif::Reader::new();
      if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
          for f in exif.fields() {
              if f.tag.to_string().to_lowercase().starts_with("datetimeoriginal") {
                  let date_string = f.display_value().to_string();
                  println!("{}: {}", f.tag, date_string);
                  let current_date = create_datetime_from_string(&date_string);
                  let formatted_date = format_date(current_date.unwrap(), pattern);
                  println!("Set date {} to file {}", formatted_date, path.display());
                  rename_file(&formatted_date, &path, &extension);
                  continue;
              }
          }
      }
  }
}

pub fn filename_to_exif(path: &Path, pattern: &str) {
    if let Err(err) = process_exif(path, pattern) {
        match err {
            ExifError::DateParseError(msg) => println!("Date parsing error: {}", msg),
            ExifError::FileError(err) => println!("File error: {}", err),
            ExifError::ExifError(err) => println!("EXIF error: {}", err),
        }
    }
}

fn process_exif(path: &Path, pattern: &str) -> Result<(), ExifError> {
    let format: &str = "%Y:%m:%d %H:%M:%S";
    let stem: &str = path.file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| ExifError::DateParseError("Invalid filename".to_string()))?;

    let date_time = create_datetime_from_pattern(&stem, pattern)
        .ok_or_else(|| ExifError::DateParseError(format!("Invalid date format {}", stem)))?;

    let formatted_date = date_time.format(&format).to_string();
    println!("Found date {} in filename {}", formatted_date, stem);

    let mut metadata = Metadata::new_from_path(&path)
        .map_err(|e| ExifError::ExifError(exif::Error::Io(e)))?;

    metadata.set_tag(ExifTag::DateTimeOriginal(formatted_date));

    metadata.write_to_file(&path)
        .map_err(|e| ExifError::ExifError(exif::Error::Io(e)))?;

    println!("Updated EXIF data in file {}", path.display());
    Ok(())
}
