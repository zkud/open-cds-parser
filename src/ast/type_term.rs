use super::super::visitor::Visitor;
use super::super::visitor_error::VisitorError;
use super::ast_term::ASTTerm;
use super::name_term::NameTerm;

pub struct TypeTerm {
  name: Box<NameTerm>,
  resolved_type_name: Box<NameTerm>,
}

impl ASTTerm for TypeTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_type(self)?;
    self.name.accept(visitor)?;
    self.resolved_type_name.accept(visitor)?;
    Ok(())
  }
}

impl TypeTerm {
  pub fn name(&self) -> &NameTerm {
    self.name.as_ref()
  }

  pub fn resolved_type_name(&self) -> &NameTerm {
    self.resolved_type_name.as_ref()
  }

  pub fn new_boxed(name: Box<NameTerm>, resolved_type_name: Box<NameTerm>) -> Box<TypeTerm> {
    Box::new(TypeTerm::new(name, resolved_type_name))
  }

  pub fn new(name: Box<NameTerm>, resolved_type_name: Box<NameTerm>) -> TypeTerm {
    TypeTerm {
      name,
      resolved_type_name,
    }
  }
}
