use std::env;
use std::fs;
use std::time::Instant;

mod interpreter;
mod lexer;
mod parser;

use interpreter::interpreter::run;
use lexer::tokenizer::tokenize;
use parser::parser::parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Pwease give me a file to run! 🥺👉👈");
        std::process::exit(1);
    }

    let start_time = Instant::now();

    let file_name = &args[1];
    let code = fs::read_to_string(file_name).expect("Failed to read the file");
    let tokens = tokenize(&code);
    let ast = parse(&tokens);
    run(&ast);

    let duration = start_time.elapsed();
    println!("\nScript ran in: {:.3?}", duration);
}
