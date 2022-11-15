use crate::{macros::impl_parse, Parse};
pub use std::time::Duration;

impl_parse! {
    Duration,

    tokens {
        custom {
            cssparser::Token::Dimension {
                value, ref unit, ..
            } if unit.as_ref().eq_ignore_ascii_case("s") => Duration::from_secs(*value as u64),
            cssparser::Token::Dimension {
                value, ref unit, ..
            } if unit.as_ref().eq_ignore_ascii_case("ms") => Duration::from_millis(*value as u64),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        Duration, assert_duration,

        custom {
            success {
                "1s" => Duration::from_secs(1),
                "10s" => Duration::from_secs(10),
                "100s" => Duration::from_secs(100),
                "1000s" => Duration::from_secs(1000),
                "1ms" => Duration::from_millis(1),
                "10ms" => Duration::from_millis(10),
                "100ms" => Duration::from_millis(100),
                "1000ms" => Duration::from_millis(1000),
            }

            failure {
                "test",
                "123",
                "1x",
                "1sms",
            }
        }
    }
}
