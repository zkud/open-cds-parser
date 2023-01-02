use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
#[ast_term(visitor_path = "process_file_path")]
pub struct FilePathTerm {
  #[prop]
  value: String,
}
