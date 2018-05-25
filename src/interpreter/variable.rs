use ast;

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: String,
    pub index: Option<usize>,
}

impl Variable {
    pub fn new<T>(index: Option<usize>, name: T) -> Variable
    where
        T: Into<String>,
    {
        Variable {
            name: name.into(),
            index,
        }
    }
}

impl<'a> From<&'a ast::Identifier> for Variable {
    fn from(value: &ast::Identifier) -> Variable {
        let ast::Identifier(identifier) = value;
        Variable {
            name: identifier.to_owned(),
            index: None,
        }
    }
}
