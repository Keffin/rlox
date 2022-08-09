use std::error::Error;

use crate::expr::{self, CustomBoolean, Expr, Literal, LiteralRepresentations};
use crate::token::Token;
use crate::token_type::TokenType;

struct Interpreter {
    test: String,
}

struct InterpreterError {
    reason: String,
}

type RLoxEvalResult = Result<Expr, InterpreterError>;

impl Interpreter {
    pub fn eval(&mut self, expr: Expr) -> RLoxEvalResult {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => self.eval_bin(left, operator, right),

            Expr::Literal { literal } => match literal {
                LiteralRepresentations::CustomBoolean { val } => {
                    return Ok(Expr::Literal {
                        literal: LiteralRepresentations::CustomBoolean { val },
                    })
                }
                LiteralRepresentations::CustomNil { val } => {
                    return Ok(Expr::Literal {
                        literal: LiteralRepresentations::CustomNil { val },
                    })
                }
                LiteralRepresentations::CustomNumber { val } => {
                    return Ok(Expr::Literal {
                        literal: LiteralRepresentations::CustomNumber { val },
                    })
                }
                LiteralRepresentations::CustomString { val } => {
                    return Ok(Expr::Literal {
                        literal: LiteralRepresentations::CustomString { val },
                    })
                }
            },

            Expr::Grouping { expr } => self.eval(*expr),
            Expr::Unary { operator, right } => self.eval_unary(operator, *right),
            Expr::FailScenario { reason } => self.eval_fail_scenario(reason),
        }
    }

    fn eval_bin(&self, left: Box<Expr>, token: Token, right: Box<Expr>) -> RLoxEvalResult {
        todo!()
    }

    fn eval_unary(&mut self, operator: Token, right: Expr) -> RLoxEvalResult {
        let right: RLoxEvalResult = self.eval(right);

        match operator.token_type {
            TokenType::BANG => match right {
                Ok(Expr::Literal {
                    literal: LiteralRepresentations::CustomBoolean { val },
                }) => {
                    return Ok(Expr::Literal {
                        literal: LiteralRepresentations::CustomBoolean {
                            val: self.is_truthy(Literal {
                                literal: LiteralRepresentations::CustomBoolean { val: !val },
                            }),
                        },
                    });
                }
                _ => Err(InterpreterError {
                    reason: "Only accepts boolean types".to_string(),
                }),
            },
            TokenType::MINUS => match right {
                Ok(Expr::Literal {
                    literal: LiteralRepresentations::CustomNumber { val: number },
                }) => {
                    return Ok(Expr::Literal {
                        literal: LiteralRepresentations::CustomNumber { val: -number },
                    });
                }
                _ => Err(InterpreterError {
                    reason: "Could not parse expression in unary evaluation".to_string(),
                }),
            },
            _ => Err(InterpreterError {
                reason: "Failed to interpret, unary method accepts only BANG and MINUS type"
                    .to_string(),
            }),
        }
    }

    fn is_truthy(&self, literal_expr: Literal) -> bool {
        match literal_expr {
            Literal {
                literal: LiteralRepresentations::CustomNil { val },
            } => return false,
            Literal {
                literal: LiteralRepresentations::CustomBoolean { val },
            } => return val,
            _ => return true,
        }
    }

    fn eval_fail_scenario(&self, reason: String) -> RLoxEvalResult {
        todo!()
    }
}
