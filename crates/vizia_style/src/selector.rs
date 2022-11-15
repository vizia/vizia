use cssparser::*;
use parcel_selectors::SelectorImpl;

use crate::{CustomParseError, Direction, Parse, PseudoClass, PseudoElement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selectors;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SelectorString<'a>(pub CowRcStr<'a>);

impl<'a> std::convert::From<CowRcStr<'a>> for SelectorString<'a> {
    fn from(s: CowRcStr<'a>) -> SelectorString<'a> {
        SelectorString(s.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct SelectorIdent<'i>(pub CowRcStr<'i>);

impl<'a> std::convert::From<CowRcStr<'a>> for SelectorIdent<'a> {
    fn from(s: CowRcStr<'a>) -> SelectorIdent {
        SelectorIdent(s.into())
    }
}

impl<'i> SelectorImpl<'i> for Selectors {
    type AttrValue = SelectorString<'i>;
    type Identifier = SelectorIdent<'i>;
    type LocalName = SelectorIdent<'i>;
    type NamespacePrefix = SelectorIdent<'i>;
    type NamespaceUrl = SelectorIdent<'i>;
    type BorrowedNamespaceUrl = SelectorIdent<'i>;
    type BorrowedLocalName = SelectorIdent<'i>;

    type NonTSPseudoClass = PseudoClass<'i>;
    type PseudoElement = PseudoElement<'i>;

    type ExtraMatchingData = ();
}

pub struct SelectorParser<'a, 'i> {
    pub default_namespace: &'a Option<CowRcStr<'i>>,
    pub is_nesting_allowed: bool,
}

impl<'a, 'i> parcel_selectors::parser::Parser<'i> for SelectorParser<'a, 'i> {
    type Impl = Selectors;
    type Error = CustomParseError<'i>;

    fn parse_non_ts_pseudo_class(
        &self,
        _: SourceLocation,
        name: CowRcStr<'i>,
    ) -> Result<PseudoClass<'i>, ParseError<'i, Self::Error>> {
        use PseudoClass::*;
        let pseudo_class = match_ignore_ascii_case! { &name,
            "hover" => Hover,
            "active" => Active,
            "focus" => Focus,
            "enabled" => Enabled,
            "disabled" => Disabled,
            "read-only" => ReadOnly,
            "read-write" => ReadWrite,
            "default" => Default,
            "checked" => Checked,
            "indeterminate" => Indeterminate,
            "blank" => Blank,
            "valid" => Valid,
            "invalid" => Invalid,
            "in-range" => InRange,
            "out-of-range" => OutOfRange,
            "required" => Required,
            "optional" => Optional,
            "user-valid" => UserValid,
            "user-invalid" => UserInvalid,

            _ => Custom(name.into())

        };

        Ok(pseudo_class)
    }

    fn parse_non_ts_functional_pseudo_class<'t>(
        &self,
        name: CowRcStr<'i>,
        parser: &mut Parser<'i, 't>,
    ) -> Result<<Self::Impl as SelectorImpl<'i>>::NonTSPseudoClass, ParseError<'i, Self::Error>>
    {
        use PseudoClass::*;
        let pseudo_class = match_ignore_ascii_case! { &name,
            "lang" => {
                let langs = parser.parse_comma_separated(|parser|{
                    parser.expect_ident_or_string()
                        .map(|s| s.clone())
                        .map_err(|e| e.into())
                })?;
                Lang(langs)
            },

            "dir" => {
                Dir(Direction::parse(parser)?)
            },

            _=> return Err(parser.new_custom_error(parcel_selectors::parser::SelectorParseErrorKind::UnexpectedIdent(name.clone()))),
        };

        Ok(pseudo_class)
    }

    fn parse_pseudo_element(
        &self,
        location: SourceLocation,
        name: CowRcStr<'i>,
    ) -> Result<<Self::Impl as SelectorImpl<'i>>::PseudoElement, ParseError<'i, Self::Error>> {
        use PseudoElement::*;
        let pseudo_element = match_ignore_ascii_case! { &name,
            "before" => Before,
            "after" => After,
            "selection" => Selection,
            _=> Custom(name.into())
        };

        Ok(pseudo_element)
    }
}

#[cfg(test)]
mod tests {
    use parcel_selectors::{
        parser::{Component, LocalName, Selector},
        SelectorList,
    };

    use super::*;

