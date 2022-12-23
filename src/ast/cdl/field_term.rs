use super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
#[ast_term(process_path = "process_field")]
pub struct FieldTerm {
  #[subnode_prop]
  name: Box<NameTerm>,
  #[subnode_prop]
  type_name: Box<NameTerm>,
}
