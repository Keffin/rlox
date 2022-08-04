use std::{
    fs,
    io::{stdin, stdout, Write},
};

pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
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
            self.run(&prettify_input);
            self.had_error = false;
        }
    }

    pub fn run_file(&self, file_path: &String) {
        let content: String = fs::read_to_string(file_path).expect("Err while reading file");
        self.run(&content);

        if self.had_error {
            std::process::exit(65);
        }
    }

    pub fn run(&self, line: &String) {
        print!("{}", line)
        //todo!("To be implemented");
    }

    pub fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message);
        self.had_error = true;
    }

    fn report(&self, line: u32, loc: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, loc, message);
    }
}
