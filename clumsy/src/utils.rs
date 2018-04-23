use parser::ast;

pub fn new_abstraction(parameter: &str, expression: ast::Expression) -> ast::Expression {
    ast::Expression::Abstraction {
        parameter: ast::Variable::new(parameter),
        expression: box expression,
    }
}

pub fn new_application(callee: ast::Expression, argument: ast::Expression) -> ast::Expression {
    ast::Expression::Application {
        callee: box callee,
        argument: box argument,
    }
}

pub fn new_variable(variable: &str) -> ast::Expression {
    ast::Expression::Variable(ast::Variable::new(variable))
}
