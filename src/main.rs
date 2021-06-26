mod lexer;
use lexer::Lexer;
use std::{
    io::{
        stdout,
        stdin,
        Write,
    },
    env::args,
    fs::read_to_string,
};

type IoResult = Result<(), std::io::Error>;

fn main() -> IoResult {
    if args().collect::<Vec<String>>().len() < 2 {
        repl()?;
    } else {
        interpret()?;
    }
    Ok(())
}

fn interpret() -> IoResult {
    let mut arguments = args();
    arguments.next();

    for file in arguments {
        println!("{}", file);
        let lexer = Lexer::new(
            read_to_string(file)?.chars().collect()
        ).start();

        if !lexer.is_empty() {
            println!("[");
            for element in lexer.iter() {
                println!("  {:?},", element)
            }
            println!("]");
        }
    }
    Ok(())
}

fn repl() -> IoResult {
    loop {
        print!("NoLang(REPL)> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        let lexer = Lexer::new(input.chars().collect()).start();

        if !lexer.is_empty() {
            println!("[");
            for element in lexer.iter() {
                println!("  {:?},", element)
            }
            println!("]");
        }
    }
}
