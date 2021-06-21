mod scanner;
use scanner::Scanner;

fn main() {
    let input = "let main = do writeln 'oi' end".chars().collect();
    let mut lexer = Scanner::new(input);

    for v in lexer.start().into_iter() {
        println!("{:?}", v)
    }
}
