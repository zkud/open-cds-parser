use super::super::visitor::Visitor;
use super::super::visitor_error::VisitorError;
use super::traits::ast_term::ASTTerm;
use super::traits::module_usable_term::ModuleUsableTerm;

pub struct ModuleTerm {
  definitions: Vec<Box<dyn ModuleUsableTerm>>,
}

impl ASTTerm for ModuleTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_module(self)?;

    for param in self.definitions.iter() {
      param.accept(visitor)?;
    }

    Ok(())
  }
}

impl ModuleTerm {
  pub fn definitions(&self) -> &[Box<dyn ModuleUsableTerm>] {
    self.definitions.as_ref()
  }

  pub fn new_boxed(definitions: Vec<Box<dyn ModuleUsableTerm>>) -> Box<ModuleTerm> {
    Box::new(ModuleTerm::new(definitions))
  }

  pub fn new(definitions: Vec<Box<dyn ModuleUsableTerm>>) -> ModuleTerm {
    ModuleTerm { definitions }
  }
}
