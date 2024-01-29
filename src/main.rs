use std::fs;
mod lexer;
use lexer::scanner::Scanner;

fn run_file(file_path: &String) -> Result<(), String> {
    let contents = fs::read_to_string(file_path).unwrap();

    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

fn main() {
    let _ = run_file(&String::from("sources/test.cpp"));
}
