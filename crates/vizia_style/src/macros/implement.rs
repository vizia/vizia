macro_rules! impl_parse {
    (
        $name: ty,

        // Parse tokens using input.next()?
        $(
            tokens {
                $(
                    ident {
                        $(
                            $ident_pattern: expr => $ident_result: expr,
                        )+
                    }
                )?
                $(
                    dimension {
                        $(
                            $dimension_pattern: literal => $dimension_result_type: tt$(::$variant: tt)?$(($multiplier: expr))?,
                        )+
                    }
                )?
                $(
                    custom {
                        $(
                            $custom_branches: tt
                        )+
                    }
                )?
            }
        )?

        // Parse using input.try_parse(...)
        $(
            try_parse {
                $(
                    $try_parse_type: ty,
                )+
            }
        )?

        // Parse using a closure
        $(
            custom {
                $custom_closure: expr
            }
        )?
    ) => {
        impl<'i> Parse<'i> for $name {
            fn parse<'t>(input: &mut cssparser::Parser<'i>) -> Result<Self, cssparser::ParseError<$crate::CustomParseError<'i>>> {
                // Parse tokens using input.next()?
                $(
                    Ok(match input.next()? {
                        $(
                            $(
                                cssparser::Token::Ident(ident) if ident.eq_ignore_ascii_case($ident_pattern) => $ident_result,
                            )+
                        )?
                        $(
                            $(
                                &cssparser::Token::Dimension { value, ref unit, .. } if unit.as_ref().eq_ignore_ascii_case($dimension_pattern) => {
                                    $dimension_result_type$(::$variant)?(value $(* $multiplier)?)
                                },
                            )+
                        )?
                        $(
                            $(
                                $custom_branches
                            )+
                        )?
                        _ => return Err(cssparser::ParseError::custom($crate::CustomParseError::InvalidDeclaration)),
                    })
                )?

                // Parse using input.try_parse(...)
                $(
                    $(
                        if let Ok(x) = input.try_parse(<$try_parse_type>::parse) {
                            return Ok(x.into());
                        }
                    )+

                    Err(cssparser::ParseError::custom($crate::CustomParseError::InvalidDeclaration))
                )?

                // Parse using a closure
                $(
                    input.try_parse($custom_closure)
                )?
            }
        }
    };
}

pub(crate) use impl_parse;
