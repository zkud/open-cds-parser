use super::super::ast::{
  ActionTerm, EntityTerm, FieldTerm, FunctionTerm, LeafTerm, ModuleTerm, NameTerm, NodeTerm,
  ParamTerm, ReturnsTerm, ServiceTerm, TypeTerm,
};
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
  fn process_mock_leaf(&mut self, _term: &LeafTerm) -> Result<(), VisitorError> {
    Ok(())
  }
  fn process_mock_node(&mut self, _term: &NodeTerm) -> Result<(), VisitorError> {
    Ok(())
  }
}
