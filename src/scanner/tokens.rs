#![allow(dead_code)]
#[derive(PartialEq, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Tokens {
    // reserved fields
    EOF,
    SPACE,
    NEWLINE,

    // value fields
    IDENT(Vec<char>), // identifier, like: let main = 1
    NUM(f64), // number
    STRING(Vec<char>),

    // reserved keywords
    TRUE,
    FALSE,
    NONE, // Null value
    LET,  // declare function
    CASE, // switch-case
    AS,
    RETURN,
    DO,
    END,
    FOR,
    WHILE,
    OR,
    AND,
    IGNORE,

    // reserved statements
    WRITE,   // print statement
    WRITELN, // println statement

    // Symbols
    COLON,     // `:`
    SEMICOLON, // `;`
    MINUS,
    PLUS,
    ASTERISK, // `*`
    SLASH,    // `/`
    PERCENT,  // `%`
    RPAREN,
    LPAREN,
    RBRACE,
    LBRACE,
    ASSIGN,  // `=`
    COMP,    // `==`
    GT,      // `>`
    LT,      // `<`
    PIPE,    // `|`
    COMMENT, // `@`
    DOLLAR,  // `$`
    COMMA,   // `,`
    POINT, // `.`
}

pub fn keyword_get_tok(k: &[char]) -> Option<Tokens> {
    match k.iter().collect::<String>().as_str() {
        "let" => Some(Tokens::LET),
        "case" => Some(Tokens::CASE),
        "for" => Some(Tokens::FOR),
        "while" => Some(Tokens::WHILE),
        "do" => Some(Tokens::DO),
        "end" => Some(Tokens::END),
        "true" => Some(Tokens::TRUE),
        "false" => Some(Tokens::FALSE),
        "none" => Some(Tokens::NONE),
        "and" => Some(Tokens::AND),
        "or" => Some(Tokens::OR),
        "as" => Some(Tokens::AS),
        "ignore" => Some(Tokens::IGNORE),
        "write" => Some(Tokens::WRITE),
        "writeln" => Some(Tokens::WRITELN),
        _ => None,
    }
}
