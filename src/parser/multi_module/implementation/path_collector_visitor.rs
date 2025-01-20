use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::ast::*;
use crate::parser::*;

pub struct PathCollectorVisitor {
    file_system: Arc<dyn FileSystem>,
    current_dir: PathBuf,
    modules_to_parse: Vec<PathBuf>,
}

impl PathCollectorVisitor {
    pub fn new(file_system: Arc<dyn FileSystem>, current_dir: &Path) -> Self {
        Self {
            file_system: file_system,
            current_dir: current_dir.to_path_buf(),
            modules_to_parse: vec![],
        }
    }

    pub fn modules_to_parse(self) -> Vec<PathBuf> {
        self.modules_to_parse
    }
}

impl Visitor for PathCollectorVisitor {
    type Error = ParseError;

    fn process<T: ASTTerm>(&mut self, term: &T) -> Result<(), ParseError> {
        if let Some(term) = term.try_convert::<ImportTerm>() {
            let path = term.path().value();
            let full_path = self
                .file_system
                .join_paths(&Path::new(&self.current_dir), &Path::new(&path))?;
            self.modules_to_parse.push(full_path);
        }
        Ok(())
    }
}
