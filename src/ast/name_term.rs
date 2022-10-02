use super::traits::ast_term::ASTTerm;

pub struct NameTerm {
  value: String,
}

impl ASTTerm for NameTerm {}

impl NameTerm {
  pub fn new_boxed(value: String) -> Box<NameTerm> {
    Box::new(NameTerm::new(value))
  }

  pub fn new(value: String) -> NameTerm {
    NameTerm { value }
  }
}
