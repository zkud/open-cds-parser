use super::traits::ast_term::ASTTerm;
use super::traits::module_usable_term::ModuleUsableTerm;

pub struct ModuleTerm {
  definitions: Vec<Box<dyn ModuleUsableTerm>>,
}

impl ASTTerm for ModuleTerm {}

impl ModuleTerm {
  pub fn new_boxed(definitions: Vec<Box<dyn ModuleUsableTerm>>) -> Box<ModuleTerm> {
    Box::new(ModuleTerm::new(definitions))
  }

  pub fn new(definitions: Vec<Box<dyn ModuleUsableTerm>>) -> ModuleTerm {
    ModuleTerm { definitions }
  }
}
