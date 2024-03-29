use std::fs;
mod lexer;
use lexer::interpreter::Interpreter;
use lexer::parser::Parser;
use lexer::scanner::Scanner;

fn run_file(file_path: &String) -> Result<(), String> {
    let contents = fs::read_to_string(file_path).unwrap();

    let mut scan = Scanner::new(contents);

    let tok = scan.scan_tokens().unwrap();

    let pas = Parser::new(tok).parse().unwrap();

    //    dbg!(pas);

    let _ = Interpreter::new().interpreter(&pas)?; // return 1 ???
    Ok(())
}

fn main() {
    match run_file(&String::from("test.py")) {
        Ok(()) => {
            println!("[     PASS!    ] ---> Compile Successfully!!!");
        }
        Err(v) => {
            println!("[    Error!    ] ---> {}", v);
        }
    }
}

/*
Todo:
Organize the overall logic of function definition and functon calling parts.Specifically:
   1 -----> interperter's environment;
   2 -----> function's closure;
   3 -----> environment's enclosing;
which is so confused and need to be reorganized.
*/
