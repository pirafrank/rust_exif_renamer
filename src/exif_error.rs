#[derive(Debug)]
pub enum ExifError {
    DateParseError(String),
    FileError(std::io::Error),
    ExifError(exif::Error),
}

impl From<std::io::Error> for ExifError {
    fn from(err: std::io::Error) -> Self {
        ExifError::FileError(err)
    }
}

impl From<exif::Error> for ExifError {
    fn from(err: exif::Error) -> Self {
        ExifError::ExifError(err)
    }
}
