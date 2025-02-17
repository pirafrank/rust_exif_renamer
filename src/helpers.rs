use std::ffi::OsStr;
use std::path::Path;
use chrono::{DateTime, NaiveDateTime, Local, TimeZone};

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

fn rename_file(formatted_date: &str, path: &Path, extension: &OsStr) {
    let new_filename = format!("{}.{}", formatted_date, extension.to_string_lossy());
    let new_path = path.with_file_name(new_filename);
    if let Err(err) = std::fs::rename(&path, &new_path) {
        println!("Failed to rename file: {}", err);
    } else {
        println!("{} renamed to: {}", path.display(), new_path.display());
    }
}

pub fn process_image_file(path: &Path, pattern: &str, extension: &OsStr) {
  if let Ok(filex) = std::fs::File::open(&path) {
      let mut bufreader = std::io::BufReader::new(&filex);
      let exifreader = exif::Reader::new();
      if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
          println!("filename: {}", path.display());
          for f in exif.fields() {
              if f.tag.to_string().to_lowercase().starts_with("datetimeoriginal") {
                  let date_string = f.display_value().to_string();
                  println!("{}: {}",
                      f.tag, date_string);
                  let current_date = create_datetime_from_string(&date_string);
                  let formatted_date = format_date(current_date.unwrap(), pattern);
                  println!("Formatted date: {}", formatted_date);
                  rename_file(&formatted_date, &path, &extension);
                  continue;
              }
          }
          println!("");
      }
  }
}
