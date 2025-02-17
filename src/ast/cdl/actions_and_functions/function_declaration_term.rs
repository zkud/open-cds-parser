use super::super::FunctionTerm;
use super::super::NameTerm;
use super::super::ParametersBlockTerm;
use super::super::TypeReferenceTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct FunctionDeclarationTerm {
    #[subnode_prop]
    function: Box<FunctionTerm>,
    #[subnode_prop]
    name: Box<NameTerm>,
    #[subnode_prop]
    parameters: Box<ParametersBlockTerm>,
    #[subnode_prop]
    returned_type: Box<TypeReferenceTerm>,
}
