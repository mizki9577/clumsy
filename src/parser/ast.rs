#[derive(Debug, PartialEq)]
pub enum AST {
    Abstraction(ASTAbstraction),
    Application(ASTApplication),
    Identifier(ASTIdentifier),
}

#[derive(Debug, PartialEq)]
pub struct ASTAbstraction {
    pub parameters: Vec<ASTIdentifier>,
    pub expression: Box<AST>,
}

impl ASTAbstraction {
    pub fn new(parameters: impl IntoIterator<Item = ASTIdentifier>, expression: AST) -> Self {
        Self {
            parameters: parameters.into_iter().collect(),
            expression: box expression,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ASTApplication {
    pub expressions: Vec<AST>,
}

impl ASTApplication {
    pub fn new(expressions: impl IntoIterator<Item = AST>) -> Self {
        Self {
            expressions: expressions.into_iter().collect(),
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
