use std::fmt;
use std::io;

#[derive(fmt::Debug)]
pub struct FileSystemError {
    message: String,
}

impl FileSystemError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for FileSystemError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "file i/o error, reason: {}", self.message)
    }
}

impl From<io::Error> for FileSystemError {
    fn from(error: io::Error) -> FileSystemError {
        FileSystemError::new(format!("{}", error))
    }
}
