#[derive(Debug, PartialEq, Clone)]
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
    Different, // `~=` or `!=` in other languages
    Gt,     // `>`
    Lt,     // `<`
    Pipe,   // `|`
    Comma,  // `,`
    Point,  // `.`
    Concat, // `..`
    Underline, // `_` used as statement on patterns
}

impl Tokens {
    pub fn is_operator(&self) -> bool {
        matches!(
            *self,
            Self::Comp
            |Self::Different
            |Self::Gt
            |Self::Lt
            |Self::Plus
            |Self::Minus
            |Self::Slash
            |Self::Asterisk
        )
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            *self,
            Self::Number(_)
            |Self::String(_)
            |Self::True
            |Self::False
            |Self::None
        )
    }

    pub fn is_unary(&self) -> bool {
        matches!(
            *self,
            Self::Not
            |Self::Minus
        )
    }

    pub fn is_ident(&self) -> bool {
        matches!(
            *self,
            Self::LocalIdent(_)
            |Self::Ident(_)
        )
    }
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
