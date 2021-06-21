mod scanner;
use scanner::Scanner;

fn main() {
    let input = "@ comentario
let main = do 
    writeln '
oi
tudo bem
flw
'
end
@ outro comentario"
        .chars()
        .collect();
    let mut lexer = Scanner::new(input);

    for v in lexer.start().into_iter() {
        println!("{:?}", v)
    }
}
