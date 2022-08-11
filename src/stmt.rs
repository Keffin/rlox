use crate::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expression),
    Print(Print),
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub expression: Expr,
}

#[derive(Debug, Clone)]
pub struct Print {
    pub print_expression: Expr,
}
