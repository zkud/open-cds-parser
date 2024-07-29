use super::super::ast::*;
#[cfg(test)]
use super::super::ast::{LeafTerm, NodeTerm};
use super::VisitorError;

pub trait Visitor {
    fn process_name(&mut self, _term: &NameTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_type(&mut self, _term: &TypeTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_param(&mut self, _term: &ParamTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_returns(&mut self, _term: &ReturnsTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_field(&mut self, _term: &FieldTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_function(&mut self, _term: &FunctionTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_action(&mut self, _term: &ActionTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_entity(&mut self, _term: &EntityTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_service(&mut self, _term: &ServiceTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_module(&mut self, _term: &ModuleTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_path(&mut self, _term: &PathTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_import(&mut self, _term: &ImportTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_selector(&mut self, _term: &SelectorTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_selection_block(&mut self, _term: &SelectionBlockTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_select_all(&mut self, _term: &SelectAllTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_dot(&mut self, _term: &DotTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_wildcart(&mut self, _term: &WildcartTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_import_identifier(
        &mut self,
        _term: &ImportIdentifierTerm,
    ) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_comma(&mut self, _term: &CommaTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_as(&mut self, _term: &AsTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_alias(&mut self, _term: &AliasTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_from(&mut self, _term: &FromTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_semicolumn(&mut self, _term: &SemicolumnTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_open_curly_brace(
        &mut self,
        _term: &OpenCurlyBraceTerm,
    ) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_close_curly_brace(
        &mut self,
        _term: &CloseCurlyBraceTerm,
    ) -> Result<(), VisitorError> {
        Ok(())
    }
    fn process_using(&mut self, _term: &UsingTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    #[cfg(test)]
    fn process_mock_leaf(&mut self, _term: &LeafTerm) -> Result<(), VisitorError> {
        Ok(())
    }
    #[cfg(test)]
    fn process_mock_node(&mut self, _term: &NodeTerm) -> Result<(), VisitorError> {
        Ok(())
    }
}
