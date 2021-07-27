#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
    // reserved fields
    Newline,
    Eof,

    // value fields
    Ident(String),      // identifier, like: let main = 1
    Number(f64),        // number
    Str(String),

    // reserved keywords
    True,
    False,
    None, // Null value
    Let,  // declare function/variable
    In,
    Return,
    Do,
    If,
    Elif,
    Else,
    Done,
    End, // `;;`
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
    Assign,      // `=`
    ArrowAssign, // `=>`
    Comp,        // `==`
    Different,   // `~=` or `!=` in other languages
    Gt,          // `>`
    GtOrEq,      // `>=`
    Lt,          // `<`
    LtOrEq,      // `<=`
    Pipe,        // `|`
    Comma,       // `,`
    Point,       // `.`
    Concat,      // `..`
    Underline,   // `_` used as statement on patterns
}

impl std::fmt::Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = |string: &str| string.to_owned(); // helper syntax sugar method

        use Tokens::*;
        write!(
            f,
            "{}",
            match *self {
                Eof => s("<EOF>"),
                Gt => s(">"),
                Lt => s("<"),
                GtOrEq => s(">="),
                LtOrEq => s("<="),
                ArrowAssign => s("=>"),
                Comp => s("=="),
                Different => s("~="),
                Assign => s("="),

                Lparen => s("("),
                Rparen => s(")"),
                Lbrace => s("{"),
                Rbrace => s("}"),

                Pipe => s("|"),
                Comma => s(","),
                Underline => s("_"),
                Newline => s("newline"),
                Point => s("."),
                Semicolon => s(";"),

                Minus => s("-"),
                Plus => s("+"),
                Asterisk => s("*"),
                Slash => s("/"),
                Percent => s("%"),
                Concat => s(".."),

                Ident(ref l) => l.to_string(),
                Number(n) => n.to_string(),
                Str(ref s) => s.to_string(),

                True => s("true"),
                False => s("false"),
                None => s("none"),

                Let => s("let"),
                In => s("in"),
                Return => s("return"),
                Do => s("do"),
                Done => s("done"),
                End => s(";;"),

                For => s("for"),
                While => s("while"),
                If => s("if"),
                Elif => s("elif"),
                Else => s("else"),

                Or => s("or"),
                And => s("and"),
                Not => s("not"),

                Write => s("write"),
                Writeln => s("writeln"),
            }
        )
    }
}

#[allow(dead_code)]
impl Tokens {
    pub fn is_operator(&self) -> bool {
        matches!(
            *self,
            Self::Comp
                | Self::Different
                | Self::Gt
                | Self::Lt
                | Self::Plus
                | Self::Minus
                | Self::Slash
                | Self::Asterisk
        )
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            *self,
            Self::Number(..)
                | Self::Str(..)
                | Self::True
                | Self::False
                | Self::None
                | Self::Ident(..)
        )
    }

    pub fn is_unary(&self) -> bool {
        matches!(*self, Self::Not | Self::Minus)
    }

    pub fn is_ident(&self) -> bool {
        matches!(*self, Self::Ident(_))
    }

    pub fn is_comparison(&self) -> bool {
        matches!(*self, Self::Comp | Self::Different | Self::Gt | Self::Lt)
    }
}

pub fn keyword_get_tok(k: &str) -> Option<Tokens> {
    match k {
        "in" => Some(Tokens::In),
        "not" => Some(Tokens::Not),
        "return" => Some(Tokens::Return),
        "let" => Some(Tokens::Let),
        "for" => Some(Tokens::For),
        "while" => Some(Tokens::While),
        "if" => Some(Tokens::If),
        "elif" => Some(Tokens::Elif),
        "else" => Some(Tokens::Else),
        "do" => Some(Tokens::Do),
        "done" => Some(Tokens::Done),
        "true" => Some(Tokens::True),
        "false" => Some(Tokens::False),
        "none" => Some(Tokens::None),
        "and" => Some(Tokens::And),
        "or" => Some(Tokens::Or),
        "write" => Some(Tokens::Write),
        "writeln" => Some(Tokens::Writeln),
        "_" => Some(Tokens::Underline),
        _ => None,
    }
}
