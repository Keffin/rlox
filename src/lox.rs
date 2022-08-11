use std::{
    fs,
    io::{stdin, stdout, Write},
};

use crate::{
    expr::Expr,
    interpreter::{self, Interpreter, RuntimeError},
    parser::Parser,
    token::Token,
};
use crate::{scanner::Scanner, token_type::TokenType};

pub struct Lox {
    pub had_error: bool,
    pub had_runtime_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            had_error: false,
            had_runtime_error: false,
        }
    }

    pub fn repl(&mut self) {
        loop {
            print!("> ");
            let _ = stdout().flush();
            let mut input: String = String::new();
            stdin()
                .read_line(&mut input)
                .expect("Err while reading input line");
            let prettify_input: String = input.trim().to_string();
            if prettify_input == ".exit" {
                std::process::exit(1);
            }
            println!("input: {}", prettify_input);
            self.run(prettify_input);
            self.had_error = false;
        }
    }

    pub fn run_file(&self, file_path: &String) {
        let content: String = fs::read_to_string(file_path).expect("Err while reading file");
        self.run(content);

        if self.had_error {
            std::process::exit(65);
        }

        if self.had_runtime_error {
            std::process::exit(70);
        }
    }

    pub fn run(&self, source: String) {
        let mut scanner: Scanner = Scanner::new(source);
        let tokens: &Vec<Token> = scanner.scan_tokens();
        let mut parser: Parser = Parser::new(tokens.to_vec());
        let expr: Expr = parser.parse();

        let mut interpreter: Interpreter = Interpreter::new();

        let res = interpreter.interpret(expr).unwrap();
        println!("{:#?}", res);
    }

    pub fn parser_error(&mut self, token: Token, message: &str) {
        self.had_error = true;
        if token.token_type == TokenType::EOF {
            self.report(token.line, "at end", message)
        } else {
            let loc_msg: String = format!("at '{}'", token.lexeme);
            self.report(token.line, &loc_msg, message)
        }
    }

    pub fn runtime_error(&mut self, err: RuntimeError) {
        println!("{} \n [line {} ]", err.reason, err.token.line);
        self.had_runtime_error = true;
    }

    pub fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message);
        self.had_error = true;
    }

    fn report(&self, line: u32, loc: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, loc, message);
    }
}
