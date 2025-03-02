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

    /**
     * Creates a new mock location to be reused in all tests
     */
    #[cfg(test)]
    pub fn new_mock() -> Self {
        Self {
            start: 0,
            end: 0,
            filepath: PathBuf::new(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_location_creation() {
        let filepath = Path::new("test/file/path.rs");
        let location = Location::new(10, 20, &filepath);

        assert_eq!(location.start(), 10);
        assert_eq!(location.end(), 20);
        assert_eq!(&location.filepath(), &filepath);
    }

    #[test]
    fn test_location_equality() {
        let filepath1 = Path::new("test/file/path.rs");
        let filepath2 = Path::new("test/file/path.rs");
        let location1 = Location::new(10, 20, &filepath1);
        let location2 = Location::new(10, 20, &filepath2);

        assert_eq!(location1, location2);
    }

    #[test]
    fn test_location_clone() {
        let filepath = Path::new("test/file/path.rs");
        let location = Location::new(10, 20, &filepath);
        let cloned_location = location.clone();

        assert_eq!(location, cloned_location);
    }

    #[test]
    fn test_location_start_end() {
        let filepath = Path::new("test/file/path.rs");
        let location = Location::new(5, 15, &filepath);

        assert_eq!(location.start(), 5);
        assert_eq!(location.end(), 15);
    }

    #[test]
    fn test_location_filepath() {
        let filepath = Path::new("another/test/path.rs");
        let location = Location::new(0, 0, &filepath);

        assert_eq!(&location.filepath(), &filepath);
    }
}
