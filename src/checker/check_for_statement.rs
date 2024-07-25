use super::Checker;
use crate::ast::ast;

impl Checker {
  pub fn check_for_statement(&mut self, for_: &ast::ForStatement) {
    // Empty statements don't change the type context
  }
}