use crate::{expr::Expr, token::Token};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expression),
    Print(Print),
    Var(Var),
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub expression: Expr,
}

#[derive(Debug, Clone)]
pub struct Print {
    pub print_expression: Expr,
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: Token,
    // Initializer should be optional
    pub initializer: Option<Expr>,
}