    fn parse<'i>(
        input: &'i str,
    ) -> Result<SelectorList<Selectors>, ParseError<'i, CustomParseError<'i>>> {
        let mut parser_input = ParserInput::new(input);
        let mut parser = Parser::new(&mut parser_input);
        SelectorList::parse(
            &SelectorParser { default_namespace: &None, is_nesting_allowed: true },
            &mut parser,
            parcel_selectors::parser::NestingRequirement::None,
        )
    }

    fn specificity(a: u32, b: u32, c: u32) -> u32 {
        a << 20 | b << 10 | c
    }

    #[test]
    fn parse_empty() {
        let mut parser_input = ParserInput::new(":empty");
        let mut parser = Parser::new(&mut parser_input);
        let result = SelectorList::parse(
            &SelectorParser { default_namespace: &None, is_nesting_allowed: true },
            &mut parser,
            parcel_selectors::parser::NestingRequirement::None,
        );
        assert!(result.is_ok());
    }

    // TODO - Some fancy macros for making this easier
    #[test]
    fn parse_universal() {
        assert_eq!(
            parse("*"),
            Ok(SelectorList::from_vec(vec![Selector::from_vec(
                vec![Component::ExplicitUniversalType],
                specificity(0, 0, 0),
                Default::default(),
            )]))
        );
    }

    #[test]
    fn parse_element() {
        assert_eq!(
            parse("bar"),
            Ok(SelectorList::from_vec(vec![Selector::from_vec(
                vec![Component::LocalName(LocalName {
                    name: SelectorIdent("bar".into()),
                    lower_name: SelectorIdent("bar".into()),
                })],
                specificity(0, 0, 1),
                Default::default(),
            )]))
        );
    }

    #[test]
    fn parse_id() {
        assert_eq!(
            parse("#bar"),
            Ok(SelectorList::from_vec(vec![Selector::from_vec(
                vec![Component::ID(SelectorIdent("bar".into()))],
                specificity(1, 0, 0),
                Default::default(),
            )]))
        );
    }

    #[test]
    fn parse_element_id() {
        assert_eq!(
            parse("foo#bar"),
            Ok(SelectorList::from_vec(vec![Selector::from_vec(
                vec![
                    Component::LocalName(LocalName {
                        name: SelectorIdent("foo".into()),
                        lower_name: SelectorIdent("foo".into()),
                    }),
                    Component::ID(SelectorIdent("bar".into()))
                ],
                specificity(1, 0, 1),
                Default::default(),
            )]))
        );
    }

    #[test]
    fn parse_class() {
        assert_eq!(
            parse(".bar"),
            Ok(SelectorList::from_vec(vec![Selector::from_vec(
                vec![Component::Class(SelectorIdent("bar".into()))],
                specificity(0, 1, 0),
                Default::default(),
            )]))
        );
    }

    #[test]
    fn parse_element_class() {
        assert_eq!(
            parse("foo.bar"),
            Ok(SelectorList::from_vec(vec![Selector::from_vec(
                vec![
                    Component::LocalName(LocalName {
                        name: SelectorIdent("foo".into()),
                        lower_name: SelectorIdent("foo".into()),
                    }),
                    Component::Class(SelectorIdent("bar".into()))
                ],
                specificity(0, 1, 1),
                Default::default(),
            )]))
        );
    }

    #[test]
    fn parse_element_class_id() {
        assert_eq!(
            parse("foo.bar#baz"),
            Ok(SelectorList::from_vec(vec![Selector::from_vec(
                vec![
                    Component::LocalName(LocalName {
                        name: SelectorIdent("foo".into()),
                        lower_name: SelectorIdent("foo".into()),
                    }),
                    Component::Class(SelectorIdent("bar".into())),
                    Component::ID(SelectorIdent("baz".into())),
                ],
                specificity(1, 1, 1),
                Default::default(),
            )]))
        );
    }

    #[test]
    fn parse_element_id_class() {
        assert_eq!(
            parse("foo#bar.baz"),
            Ok(SelectorList::from_vec(vec![Selector::from_vec(
                vec![
                    Component::LocalName(LocalName {
                        name: SelectorIdent("foo".into()),
                        lower_name: SelectorIdent("foo".into()),
                    }),
                    Component::ID(SelectorIdent("bar".into())),
                    Component::Class(SelectorIdent("baz".into())),
                ],
                specificity(1, 1, 1),
                Default::default(),
            )]))
        );
    }

    // TODO - Add more tests for selectors
    // TODO - Add tests for selector matching
    //   NOTE - Requires creating a dummy node for testing purposes (and also modification to selectors crate to allow properties from external store)
}
