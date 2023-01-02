use super::module_term::ModuleTerm;
use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
#[ast_term(visitor_path = "process_root")]
pub struct RootTerm {
  #[subnode_prop]
  modules: Vec<ModuleTerm>,
}
