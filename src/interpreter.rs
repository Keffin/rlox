use crate::expr::Expr::Literal;
use crate::expr::{self, CustomBoolean, Expr, LiteralRepresentations};
use crate::token::Token;

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
                LiteralRepresentations::CustomBoolean { val } => Expr::Literal {
                    literal: LiteralRepresentations::CustomBoolean { val },
                },
                LiteralRepresentations::CustomNil { val } => Expr::Literal {
                    literal: LiteralRepresentations::CustomNil { val },
                },
                LiteralRepresentations::CustomNumber { val } => Expr::Literal {
                    literal: LiteralRepresentations::CustomNumber { val },
                },
                LiteralRepresentations::CustomString { val } => Expr::Literal {
                    literal: LiteralRepresentations::CustomString { val },
                },
            },

            Expr::Grouping { expr } => self.eval(*expr),
            Expr::Unary { operator, right } => self.eval_unary(operator, right),
            Expr::FailScenario { reason } => self.eval_fail_scenario(reason),
        }
    }

    fn eval_bin(&self, left: Box<Expr>, token: Token, right: Box<Expr>) -> Expr {
        todo!()
    }

    fn eval_unary(&self, operator: Token, right: Box<Expr>) -> Expr {
        todo!()
    }

    fn eval_fail_scenario(&self, reason: String) -> Expr {
        todo!()
    }
}
