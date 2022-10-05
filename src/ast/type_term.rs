use super::super::visitor::Visitor;
use super::super::visitor_error::VisitorError;
use super::traits::ast_term::ASTTerm;
use super::traits::module_term_type::ModuleTermType;
use super::traits::module_usable_term::ModuleUsableTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;

impl ModuleUsableTerm for TypeTerm {
  fn get_type(&self) -> ModuleTermType {
    ModuleTermType::Type
  }
}

impl ServiceUsableTerm for TypeTerm {
  fn get_type(&self) -> ServiceTermType {
    ServiceTermType::Type
  }
}

pub struct TypeTerm {
  name: Box<dyn ASTTerm>,
  resolved_type_name: Box<dyn ASTTerm>,
}

impl ASTTerm for TypeTerm {
  fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
    visitor.process_type(&self)?;
    self.name.accept(visitor)?;
    self.resolved_type_name.accept(visitor)?;
    Ok(())
  }
}

impl TypeTerm {
  pub fn new_boxed(name: Box<dyn ASTTerm>, resolved_type_name: Box<dyn ASTTerm>) -> Box<TypeTerm> {
    Box::new(TypeTerm::new(name, resolved_type_name))
  }

  pub fn new(name: Box<dyn ASTTerm>, resolved_type_name: Box<dyn ASTTerm>) -> TypeTerm {
    TypeTerm {
      name,
      resolved_type_name,
    }
  }
}
