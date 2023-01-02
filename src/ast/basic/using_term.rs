use super::file_path_term::FilePathTerm;
use super::using_ref_term::UsingRefTerm;
use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
#[ast_term(visitor_path = "process_using")]
pub struct UsingTerm {
  #[subnode_prop]
  file_path: Box<FilePathTerm>,
  #[subnode_prop]
  using_refs: Vec<UsingRefTerm>,
}
