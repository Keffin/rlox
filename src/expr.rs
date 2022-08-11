use crate::token::Token;

#[derive(Debug, Clone)]
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
    Literal {
        // Temp solution for representing string as null
        literal: LiteralRepresentations,
    },
    Grouping {
        expr: Box<Expr>,
    },
    // Temp solution for handling end scenario of primary
    FailScenario {
        reason: String,
    },
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub literal: LiteralRepresentations,
}

// Represent Lox Nil type as a custom Null type which will be a string, temp solution
type Null = String;
#[derive(Debug, PartialEq, Clone)]
pub enum LiteralRepresentations {
    CustomBoolean { val: bool },
    CustomNil { val: Null },
    CustomNumber { val: f64 },
    CustomString { val: String },
}

pub struct CustomBoolean {
    val: bool,
}

pub struct CustomNil {
    val: Null,
}

pub struct CustomNumber {
    val: f64,
}

pub struct CustomString {
    val: String,
}
