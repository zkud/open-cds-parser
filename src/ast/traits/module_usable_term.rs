use super::ast_term::ASTTerm;
use super::module_term_type::ModuleTermType;

pub trait ModuleUsableTerm: ASTTerm {
  fn get_type(&self) -> ModuleTermType;
}
