use super::name_term::NameTerm;
use super::param_term::ParamTerm;
use super::returns_term::ReturnsTerm;
use super::super::OpenRoundBracketTerm;
use super::super::CloseRoundBracketTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ActionTerm {
    #[subnode_prop]
    name: Box<NameTerm>,
    #[subnode_prop]
    open_bracket: Box<OpenRoundBracketTerm>,
    #[subnode_prop]
    params: Vec<ParamTerm>,
    #[subnode_prop]
    close_bracket: Box<CloseRoundBracketTerm>,
    #[subnode_prop]
    returned_type: Option<Box<ReturnsTerm>>,
}
