pub fn is_ch_valid(c: &char) -> bool {
    c.is_alphabetic() || c == &'_'
}

pub fn is_valid_math_symbol(expr: &char) -> bool {
    expr.is_numeric() || expr == &'.'
}

#[macro_export]
macro_rules! get_val {
    ($self:expr; $cond:expr => $create:ident) => {
        let mut $create = String::new();
        loop {
            if $cond {
                break;
            }
            $create.push($self.ch);
            $self.next();
        }
        $self.back()
    };
}
