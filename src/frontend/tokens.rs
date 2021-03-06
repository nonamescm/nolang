#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
    // reserved fields
    Newline,
    Eof,

    // value fields
    Ident(String), // identifier, like: let main = 1
    Num(f64),
    Str(String),

    // reserved keywords
    True,
    False,
    None, // Null value
    Let,  // declare function/variable
    Defn,
    Do,
    If,
    Elif,
    Else,
    Then,
    End, // `;;`
    Or,
    Not,
    And,

    // Symbols
    Semicolon, // `;`
    Minus,
    Plus,
    Asterisk, // `*`
    Slash,    // `/`
    Percent,  // `%`
    Pow,      // `**`
    Rparen,
    Lparen,
    Rbrace,
    Lbrace,
    Assign,      // `=`
    Comp,        // `==`
    Different,   // `~=` or `!=` in other languages
    Gt,          // `>`
    GtOrEq,      // `>=`
    Lt,          // `<`
    LtOrEq,      // `<=`
    Comma,       // `,`
    Point,       // `.`
}

impl std::fmt::Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = |string: &str| string.to_string(); // helper syntax sugar method

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
                Comp => s("=="),
                Different => s("~="),
                Assign => s("="),

                Lparen => s("("),
                Rparen => s(")"),
                Lbrace => s("{"),
                Rbrace => s("}"),

                Comma => s(","),
                Newline => s("newline"),
                Point => s("."),
                Semicolon => s(";"),

                Minus => s("-"),
                Plus => s("+"),
                Asterisk => s("*"),
                Slash => s("/"),
                Percent => s("%"),
                Pow => s("**"),

                Ident(ref l) => l.to_string(),
                Num(ref f) => f.to_string(),
                Str(ref s) => s.to_string(),

                True => s("true"),
                False => s("false"),
                None => s("none"),

                Let => s("let"),
                Defn => s("defn"),
                Do => s("do"),
                End => s(";;"),

                If => s("if"),
                Elif => s("elif"),
                Else => s("else"),
                Then => s("then"),

                Or => s("or"),
                And => s("and"),
                Not => s("not"),
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
                Self::Num(..)
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
        "not" => Some(Tokens::Not),
        "let" => Some(Tokens::Let),
        "defn" => Some(Tokens::Defn),
        "if" => Some(Tokens::If),
        "elif" => Some(Tokens::Elif),
        "else" => Some(Tokens::Else),
        "then" => Some(Tokens::Then),
        "do" => Some(Tokens::Do),
        "true" => Some(Tokens::True),
        "false" => Some(Tokens::False),
        "none" => Some(Tokens::None),
        "and" => Some(Tokens::And),
        "or" => Some(Tokens::Or),
        "end" => Some(Tokens::End), // the same as `;;`
        _ => None,
    }
}
