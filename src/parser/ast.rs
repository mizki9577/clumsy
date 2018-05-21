#[derive(Debug, PartialEq)]
pub struct ASTProgram(pub Vec<ASTDirective>);

#[derive(Debug, PartialEq)]
pub enum ASTDirective {
    Expression(ASTExpression),
    Let(ASTLet),
}

#[derive(Debug, PartialEq)]
pub enum ASTExpression {
    Abstraction(ASTAbstraction),
    Application(ASTApplication),
    Identifier(ASTIdentifier),
}

impl<T> From<T> for ASTExpression
where
    T: Into<ASTIdentifier>,
{
    fn from(value: T) -> Self {
        ASTExpression::Identifier(value.into())
    }
}

#[derive(Debug, PartialEq)]
pub struct ASTAbstraction {
    pub parameters: Vec<ASTIdentifier>,
    pub expression: Box<ASTExpression>,
}

impl ASTAbstraction {
    pub fn new(
        parameters: impl IntoIterator<Item = impl Into<ASTIdentifier>>,
        expression: impl Into<ASTExpression>,
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

impl From<ASTAbstraction> for ASTExpression {
    fn from(value: ASTAbstraction) -> Self {
        ASTExpression::Abstraction(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct ASTApplication {
    pub expressions: Vec<ASTExpression>,
}

impl ASTApplication {
    pub fn new(expressions: impl IntoIterator<Item = impl Into<ASTExpression>>) -> Self {
        ASTApplication {
            expressions: expressions
                .into_iter()
                .map(|expression| expression.into())
                .collect(),
        }
    }
}

impl From<ASTApplication> for ASTExpression {
    fn from(value: ASTApplication) -> Self {
        ASTExpression::Application(value)
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

#[derive(Debug, PartialEq)]
pub struct ASTLet {
    pub variable: ASTIdentifier,
    pub expression: Box<ASTExpression>,
}

impl ASTLet {
    pub fn new(variable: impl Into<ASTIdentifier>, expression: impl Into<ASTExpression>) -> Self {
        ASTLet {
            variable: variable.into(),
            expression: box expression.into(),
        }
    }
}
