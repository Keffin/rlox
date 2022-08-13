use crate::environment::Environment;
use crate::expr::{self, CustomBoolean, CustomNumber, Expr, Literal, LiteralRepresentations};
use crate::interpreter_objects::InterpretedParsed;
use crate::stmt::{Expression, Print, Stmt, Var};
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Interpreter {
    environment: Environment,
}

#[derive(Debug)]
pub struct InterpreterError {
    reason: String,
}

// TODO: Re-implement error structs better
pub struct RuntimeError {
    pub reason: String,
    pub token: Token,
}

type RLoxEvalResult = Result<Expr, InterpreterError>;

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret_stmts(&mut self, statements: Vec<Stmt>) -> Result<(), &str> {
        // TODO: Need to fix this
        for expr in statements {
            let x = self.eval_stmt(expr);

            println!("{:#?}", x.unwrap());
        }
        Ok(())
    }

    pub fn interpret(&mut self, expr: Expr) -> RLoxEvalResult {
        return self.eval(expr);
    }

    fn eval_stmt(&mut self, expr: Stmt) -> RLoxEvalResult {
        match expr {
            Stmt::Expression(Expression { expression: expr }) => {
                return self.eval(expr);
            }
            Stmt::Print(Print {
                print_expression: expr,
            }) => {
                return self.eval(expr);
            }
            Stmt::Var(Var { name, initializer }) => match initializer {
                Some(initializer) => {
                    let expr = self.eval(initializer)?;

                    match self.parse_expr(&expr) {
                        Ok(value) => {
                            self.environment.define(name.lexeme, value);
                            return Ok(expr);
                        }
                        Err(_) => {
                            return Err(InterpreterError {
                                reason: format!(
                                    "Failed to fetch literal from expression {:#?}",
                                    expr
                                ),
                            })
                        }
                    }
                }
                None => {
                    let null_expr: Expr = Expr::Literal {
                        literal: LiteralRepresentations::CustomNil {
                            val: "Null".to_string(),
                        },
                    };
                    self.environment.define(
                        name.lexeme,
                        LiteralRepresentations::CustomNil {
                            val: "Null".to_string(),
                        },
                    );
                    return Ok(null_expr);
                }
            },
        }
    }

    fn eval(&mut self, expr: Expr) -> RLoxEvalResult {
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
            Expr::Variable { name } => {
                let literal_value: LiteralRepresentations = self.environment.get(name).unwrap();

                Ok(self.convert_literal_to_expr(literal_value))
            }
        }
    }

    fn eval_binary(&mut self, left: Expr, operator: Token, right: Expr) -> RLoxEvalResult {
        let left: RLoxEvalResult = self.eval(left);
        let right: RLoxEvalResult = self.eval(right);

        let left_expr = left.unwrap();
        let right_expr = right.unwrap();

        let x: (InterpretedParsed, InterpretedParsed) =
            self.binary_evaluation(&left_expr, &right_expr)?;

        match operator.token_type {
            TokenType::BANGEQUAL => {
                return Ok(Expr::Literal {
                    literal: LiteralRepresentations::CustomBoolean {
                        val: !self.is_equal(left_expr, right_expr),
                    },
                })
            }

            TokenType::EQUALEQUAL => {
                return Ok(Expr::Literal {
                    literal: LiteralRepresentations::CustomBoolean {
                        val: self.is_equal(left_expr, right_expr),
                    },
                })
            }

            TokenType::GREATER => {
                self.check_number_operands(operator, left_expr, right_expr)?;
                let left_num: f64 = self.fetch_numeric_value(x.0)?;
                let right_num: f64 = self.fetch_numeric_value(x.1)?;

                return self.fetch_boolean_evaluation(TokenType::GREATER, left_num, right_num);
            }

            TokenType::GREATEREQUAL => {
                self.check_number_operands(operator, left_expr, right_expr)?;
                let left_num: f64 = self.fetch_numeric_value(x.0)?;
                let right_num: f64 = self.fetch_numeric_value(x.1)?;

                return self.fetch_boolean_evaluation(TokenType::GREATEREQUAL, left_num, right_num);
            }

            TokenType::LESS => {
                self.check_number_operands(operator, left_expr, right_expr)?;
                let left_num: f64 = self.fetch_numeric_value(x.0)?;
                let right_num: f64 = self.fetch_numeric_value(x.1)?;

                return self.fetch_boolean_evaluation(TokenType::LESS, left_num, right_num);
            }

            TokenType::LESSEQUAL => {
                self.check_number_operands(operator, left_expr, right_expr)?;
                let left_num: f64 = self.fetch_numeric_value(x.0)?;
                let right_num: f64 = self.fetch_numeric_value(x.1)?;

                return self.fetch_boolean_evaluation(TokenType::LESSEQUAL, left_num, right_num);
            }

            TokenType::MINUS => {
                self.check_number_operands(operator, left_expr, right_expr)?;
                let left_num = self.fetch_numeric_value(x.0)?;
                let right_num = self.fetch_numeric_value(x.1)?;

                return Ok(Expr::Literal {
                    literal: LiteralRepresentations::CustomNumber {
                        val: left_num - right_num,
                    },
                });
            }

            TokenType::PLUS => {
                match (left_expr, right_expr) {
                    (
                        Expr::Literal {
                            literal: LiteralRepresentations::CustomNumber { val: _l },
                        },
                        Expr::Literal {
                            literal: LiteralRepresentations::CustomNumber { val: _r },
                        },
                    ) => {
                        let left_num = self.fetch_numeric_value(x.0)?;
                        let right_num = self.fetch_numeric_value(x.1)?;

                        return Ok(Expr::Literal {
                            literal: LiteralRepresentations::CustomNumber {
                                val: left_num + right_num,
                            },
                        });
                    }

                    (
                        Expr::Literal {
                            literal: LiteralRepresentations::CustomString { val: _l },
                        },
                        Expr::Literal {
                            literal: LiteralRepresentations::CustomString { val: _r },
                        },
                    ) => {
                        let left_str = self.fetch_stringified_value(x.0)?;
                        let right_str = self.fetch_stringified_value(x.1)?;

                        return Ok(Expr::Literal {
                            literal: LiteralRepresentations::CustomString {
                                val: format!("{}{}", left_str, right_str),
                            },
                        });
                    }
                    _ => {
                        return Err(InterpreterError {
                            reason: "PLUS operation failed, can only add 2 numbers or 2 strings"
                                .to_string(),
                        });
                    }
                };
            }

            TokenType::SLASH => {
                self.check_number_operands(operator, left_expr, right_expr)?;
                let left_num = self.fetch_numeric_value(x.0)?;
                let right_num = self.fetch_numeric_value(x.1)?;

                return Ok(Expr::Literal {
                    literal: LiteralRepresentations::CustomNumber {
                        val: left_num / right_num,
                    },
                });
            }

            TokenType::STAR => {
                self.check_number_operands(operator, left_expr, right_expr)?;
                let left_num = self.fetch_numeric_value(x.0)?;
                let right_num = self.fetch_numeric_value(x.1)?;

                return Ok(Expr::Literal {
                    literal: LiteralRepresentations::CustomNumber {
                        val: left_num * right_num,
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
    ) -> Result<(InterpretedParsed, InterpretedParsed), InterpreterError> {
        let left_expr = left;
        let right_expr = right;

        let left_lit = self.parse_expr(left_expr)?;
        let right_lit = self.parse_expr(right_expr)?;

        return Ok((
            self.get_val_from_literal(left_lit),
            self.get_val_from_literal(right_lit),
        ));
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
                    self.check_number_operand(operator, right.unwrap())?;

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

    // Utilities

    fn check_number_operand(&self, operator: Token, operand: Expr) -> Result<(), InterpreterError> {
        match operand {
            Expr::Literal {
                literal: LiteralRepresentations::CustomNumber { val: _ },
            } => Ok(()),
            _ => Err(InterpreterError {
                reason: format!("Operand must be a number, operator is {:#?}", operator),
            }),
        }
    }

    fn check_number_operands(
        &self,
        operator: Token,
        left: Expr,
        right: Expr,
    ) -> Result<(), InterpreterError> {
        match (left, right) {
            (
                Expr::Literal {
                    literal: LiteralRepresentations::CustomNumber { val: _ },
                },
                Expr::Literal {
                    literal: LiteralRepresentations::CustomNumber { val: _ },
                },
            ) => Ok(()),
            _ => Err(InterpreterError {
                reason: format!("Operands must be numbers, operator is {:#?}", operator),
            }),
        }
    }

    fn convert_literal_to_expr(&self, literal: LiteralRepresentations) -> Expr {
        match literal {
            LiteralRepresentations::CustomBoolean { val } => {
                return Expr::Literal {
                    literal: LiteralRepresentations::CustomBoolean { val: val },
                }
            }
            LiteralRepresentations::CustomNil { val } => {
                return Expr::Literal {
                    literal: LiteralRepresentations::CustomNil { val: val },
                }
            }
            LiteralRepresentations::CustomNumber { val } => {
                return Expr::Literal {
                    literal: LiteralRepresentations::CustomNumber { val: val },
                }
            }
            LiteralRepresentations::CustomString { val } => {
                return Expr::Literal {
                    literal: LiteralRepresentations::CustomString { val: val },
                }
            }
        }
    }

    fn fetch_numeric_value(
        &self,
        interpreted_value: InterpretedParsed,
    ) -> Result<f64, InterpreterError> {
        if let InterpretedParsed::IntepretedNum { value } = interpreted_value {
            return Ok(value);
        } else {
            return Err(InterpreterError {
                reason: "Failed to fetch numeric value".to_string(),
            });
        }
    }

    fn fetch_stringified_value(
        &self,
        interpreted_value: InterpretedParsed,
    ) -> Result<String, InterpreterError> {
        if let InterpretedParsed::InterpretedStr { value } = interpreted_value {
            return Ok(value);
        } else {
            return Err(InterpreterError {
                reason: "Failed to fetch String".to_string(),
            });
        }
    }

    fn fetch_boolean_evaluation(
        &self,
        operator_token_type: TokenType,
        left_num: f64,
        right_num: f64,
    ) -> RLoxEvalResult {
        match operator_token_type {
            TokenType::GREATER => {
                return Ok(Expr::Literal {
                    literal: LiteralRepresentations::CustomBoolean {
                        val: left_num > right_num,
                    },
                })
            }

            TokenType::GREATEREQUAL => {
                return Ok(Expr::Literal {
                    literal: LiteralRepresentations::CustomBoolean {
                        val: left_num >= right_num,
                    },
                })
            }

            TokenType::LESS => {
                return Ok(Expr::Literal {
                    literal: LiteralRepresentations::CustomBoolean {
                        val: left_num < right_num,
                    },
                })
            }

            TokenType::LESSEQUAL => {
                return Ok(Expr::Literal {
                    literal: LiteralRepresentations::CustomBoolean {
                        val: left_num <= right_num,
                    },
                })
            }

            _ => {
                return Err(InterpreterError {
                    reason: "Failed, only expecting boolean comparisions".to_string(),
                })
            }
        }
    }

    fn is_equal(&self, left: Expr, right: Expr) -> bool {
        match (left, right) {
            (
                Expr::Literal {
                    literal: LiteralRepresentations::CustomNil { val: _ },
                },
                Expr::Literal {
                    literal: LiteralRepresentations::CustomNil { val: _ },
                },
            ) => return true,
            (
                Expr::Literal {
                    literal: LiteralRepresentations::CustomString { val: l },
                },
                Expr::Literal {
                    literal: LiteralRepresentations::CustomString { val: r },
                },
            ) => return l == r,
            (
                Expr::Literal {
                    literal: LiteralRepresentations::CustomBoolean { val: l },
                },
                Expr::Literal {
                    literal: LiteralRepresentations::CustomBoolean { val: r },
                },
            ) => return l == r,
            (
                Expr::Literal {
                    literal: LiteralRepresentations::CustomNumber { val: l },
                },
                Expr::Literal {
                    literal: LiteralRepresentations::CustomNumber { val: r },
                },
            ) => return l == r,
            _ => return false,
        }
    }

    fn get_val_from_literal(&self, l: LiteralRepresentations) -> InterpretedParsed {
        match l {
            LiteralRepresentations::CustomNumber { val } => {
                InterpretedParsed::IntepretedNum { value: val }
            }
            LiteralRepresentations::CustomBoolean { val } => {
                InterpretedParsed::InterpretedBool { value: val }
            }
            LiteralRepresentations::CustomString { val } => {
                InterpretedParsed::InterpretedStr { value: val }
            }
            _ => panic!(),
        }
    }

    fn parse_expr(&self, expr: &Expr) -> Result<LiteralRepresentations, InterpreterError> {
        match expr {
            Expr::Literal { literal } => match literal {
                LiteralRepresentations::CustomBoolean { val } => {
                    Ok(LiteralRepresentations::CustomBoolean { val: *val })
                }
                LiteralRepresentations::CustomNil { val: _ } => {
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

    fn is_truthy(&self, literal_expr: Literal) -> bool {
        match literal_expr {
            Literal {
                literal: LiteralRepresentations::CustomNil { val: _ },
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
