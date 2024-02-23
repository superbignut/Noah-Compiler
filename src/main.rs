use std::fs;
mod lexer;
use lexer::scanner::Scanner;

fn run_file(file_path: &String) -> Result<(), String> {
    let contents = fs::read_to_string(file_path).unwrap();
    //println!("{}", contents);

    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

// #[derive(Clone)]
// struct test {
//     aaa: String,
// }

fn main() {
    let res = run_file(&String::from("sources/test.cpp"));
}
