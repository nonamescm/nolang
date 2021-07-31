pub fn is_ch_valid(c: &char) -> bool {
    c.is_ascii_alphanumeric() || c == &'_'
}

pub fn is_valid_math_symbol(expr: &char) -> bool {
    expr.is_numeric() || expr == &'.'
}

#[macro_export]
macro_rules! get_val {
    ($self:expr; $cond:expr => $create:ident) => {
        let mut $create = String::new();
        loop {
            if !$cond {
                break;
            }
            $create.push($self.ch);
            let pos = $self.pos;
            $self.next();

            if pos == $self.pos {
                crate::error!("ParserError"; "unclosed delimiter at line {}", $self.line => 1)
            }
        }
        $self.back()
    };
}

#[macro_export]
macro_rules! get_str {
    ($self:expr; $cond:expr => $create:ident) => {
        let mut $create = String::new();
        loop {
            if !$cond {
                break;
            }
            $create.push(match $self.ch {
                '\\' => {
                    $self.next();
                    match $self.ch {
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        '0' => '\0',
                        '\\' => '\\',
                        '\'' => '\'',
                        '"' => '"',
                        c => crate::error!("StrError"; "unkown escape sequence '\\{}' on line {}", c, $self.line => 1)
                    }
                },
                ch => ch
            });
            let pos = $self.pos;
            $self.next();

            if pos == $self.pos {
                crate::error!("ParserError"; "unclosed delimiter at line {}", $self.line => 1)
            }
        }
        $self.back()
    };
}
