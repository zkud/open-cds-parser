use super::traits::ast_term::ASTTerm;

pub struct FieldTerm {
  name: Box<dyn ASTTerm>,
  type_name: Box<dyn ASTTerm>,
}

impl ASTTerm for FieldTerm {}

impl FieldTerm {
  pub fn new_boxed(name: Box<dyn ASTTerm>, type_name: Box<dyn ASTTerm>) -> Box<FieldTerm> {
    Box::new(FieldTerm::new(name, type_name))
  }

  pub fn new(name: Box<dyn ASTTerm>, type_name: Box<dyn ASTTerm>) -> FieldTerm {
    FieldTerm { name, type_name }
  }
}
