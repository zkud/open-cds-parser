use std::path::{Path, PathBuf};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Location {
    start: usize,
    end: usize,
    filepath: PathBuf,
}

impl Location {
    /// Creates a new [`Location`].
    pub fn new(start: usize, end: usize, filepath: &Path) -> Self {
        Self {
            start,
            end,
            filepath: PathBuf::from(filepath),
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn filepath(&self) -> &Path {
        &self.filepath
    }
}
