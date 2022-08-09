use std::borrow::Borrow;
use std::error::Error;
use std::marker::PhantomData;

use crate::expr::{self, CustomBoolean, CustomNumber, Expr, Literal, LiteralRepresentations};
use crate::token::Token;
use crate::token_type::TokenType;

struct Interpreter {
    test: String,
}

#[derive(Debug)]
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
            } => self.eval_binary(*left, operator, *right),

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

    fn eval_binary<'a>(&mut self, left: Expr, operator: Token, right: Expr) -> RLoxEvalResult {
        let left: RLoxEvalResult = self.eval(left);
        let right: RLoxEvalResult = self.eval(right);

        match operator.token_type {
            TokenType::MINUS => {
                let left_expr = left.unwrap();
                let right_expr = right.unwrap();
                let x = self.binary_evaluation(&left_expr, &right_expr)?;
                let left_num = x.0 .0;
                let right_num = x.1 .0;

                return Ok(Expr::Literal {
                    literal: LiteralRepresentations::CustomNumber {
                        val: left_num - right_num,
                    },
                });
            }

            TokenType::PLUS => {
                let left_expr = left.unwrap();
                let right_expr = right.unwrap();
                let x = self.binary_evaluation(&left_expr, &right_expr)?;

                //let m = left_expr.clone();
                //let n = right_expr.clone();

                match (left_expr, right_expr) {
                    (
                        Expr::Literal {
                            literal: LiteralRepresentations::CustomNumber { val: l },
                        },
                        Expr::Literal {
                            literal: LiteralRepresentations::CustomNumber { val: r },
                        },
                    ) => {
                        let left_num = x.0 .0;
                        let right_num = x.1 .0;

                        return Ok(Expr::Literal {
                            literal: LiteralRepresentations::CustomNumber {
                                val: left_num + right_num,
                            },
                        });
                    }

                    (
                        Expr::Literal {
                            literal: LiteralRepresentations::CustomString { val: l },
                        },
                        Expr::Literal {
                            literal: LiteralRepresentations::CustomString { val: r },
                        },
                    ) => {
                        let left_str = x.0 .2;
                        let right_str = x.1 .2;

                        return Ok(Expr::Literal {
                            literal: LiteralRepresentations::CustomString {
                                val: format!("{}{}", left_str, right_str),
                            },
                        });
                    }
                    _ => panic!("fff"),
                };
            }

            TokenType::SLASH => {
                let left_expr = left.unwrap();
                let right_expr = right.unwrap();
                let x = self.binary_evaluation(&left_expr, &right_expr)?;
                let left_num = x.0 .0;
                let right_num = x.1 .0;

                return Ok(Expr::Literal {
                    literal: LiteralRepresentations::CustomNumber {
                        val: left_num / right_num,
                    },
                });
            }

            _ => Err(InterpreterError {
                reason: "Fail".to_string(),
            }),
        }
    }

    fn binary_evaluation(
        &self,
        left: &Expr,
        right: &Expr,
    ) -> Result<((f64, bool, String), (f64, bool, String)), InterpreterError> {
        let left_expr = left;
        let right_expr = right;
        /*let left_expr = match left {
            Ok(ex) => ex,
            Err(_) => {
                return Err(InterpreterError {
                    reason: "Failed to interpret left expression, binary eval".to_string(),
                })
            }
        };

        let right_expr: Expr = match right {
            Ok(ex) => ex,
            Err(_) => {
                return Err(InterpreterError {
                    reason: "Failed to interpret right expression, binary eval".to_string(),
                })
            }
        };*/

        let left_lit = self.parse_expr(left_expr)?;
        let right_lit = self.parse_expr(right_expr)?;

        return Ok((
            self.get_val_from_literal(left_lit),
            self.get_val_from_literal(right_lit),
        ));
    }

    fn get_val_from_literal(&self, l: LiteralRepresentations) -> (f64, bool, String) {
        match l {
            LiteralRepresentations::CustomNumber { val } => (val, false, "null".to_string()),
            LiteralRepresentations::CustomBoolean { val } => (0.0, val, "null".to_string()),
            LiteralRepresentations::CustomString { val } => (0.0, false, val),
            _ => panic!(),
        }
    }

    fn parse_expr(&self, expr: &Expr) -> Result<LiteralRepresentations, InterpreterError> {
        match expr {
            Expr::Literal { literal } => match literal {
                LiteralRepresentations::CustomBoolean { val } => {
                    Ok(LiteralRepresentations::CustomBoolean { val: *val })
                }
                LiteralRepresentations::CustomNil { val } => {
                    Ok(LiteralRepresentations::CustomNil {
                        val: "Fail".to_string(),
                    })
                }

                LiteralRepresentations::CustomNumber { val } => {
                    Ok(LiteralRepresentations::CustomNumber { val: *val })
                }
                LiteralRepresentations::CustomString { val } => {
                    Ok(LiteralRepresentations::CustomString {
                        val: String::from(val),
                    })
                }

                _ => Err(InterpreterError {
                    reason: "Incorrect literal representation reached".to_string(),
                }),
            },
            _ => Err(InterpreterError {
                reason: "Only parsing literals in this method.".to_string(),
            }),
        }
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
