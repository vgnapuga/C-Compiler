mod ast;
mod parser;

use std::io::Read;
use std::{env, fs, process};

fn main() {
    let source = match env::args().nth(1) {
        Some(path) => fs::read_to_string(&path).unwrap_or_else(|e| {
            eprintln!("Не удалось открыть '{path}': {e}");
            process::exit(1);
        }),
        None => {
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        }
    };

    match parser::parse_program(&source) {
        Ok(program) => {
            program.print_tree();
            println!("\n <== OK ==>")
        }
        Err(e) => {
            eprintln!("Синтаксическая ошибка:\n{e}");
            process::exit(1)
        }
    }
}
