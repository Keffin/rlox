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

impl LiteralRepresentations {
    pub fn create_num(&self, num: f64) -> CustomNumber {
        CustomNumber { val: num }
    }

    pub fn get_boolean(&self, l: CustomBoolean) -> bool {
        l.val
    }

    pub fn get_number(&self, l: CustomNumber) -> f64 {
        l.val
    }

    pub fn get_string(&self, l: CustomString) -> String {
        l.val
    }
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
