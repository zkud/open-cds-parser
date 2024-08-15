use std::sync::Arc;

use crate::ast::*;
use crate::parser::*;

pub struct PathCollectorVisitor {
    file_system: Arc<dyn FileSystem>,
    current_dir: String,
    modules_to_parse: Vec<String>,
}

impl PathCollectorVisitor {
    pub fn new(file_system: Arc<dyn FileSystem>, current_dir: &str) -> Self {
        Self {
            file_system: file_system,
            current_dir: current_dir.to_string(),
            modules_to_parse: vec![],
        }
    }

    pub fn modules_to_parse(self) -> Vec<String> {
        self.modules_to_parse
    }
}

impl Visitor for PathCollectorVisitor {
    type Error = ParseError;

    fn process<T: ASTTerm>(&mut self, term: &T) -> Result<(), ParseError> {
        if let Some(term) = term.try_convert::<ImportTerm>() {
            let path = term.path().value();
            let full_path = self.file_system.join_paths(&self.current_dir, &path)?;
            self.modules_to_parse.push(full_path);
        }
        Ok(())
    }
}
