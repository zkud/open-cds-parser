use super::traits::ast_term::ASTTerm;

pub struct ParamTerm {
  name: Box<dyn ASTTerm>,
  type_name: Box<dyn ASTTerm>,
}

impl ASTTerm for ParamTerm {}

impl ParamTerm {
  pub fn new_boxed(name: Box<dyn ASTTerm>, type_name: Box<dyn ASTTerm>) -> Box<ParamTerm> {
    Box::new(ParamTerm::new(name, type_name))
  }

  pub fn new(name: Box<dyn ASTTerm>, type_name: Box<dyn ASTTerm>) -> ParamTerm {
    ParamTerm { name, type_name }
  }
}
