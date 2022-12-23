use super::field_term::FieldTerm;
use super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
#[ast_term(process_path = "process_entity")]
pub struct EntityTerm {
  #[subnode_prop]
  name: Box<NameTerm>,
  #[subnode_prop]
  applied_aspects: Vec<Box<NameTerm>>,
  #[subnode_prop]
  fields: Vec<Box<FieldTerm>>,
}
