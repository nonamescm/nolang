#[derive(PartialEq, Debug)]
pub enum Tokens {
    // reserved fields
    Eof,
    Space,
    Newline,

    // value fields
    Ident(Vec<char>), // identifier, like: let main = 1
    Number(f64), // number
    String(String),

    // reserved keywords
    True,
    False,
    None, // Null value
    Let,  // declare function
    Case, // switch-case
    As,
    Return,
    Do,
    End,
    For,
    While,
    Or,
    Not,
    And,
    Ignore,

    // reserved statements
    Write,   // print statement
    Writeln, // println statement

    // Symbols
    Colon,     // `:`
    Semicolon, // `;`
    Minus,
    Plus,
    Asterisk, // `*`
    Slash,    // `/`
    Percent,  // `%`
    Rparen,
    Lparen,
    Rbrace,
    Lbrace,
    Assign,  // `=`
    Comp,    // `==`
    Gt,      // `>`
    Lt,      // `<`
    Pipe,    // `|`
    Dollar,  // `$`
    Comma,   // `,`
    Point,   // `.`
}

pub fn keyword_get_tok(k: &[char]) -> Option<Tokens> {
    match k.iter().collect::<String>().as_str() {
        "not" => Some(Tokens::Not),
        "return" => Some(Tokens::Return),
        "let" => Some(Tokens::Let),
        "case" => Some(Tokens::Case),
        "for" => Some(Tokens::For),
        "while" => Some(Tokens::While),
        "do" => Some(Tokens::Do),
        "end" => Some(Tokens::End),
        "true" => Some(Tokens::True),
        "false" => Some(Tokens::False),
        "none" => Some(Tokens::None),
        "and" => Some(Tokens::And),
        "or" => Some(Tokens::Or),
        "as" => Some(Tokens::As),
        "ignore" => Some(Tokens::Ignore),
        "write" => Some(Tokens::Write),
        "writeln" => Some(Tokens::Writeln),
        _ => None,
    }
}
