use cssparser::{BasicParseErrorKind, CowRcStr, ParseError, ParseErrorKind, SourceLocation, Token};
use selectors::parser::SelectorParseErrorKind;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Error<T> {
    pub kind: T,
    pub location: Option<ErrorLocation>,
}

impl<T: fmt::Display> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)?;
        if let Some(loc) = &self.location {
            write!(f, " at {}", loc)?;
        }
        Ok(())
    }
}

impl<T: fmt::Display + fmt::Debug> std::error::Error for Error<T> {}

#[derive(Debug, PartialEq, Clone)]
pub struct ErrorLocation {
    pub filename: String,
    pub line: u32,
    pub column: u32,
}

impl ErrorLocation {
    pub fn new(loc: Location, filename: String) -> Self {
        ErrorLocation { filename, line: loc.line, column: loc.column }
    }
}

impl fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line, self.column)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CustomParseError<'i> {
    InvalidValue,
    InvalidDeclaration,
    InvalidNesting,
    SelectorError(SelectorError),
    EndOfInput,
    UnexpectedToken(Option<Token<'i>>),
    AtRuleInvalid(Option<CowRcStr<'i>>),
    AtRuleBodyInvalid,
    QualifiedRuleInvalid,
}

impl From<SelectorParseErrorKind> for CustomParseError<'_> {
    fn from(err: SelectorParseErrorKind) -> Self {
        CustomParseError::SelectorError(err.into())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SelectorError {
    NoQualifiedNameInAttributeSelector,
    EmptySelector,
    DanglingCombinator,
    NonCompoundSelector,
    NonPseudoElementAfterSlotted,
    InvalidPseudoElementAfterSlotted,
    InvalidPseudoElementInsideWhere,
    InvalidPseudoClassBeforeWebKitScrollbar,
    InvalidPseudoClassAfterWebKitScrollbar,
    InvalidPseudoClassAfterPseudoElement,
    InvalidState,
    MissingNestingSelector,
    MissingNestingPrefix,
    UnexpectedTokenInAttributeSelector,
    PseudoElementExpectedColon,
    PseudoElementExpectedIdent,
    NoIdentForPseudo,
    UnsupportedPseudoClassOrElement,
    UnexpectedIdent,
    ExpectedNamespace,
    ExpectedBarInAttr,
    BadValueInAttr,
    InvalidQualNameInAttr,
    ExplicitNamespaceUnexpectedToken,
    ClassNeedsIdent,
}

impl From<SelectorParseErrorKind> for SelectorError {
    fn from(err: SelectorParseErrorKind) -> Self {
        match err {
            SelectorParseErrorKind::NoQualifiedNameInAttributeSelector => {
                Self::NoQualifiedNameInAttributeSelector
            }
            SelectorParseErrorKind::EmptySelector => Self::EmptySelector,
            SelectorParseErrorKind::DanglingCombinator => Self::DanglingCombinator,
            SelectorParseErrorKind::NonCompoundSelector => Self::NonCompoundSelector,
            SelectorParseErrorKind::NonPseudoElementAfterSlotted => {
                Self::NonPseudoElementAfterSlotted
            }
            SelectorParseErrorKind::InvalidPseudoElementAfterSlotted => {
                Self::InvalidPseudoElementAfterSlotted
            }
            SelectorParseErrorKind::InvalidPseudoElementInsideWhere => {
                Self::InvalidPseudoElementInsideWhere
            }
            SelectorParseErrorKind::InvalidState => Self::InvalidState,
            SelectorParseErrorKind::UnexpectedTokenInAttributeSelector => {
                Self::UnexpectedTokenInAttributeSelector
            }
            SelectorParseErrorKind::PseudoElementExpectedColon => Self::PseudoElementExpectedColon,
            SelectorParseErrorKind::PseudoElementExpectedIdent => Self::PseudoElementExpectedIdent,
            SelectorParseErrorKind::NoIdentForPseudo => Self::NoIdentForPseudo,
            SelectorParseErrorKind::UnsupportedPseudoClassOrElement => {
                Self::UnsupportedPseudoClassOrElement
            }
            SelectorParseErrorKind::UnexpectedIdent => Self::UnexpectedIdent,
            SelectorParseErrorKind::ExpectedNamespace => Self::ExpectedNamespace,
            SelectorParseErrorKind::ExpectedBarInAttr => Self::ExpectedBarInAttr,
            SelectorParseErrorKind::BadValueInAttr => Self::BadValueInAttr,
            SelectorParseErrorKind::InvalidQualNameInAttr => Self::InvalidQualNameInAttr,
            SelectorParseErrorKind::ExplicitNamespaceUnexpectedToken => {
                Self::ExplicitNamespaceUnexpectedToken
            }
            SelectorParseErrorKind::ClassNeedsIdent => Self::ClassNeedsIdent,
        }
    }
}

impl SelectorError {
    fn _reason(&self) -> String {
        format!("Error parsing selector: {self:?}")
    }
}

impl<'i> Error<CustomParseError<'i>> {
    pub fn from(
        err: ParseError<CustomParseError<'i>>,
        filename: String,
    ) -> Error<CustomParseError<'i>> {
        Self::from_at(err, filename, None)
    }

    pub fn from_at(
        err: ParseError<CustomParseError<'i>>,
        filename: String,
        source_location: Option<SourceLocation>,
    ) -> Error<CustomParseError<'i>> {
        let kind = match err.kind {
            ParseErrorKind::Basic(b) => match b {
                BasicParseErrorKind::UnexpectedToken => CustomParseError::UnexpectedToken(None),
                BasicParseErrorKind::EndOfInput => CustomParseError::EndOfInput,
                BasicParseErrorKind::AtRuleInvalid => CustomParseError::AtRuleInvalid(None),
                BasicParseErrorKind::AtRuleBodyInvalid => CustomParseError::AtRuleBodyInvalid,
                BasicParseErrorKind::QualifiedRuleInvalid => CustomParseError::QualifiedRuleInvalid,
                BasicParseErrorKind::TooManyNestedBlocks => CustomParseError::InvalidNesting,
            },
            ParseErrorKind::Custom(c) => c,
        };

        Error {
            kind,
            location: source_location.map(|loc| ErrorLocation {
                filename,
                line: loc.line,
                column: loc.column,
            }),
        }
    }
}
