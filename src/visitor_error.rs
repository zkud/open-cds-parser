use std::fmt;

#[derive(fmt::Debug)]
pub struct VisitorError {
  message: String,
}

impl VisitorError {
  pub fn new(message: String) -> VisitorError {
    VisitorError {
      message,
    }
  }

  pub fn get_message(&self) -> String {
    self.message.clone()
  }
}

impl fmt::Display for VisitorError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "Visitor reason: {}",
      self.message
    )
  }
}