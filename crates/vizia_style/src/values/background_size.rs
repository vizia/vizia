use crate::ParserErrorExt;
use crate::values::length::length::parse_unitless_zero;
use crate::{CustomParseError, Length, LengthOrPercentage, LengthPercentageOrAuto, Parse};
use cssparser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundSize {
    Explicit { width: LengthPercentageOrAuto, height: LengthPercentageOrAuto },

    Cover,

    Contain,
}

impl Default for BackgroundSize {
    fn default() -> BackgroundSize {
        BackgroundSize::Explicit {
            width: LengthPercentageOrAuto::Auto,
            height: LengthPercentageOrAuto::Auto,
        }
    }
}

impl<'i> Parse<'i> for BackgroundSize {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        fn parse_dimension<'i>(
            input: &mut Parser<'i>,
        ) -> Result<LengthPercentageOrAuto, ParseError<CustomParseError<'i>>> {
            if let Ok(zero) = input.try_parse(parse_unitless_zero) {
                return Ok(zero.into());
            }
            let value = LengthPercentageOrAuto::parse(input)?;
            if matches!(
                value,
                LengthPercentageOrAuto::LengthPercentage(LengthOrPercentage::Length(Length::Calc(
                    _
                )))
            ) {
                return Err(input.new_custom_error(CustomParseError::InvalidValue));
            }
            Ok(value)
        }

        if let Ok(width) = input.try_parse(parse_dimension) {
            let height = input.try_parse(parse_dimension).unwrap_or(LengthPercentageOrAuto::Auto);
            return Ok(BackgroundSize::Explicit { width, height });
        }

        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
          "cover" => BackgroundSize::Cover,
          "contain" => BackgroundSize::Contain,
          _ => return Err(location.new_unexpected_token_error(
            cssparser::Token::Ident(ident.clone())
          ))
        })
    }
}

impl<'i> Parse<'i> for Vec<BackgroundSize> {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        input.parse_comma_separated(BackgroundSize::parse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LengthOrPercentage;

    #[test]
    fn unitless_zero_sizes_parse_per_axis_and_layer() {
        let zero = LengthPercentageOrAuto::px(0.0);
        let auto = LengthPercentageOrAuto::Auto;
        let px = LengthPercentageOrAuto::px(10.0);
        let percent =
            LengthPercentageOrAuto::LengthPercentage(LengthOrPercentage::Percentage(25.0));
        for (value, width, height) in [
            ("0", zero.clone(), auto.clone()),
            ("0 0", zero.clone(), zero.clone()),
            ("-0 25%", zero.clone(), percent),
            ("auto 0.0", auto, zero.clone()),
            ("10px 0", px, zero.clone()),
        ] {
            let mut parser = Parser::new(value);
            assert_eq!(
                Vec::<BackgroundSize>::parse(&mut parser).unwrap(),
                vec![BackgroundSize::Explicit { width, height }],
                "{value}"
            );
            assert!(parser.is_exhausted(), "{value}");
        }
        let mut parser = Parser::new("0 0, cover, 10px 0");
        let sizes = Vec::<BackgroundSize>::parse(&mut parser).unwrap();
        assert_eq!(sizes.len(), 3);
        assert_eq!(
            sizes[0],
            BackgroundSize::Explicit { width: zero.clone(), height: zero.clone() }
        );
        assert_eq!(sizes[1], BackgroundSize::Cover);
        assert_eq!(
            sizes[2],
            BackgroundSize::Explicit { width: LengthPercentageOrAuto::px(10.0), height: zero }
        );
        assert!(parser.is_exhausted());
    }

    #[test]
    fn unitless_nonzero_sizes_are_rejected() {
        for value in ["1", "0 1", "1 0", "0 0 0", "0,"] {
            assert!(Vec::<BackgroundSize>::parse(&mut Parser::new(value)).is_err(), "{value}");
        }
    }

    #[test]
    fn unresolved_calculations_are_rejected_in_each_axis_and_layer() {
        for value in ["calc(1px + 1em) 20px", "20px calc(1px + 1em)", "cover, calc(1px + 1em) auto"]
        {
            assert!(Vec::<BackgroundSize>::parse(&mut Parser::new(value)).is_err(), "{value}");
        }
        assert!(Vec::<BackgroundSize>::parse(&mut Parser::new("calc(1px + 2px) auto")).is_ok());
    }
}
