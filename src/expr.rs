use crate::token::Token;

pub enum Expr {
    Binary {
        // Recursive types, need to Box
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Literal(LiteralRepresentations),
    Grouping(Box<Expr>),
    // Temp solution for handling end scenario of primary
    FailScenario {
        reason: String,
    },
}

pub enum LiteralRepresentations {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
}
