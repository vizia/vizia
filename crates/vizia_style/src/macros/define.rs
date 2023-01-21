macro_rules! define_enum {
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$meta: meta])*
                $str: literal: $id: ident,
            )+
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        $vis enum $name {
            $(
                $(#[$meta])*
                $id,
            )+
        }

        $crate::impl_parse! {
            $name,

            tokens {
                ident {
                    $($str => $name::$id,)+
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            $crate::tests::assert_parse! {
                $name, assert_parse,

                ident {
                    $($str => $name::$id,)+
                }
            }
        }
  };
}

macro_rules! define_property {
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident<'i> {
            $(
                $(#[$meta: meta])*
                $str: literal: $variant: ident($inner_ty: ty),
            )+
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq)]
        $vis enum $name<'i> {
            $(
                $(#[$meta])*
                $variant($inner_ty),
            )+
            Unparsed(UnparsedProperty<'i>),
            Custom(CustomProperty<'i>),
        }

        impl<'i> $name<'i> {
            pub fn parse_value<'t>(name: cssparser::CowRcStr<'i>, input: &mut Parser<'i, 't>) -> Result<Self, cssparser::ParseError<'i, CustomParseError<'i>>> {

                let state = input.state();
                let name_ref = name.as_ref();
                match name_ref {
                    $(
                        $str => {
                            if let Ok(val) = <$inner_ty>::parse(input) {
                                return Ok($name::$variant(val));
                            }
                        }
                    )+
                    _ => {
                        if let Ok(custom) = CustomProperty::parse(name.clone(), input) {
                            return Ok(Property::Custom(custom));
                        }
                    }
                }

                input.reset(&state);
                return Ok(Property::Unparsed(UnparsedProperty::parse(name, input)?));
            }
        }
    };
}

pub(crate) use define_enum;
pub(crate) use define_property;
