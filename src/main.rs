mod scanner;
use scanner::Scanner;
use std::io::{
    stdout,
    stdin,
    Write,
};

fn main() -> Result<(), std::io::Error> {
    loop {
        print!(">>> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        let mut lexer = Scanner::new(input.chars().collect());

        for v in lexer.start().into_iter() {
            println!("{:?}", v)
        }
    }
}
