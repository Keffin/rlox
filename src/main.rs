use std::{
    env, fs,
    io::{stdin, stdout, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args.len());
    if args.len() == 1 {
        println!("Arg len 1 => Run prompt!");
        repl();
    } else if args.len() == 2 {
        println!("Arg len 2 => Run using file input");
        let content: String = fs::read_to_string(&args[1]).expect("Err while reading file");
        run(&content);
    } else {
        println!("Arg len invalid => exiting program..");
    }
}

fn repl() {
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
        run(&prettify_input);
    }
}

fn run(line: &String) {
    print!("{}", line)
    //todo!("To be implemented");
}
