use crate::{impl_parse, Parse};
use cssparser::Token;

impl_parse! {
    i8,

    tokens {
        custom{
            Token::Number { int_value: Some(int_value), .. } if i8::try_from(*int_value).is_ok() => *int_value as i8,
        }
    }
}

impl_parse! {
    i16,

    tokens {
        custom{
            Token::Number { int_value: Some(int_value), .. } if i16::try_from(*int_value).is_ok() => *int_value as i16,
        }
    }
}

impl_parse! {
    i32,

    tokens {
        custom{
            Token::Number { int_value: Some(int_value), .. } => *int_value,
        }
    }
}

impl_parse! {
    i64,

    tokens {
        custom{
            Token::Number { int_value: Some(int_value), .. } => i64::from(*int_value),
        }
    }
}

impl_parse! {
    i128,

    tokens {
        custom{
            Token::Number { int_value: Some(int_value), .. } => i128::from(*int_value),
        }
    }
}

impl_parse! {
    isize,

    tokens {
        custom{
            Token::Number { int_value: Some(int_value), .. } if isize::try_from(*int_value).is_ok() => *int_value as isize,
        }
    }
}

impl_parse! {
    u8,

    tokens {
        custom{
            Token::Number { int_value: Some(int_value), .. } if u8::try_from(*int_value).is_ok() => *int_value as u8,
        }
    }
}

impl_parse! {
    u16,

    tokens {
        custom{
            Token::Number { int_value: Some(int_value), .. } if u16::try_from(*int_value).is_ok() => *int_value as u16,
        }
    }
}

impl_parse! {
    u32,

    tokens {
        custom{
            Token::Number { int_value: Some(int_value), .. } if u32::try_from(*int_value).is_ok() => *int_value as u32,
        }
    }
}

impl_parse! {
    u64,

    tokens {
        custom{
            Token::Number { int_value: Some(int_value), .. } if u64::try_from(*int_value).is_ok() => *int_value as u64,
        }
    }
}

impl_parse! {
    u128,

    tokens {
        custom{
            Token::Number { int_value: Some(int_value), .. } if u128::try_from(*int_value).is_ok() => *int_value as u128,
        }
    }
}

impl_parse! {
    usize,

    tokens {
        custom{
            Token::Number { int_value: Some(int_value), .. } if usize::try_from(*int_value).is_ok() => *int_value as usize,
        }
    }
}

impl_parse! {
    f32,

    tokens {
        custom{
            Token::Number { value, .. } => *value,
        }
    }
}

impl_parse! {
    f64,

    tokens {
        custom{
            Token::Number { value, .. } => f64::from(*value),
        }
    }
}
