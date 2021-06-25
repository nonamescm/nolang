mod lexer;
use lexer::Lexer;
use std::io::{
    stdout,
    stdin,
    Write,
};

fn main() -> Result<(), std::io::Error> {
    loop {
        print!("NoLang(REPL)> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        let mut lexer = Lexer::new(input.chars().collect());

        println!("[");
        for element in lexer.start().iter() {
            println!("  {:?},", element)
        }
        println!("]");
    }
}
