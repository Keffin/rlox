use crate::expr::Expr::Literal;
use crate::expr::{self, CustomBoolean, Expr, LiteralRepresentations};
use crate::token::Token;
use crate::token_type::TokenType;

struct Interpreter {
    test: String,
}

impl Interpreter {
    pub fn eval(&mut self, expr: Expr) -> Expr {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => self.eval_bin(left, operator, right),

            Expr::Literal { literal } => match literal {
                LiteralRepresentations::CustomBoolean { val } => {
                    return Expr::Literal {
                        literal: LiteralRepresentations::CustomBoolean { val },
                    }
                }
                LiteralRepresentations::CustomNil { val } => {
                    return Expr::Literal {
                        literal: LiteralRepresentations::CustomNil { val },
                    }
                }
                LiteralRepresentations::CustomNumber { val } => {
                    return Expr::Literal {
                        literal: LiteralRepresentations::CustomNumber { val },
                    }
                }
                LiteralRepresentations::CustomString { val } => {
                    return Expr::Literal {
                        literal: LiteralRepresentations::CustomString { val },
                    }
                }
            },

            Expr::Grouping { expr } => self.eval(*expr),
            Expr::Unary { operator, right } => self.eval_unary(operator, *right),
            Expr::FailScenario { reason } => self.eval_fail_scenario(reason),
        }
    }

    fn eval_bin(&self, left: Box<Expr>, token: Token, right: Box<Expr>) -> Expr {
        todo!()
    }

    fn eval_unary(&mut self, operator: Token, right: Expr) -> Expr {
        let right: Expr = self.eval(right);

        match operator.token_type {
            TokenType::MINUS => match right {
                Expr::Literal {
                    literal: LiteralRepresentations::CustomNumber { val: number },
                } => {
                    return Expr::Literal {
                        literal: LiteralRepresentations::CustomNumber { val: -number },
                    }
                }
                _ => panic!("Could not parse expression in unary evaluation"),
            },
            _ => panic!("WRONG"),
        }
    }

    fn eval_fail_scenario(&self, reason: String) -> Expr {
        todo!()
    }
}
