use std::fmt;

#[derive(fmt::Debug)]
pub struct VisitorError {
  message: String,
}

impl VisitorError {
  pub fn new(message: String) -> VisitorError {
    VisitorError { message }
  }

  pub fn get_message(&self) -> String {
    self.message.clone()
  }
}

impl fmt::Display for VisitorError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(formatter, "Visitor reason: {}", self.message)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let error = VisitorError::new("Test error".to_string());
    assert_eq!(error.message, "Test error");
  }

  #[test]
  fn test_get_message() {
    let error = VisitorError::new("Test error".to_string());
    assert_eq!(error.get_message(), "Test error");
  }

  #[test]
  fn test_debug() {
    let error = VisitorError::new("Test error".to_string());
    assert_eq!(
      format!("{:?}", error),
      "VisitorError { message: \"Test error\" }"
    );
  }

  #[test]
  fn test_display() {
    let error = VisitorError::new("Test error".to_string());
    assert_eq!(format!("{}", error), "Visitor reason: Test error");
  }
}
