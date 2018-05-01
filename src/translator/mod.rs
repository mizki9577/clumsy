#[cfg(test)]
mod tests;

use parser::ast;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum DeBruijnIndex {
    Abstraction(Box<DeBruijnIndex>),
    Application {
        callee: Box<DeBruijnIndex>,
        argument: Box<DeBruijnIndex>,
    },
    Index(Option<usize>),
}

impl DeBruijnIndex {
    pub fn from_ast(expr: &ast::Expression) -> DeBruijnIndex {
        DeBruijnIndex::from_ast_impl(expr, &mut HashMap::new())
    }

    fn from_ast_impl<'a>(
        expr: &'a ast::Expression,
        symbol_table: &mut HashMap<&'a ast::Variable, usize>,
    ) -> DeBruijnIndex {
        match expr {
            ast::Expression::Abstraction {
                parameter,
                expression,
            } => {
                symbol_table.iter_mut().for_each(|(_, i)| *i += 1);
                symbol_table.insert(parameter, 0);

                DeBruijnIndex::Abstraction(box DeBruijnIndex::from_ast_impl(
                    expression,
                    symbol_table,
                ))
            }
            ast::Expression::Application { callee, argument } => {
                let callee = box DeBruijnIndex::from_ast_impl(callee, symbol_table);
                let argument = box DeBruijnIndex::from_ast_impl(argument, symbol_table);
                DeBruijnIndex::Application { callee, argument }
            }
            ast::Expression::Variable(variable) => {
                DeBruijnIndex::Index(symbol_table.get(variable).map(|i| *i))
            }
        }
    }
}
