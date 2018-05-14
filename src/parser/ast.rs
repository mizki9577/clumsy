#[derive(Debug, PartialEq)]
pub enum AST {
    Abstraction(ASTAbstraction),
    Application(ASTApplication),
    Identifier(ASTIdentifier),
}

impl<T> From<T> for AST
where
    T: Into<ASTIdentifier>,
{
    fn from(value: T) -> Self {
        AST::Identifier(value.into())
    }
}

#[derive(Debug, PartialEq)]
pub struct ASTAbstraction {
    pub parameters: Vec<ASTIdentifier>,
    pub expression: Box<AST>,
}

impl ASTAbstraction {
    pub fn new(
        parameters: impl IntoIterator<Item = impl Into<ASTIdentifier>>,
        expression: impl Into<AST>,
    ) -> Self {
        ASTAbstraction {
            parameters: parameters
                .into_iter()
                .map(|parameter| parameter.into())
                .collect(),
            expression: box expression.into(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ASTApplication {
    pub expressions: Vec<AST>,
}

impl ASTApplication {
    pub fn new(expressions: impl IntoIterator<Item = impl Into<AST>>) -> Self {
        ASTApplication {
            expressions: expressions
                .into_iter()
                .map(|expression| expression.into())
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ASTIdentifier(pub String);

impl<T> From<T> for ASTIdentifier
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        ASTIdentifier(value.into())
    }
}
