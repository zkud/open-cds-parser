#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Location {
    start: usize,
    end: usize,
    filepath: String,
}

impl Location {
    /// Creates a new [`Location`].
    pub fn new(start: usize, end: usize, filepath: String) -> Self {
        Self {
            start,
            end,
            filepath,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn filepath(&self) -> &str {
        &self.filepath
    }
}
