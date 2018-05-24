use ast::Statement;

#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<Statement>);
