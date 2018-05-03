#[cfg(test)]
mod tests;

use parser::ast;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum DeBruijnIndex {
    Abstraction {
        name: String,
        expression: Box<DeBruijnIndex>,
    },
    Application {
        callee: Box<DeBruijnIndex>,
        argument: Box<DeBruijnIndex>,
    },
    Variable {
        index: Option<usize>,
        name: String,
    },
}

impl DeBruijnIndex {
    pub fn from_ast(expr: &ast::Expression) -> DeBruijnIndex {
        DeBruijnIndex::from_ast_impl(expr, &mut HashMap::new())
    }

    fn from_ast_impl<'a>(
        expr: &'a ast::Expression,
        symbol_table: &mut HashMap<&'a str, usize>,
    ) -> DeBruijnIndex {
        match expr {
            ast::Expression::Abstraction {
                parameter: ast::Variable(parameter),
                expression,
            } => {
                symbol_table.iter_mut().for_each(|(_, i)| *i += 1);
                symbol_table.insert(parameter, 0);

                DeBruijnIndex::Abstraction {
                    name: parameter.to_owned(),
                    expression: box DeBruijnIndex::from_ast_impl(expression, symbol_table),
                }
            }
            ast::Expression::Application { callee, argument } => {
                let callee = box DeBruijnIndex::from_ast_impl(callee, symbol_table);
                let argument = box DeBruijnIndex::from_ast_impl(argument, symbol_table);
                DeBruijnIndex::Application { callee, argument }
            }
            ast::Expression::Variable(ast::Variable(variable)) => DeBruijnIndex::Variable {
                index: symbol_table.get(variable.as_str()).map(|i| *i),
                name: variable.to_owned(),
            },
        }
    }
}
