macro_rules! assert_parse {
    (
        $parse_type: ty, $module: ident,

        $(
            success {
                $(
                    $success_string: expr => $value: expr,
                )+
            }
        )?
        $(
            failure {
                $(
                    $failure_string: expr,
                )+
            }
        )?
    ) => {
        mod $module {
            use super::*;

            $(
                #[test]
                fn test_success() {
                    $({
                        let success_string = $success_string;
                        let mut parser_input = cssparser::ParserInput::new(success_string);
                        let mut parser = cssparser::Parser::new(&mut parser_input);
                        let result = <$parse_type>::parse(&mut parser);
                        assert_eq!(result, Ok($value));
                    })+
                }
            )?

            $(
                #[test]
                fn test_failure() {
                    $({
                        let failure_string = $failure_string;
                        let mut parser_input = cssparser::ParserInput::new(failure_string);
                        let mut parser = cssparser::Parser::new(&mut parser_input);
                        let result = <$parse_type>::parse(&mut parser);
                        assert!(result.is_err());
                    })+
                }
            )?
        }
    };
    (
        $parse_type: ty, $module: ident,

        // Ident values
        $(
            ident {
                $(
                    $ident_string: expr => $ident_value: expr,
                )+
            }
        )?

        // Number values
        $(
            number {
                $number_type: expr,
            }
        )?

        // Percentage values
        $(
            percentage {
                $percentage_type: expr,
            }
        )?

        // Dimension values
        $(
            dimension {
                $(
                    $dimension_unit: expr =>
                        $dimension_type: tt$(::$dimension_variant: tt)?$(($dimension_multiplier: expr))?,
                )+
            }
        )?

        // Length values
        $(
            length {
                $length_type: expr,
            }
        )?

        // Custom values
        $(
            custom {
                $(
                    success {
                        $(
                            $custom_success_string: expr => $custom_value: expr,
                        )+
                    }
                )?
                $(
                    failure {
                        $(
                            $custom_failure_string: expr,
                        )+
                    }
                )?
            }
        )?
    ) => {
        $crate::tests::assert_parse! {
            $parse_type, $module,

            success {
                // Ident values
                $(
                    $(
                        $ident_string => $ident_value,
                        &$ident_string.to_uppercase() => $ident_value,
                        &$ident_string.to_lowercase() => $ident_value,
                    )+
                )?

                // Number values
                $(
                    "1" => $number_type(1.0),
                    "2" => $number_type(2.0),
                    "3" => $number_type(3.0),
                    "4" => $number_type(4.0),
                    "5" => $number_type(5.0),
                    "6" => $number_type(6.0),
                    "7" => $number_type(7.0),
                    "8" => $number_type(8.0),
                    "9" => $number_type(9.0),
                    "0.1" => $number_type(0.1),
                    "0.2" => $number_type(0.2),
                    "0.3" => $number_type(0.3),
                    "0.4" => $number_type(0.4),
                    "0.5" => $number_type(0.5),
                    "0.6" => $number_type(0.6),
                    "0.7" => $number_type(0.7),
                    "0.8" => $number_type(0.8),
                    "0.9" => $number_type(0.9),
                    "1.0" => $number_type(1.0),
                    "0.00001" => $number_type(0.00001),
                    "0.99999" => $number_type(0.99999),
                )?

                // Percentage values
                $(
                    "10%" => $percentage_type(0.1),
                    "20%" => $percentage_type(0.2),
                    "30%" => $percentage_type(0.3),
                    "40%" => $percentage_type(0.4),
                    "50%" => $percentage_type(0.5),
                    "60%" => $percentage_type(0.6),
                    "70%" => $percentage_type(0.7),
                    "80%" => $percentage_type(0.8),
                    "90%" => $percentage_type(0.9),
                    "100%" => $percentage_type(1.0),
                    "0.001%" => $percentage_type(0.00001),
                    "99.999%" => $percentage_type(0.99999),
                )?

                // Dimension values
                $(
                    $(
                        concat!(1, $dimension_unit) => $dimension_type$(::$dimension_variant)?(1.0 $(* $dimension_multiplier)?),
                        concat!(2, $dimension_unit) => $dimension_type$(::$dimension_variant)?(2.0 $(* $dimension_multiplier)?),
                        concat!(3, $dimension_unit) => $dimension_type$(::$dimension_variant)?(3.0 $(* $dimension_multiplier)?),
                        concat!(4, $dimension_unit) => $dimension_type$(::$dimension_variant)?(4.0 $(* $dimension_multiplier)?),
                        concat!(5, $dimension_unit) => $dimension_type$(::$dimension_variant)?(5.0 $(* $dimension_multiplier)?),
                        concat!(6, $dimension_unit) => $dimension_type$(::$dimension_variant)?(6.0 $(* $dimension_multiplier)?),
                        concat!(7, $dimension_unit) => $dimension_type$(::$dimension_variant)?(7.0 $(* $dimension_multiplier)?),
                        concat!(8, $dimension_unit) => $dimension_type$(::$dimension_variant)?(8.0 $(* $dimension_multiplier)?),
                        concat!(9, $dimension_unit) => $dimension_type$(::$dimension_variant)?(9.0 $(* $dimension_multiplier)?),
                        concat!(10, $dimension_unit) => $dimension_type$(::$dimension_variant)?(10.0 $(* $dimension_multiplier)?),
                        concat!(0.00001, $dimension_unit) => $dimension_type$(::$dimension_variant)?(0.00001 $(* $dimension_multiplier)?),
                        concat!(99999.0, $dimension_unit) => $dimension_type$(::$dimension_variant)?(99999.0 $(* $dimension_multiplier)?),
                        &format!("{}{}", 1, $dimension_unit.to_uppercase()) => $dimension_type$(::$dimension_variant)?(1.0 $(* $dimension_multiplier)?),
                        &format!("{}{}", 1, $dimension_unit.to_lowercase()) => $dimension_type$(::$dimension_variant)?(1.0 $(* $dimension_multiplier)?),
                    )+
                )?

                // Length values
                $(
                    "1px" => $length_type($crate::Length::px(1.0)),
                    "2in" => $length_type($crate::Length::Value($crate::LengthValue::In(2.0))),
                    "3cm" => $length_type($crate::Length::Value($crate::LengthValue::Cm(3.0))),
                    "4mm" => $length_type($crate::Length::Value($crate::LengthValue::Mm(4.0))),
                    "5q" => $length_type($crate::Length::Value($crate::LengthValue::Q(5.0))),
                    "6pt" => $length_type($crate::Length::Value($crate::LengthValue::Pt(6.0))),
                    "7pc" => $length_type($crate::Length::Value($crate::LengthValue::Pc(7.0))),
                    "8em" => $length_type($crate::Length::Value($crate::LengthValue::Em(8.0))),
                    "9ex" => $length_type($crate::Length::Value($crate::LengthValue::Ex(9.0))),
                    "10ch" => $length_type($crate::Length::Value($crate::LengthValue::Ch(10.0))),
                    "11rem" => $length_type($crate::Length::Value($crate::LengthValue::Rem(11.0))),
                    "12vw" => $length_type($crate::Length::Value($crate::LengthValue::Vw(12.0))),
                    "13vh" => $length_type($crate::Length::Value($crate::LengthValue::Vh(13.0))),
                    "14vmin" => $length_type($crate::Length::Value($crate::LengthValue::Vmin(14.0))),
                    "15vmax" => $length_type($crate::Length::Value($crate::LengthValue::Vmax(15.0))),
                )?

                // Custom values
                $(
                    $(
                        $(
                            $custom_success_string => $custom_value,
                        )+
                    )?
                )?
            }

            failure {
                // Ident values
                $(
                    $(
                        concat!($ident_string, "abc"),
                        concat!("abc", $ident_string),
                        concat!(1, $ident_string),
                        concat!($ident_string, 1),
                    )+
                )?

                // Number values
                $(
                    stringify!($number_type),
                    "abc",
                    "0abc",
                    "a0",
                )?

                // Percentage values
                $(
                    stringify!($percentage_type),
                    "abc",
                    "0abc",
                    "a0",
                )?

                // Dimension values
                $(
                    $(
                        "1",
                        concat!(1, $dimension_unit, "abc"),
                        concat!($dimension_unit, 1),
                    )+
                )?

                // Length values
                $(
                    stringify!($length_type),
                    "abc",
                    "0abc",
                    "a0",
                )?

                // Custom values
                $(
                    $(
                        $(
                            $custom_failure_string,
                        )+
                    )?
                )?
            }
        }
    }
}

pub(crate) use assert_parse;
