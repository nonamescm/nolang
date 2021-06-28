#[derive(PartialEq, Debug)]
pub enum Tokens {
    // reserved fields
    Space,
    Newline,

    // value fields
    LocalIdent(String), // local identifier, like: $symbol = 20
    Ident(String),      // identifier, like: let main = 1
    Number(f64),        // number
    String(String),

    // reserved keywords
    True,
    False,
    None, // Null value
    Let,  // declare function/variable
    Defun,
    Case, // switch-case
    As,
    In,
    Return,
    Do,
    End,
    For,
    While,
    Or,
    Not,
    And,

    // reserved statements
    Write,   // print statement
    Writeln, // println statement

    // Symbols
    Semicolon, // `;`
    Interrogation, // `?`
    Minus,
    Plus,
    Asterisk, // `*`
    Slash,    // `/`
    Percent,  // `%`
    Rparen,
    Lparen,
    Rbrace,
    Lbrace,
    Assign, // `=`
    Comp,   // `==`
    Gt,     // `>`
    Lt,     // `<`
    Pipe,   // `|`
    Comma,  // `,`
    Point,  // `.`
    Concat, // `..`
    Underline, // `_` used as statement on patterns
}

pub fn keyword_get_tok(k: &str) -> Option<Tokens> {
    match k {
        "defun" => Some(Tokens::Defun),
        "in" => Some(Tokens::In),
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
        "write" => Some(Tokens::Write),
        "writeln" => Some(Tokens::Writeln),
        "_" => Some(Tokens::Underline),
        _ => None,
    }
}
