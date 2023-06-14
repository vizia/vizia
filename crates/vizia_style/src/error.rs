use cssparser::{BasicParseErrorKind, CowRcStr, ParseError, ParseErrorKind, Token};
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
    /// The filename in which the error occurred.
    pub filename: String,
    /// The line number, starting from 0.
    pub line: u32,
    /// The column number, starting from 1.
    pub column: u32,
}

impl ErrorLocation {
    /// Create a new error location from a source location and filename.
    pub fn new(loc: Location, filename: String) -> Self {
        ErrorLocation { filename, line: loc.line, column: loc.column }
    }
}

impl fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line, self.column)
    }
}

/// A source location.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Location {
    /// The line number, starting at 0.
    pub line: u32,
    /// The column number within a line, starting at 1 for first the character of the line.
    /// Column numbers are counted in UTF-16 code units.
    pub column: u32,
}

#[derive(Debug, PartialEq)]
pub enum CustomParseError<'i> {
    InvalidValue,
    InvalidDeclaration,
    InvalidNesting,
    SelectorError(SelectorError<'i>),
    EndOfInput,
    UnexpectedToken(Token<'i>),
    AtRuleInvalid(CowRcStr<'i>),
    AtRuleBodyInvalid,
    QualifiedRuleInvalid,
}

impl<'i> From<SelectorParseErrorKind<'i>> for CustomParseError<'i> {
    fn from(err: SelectorParseErrorKind<'i>) -> CustomParseError<'i> {
        CustomParseError::SelectorError(err.into())
    }
}

#[derive(Debug, PartialEq)]
pub enum SelectorError<'i> {
    NoQualifiedNameInAttributeSelector(Token<'i>),
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
    UnexpectedTokenInAttributeSelector(Token<'i>),
    PseudoElementExpectedColon(Token<'i>),
    PseudoElementExpectedIdent(Token<'i>),
    NoIdentForPseudo(Token<'i>),
    UnsupportedPseudoClassOrElement(CowRcStr<'i>),
    UnexpectedIdent(CowRcStr<'i>),
    ExpectedNamespace(CowRcStr<'i>),
    ExpectedBarInAttr(Token<'i>),
    BadValueInAttr(Token<'i>),
    InvalidQualNameInAttr(Token<'i>),
    ExplicitNamespaceUnexpectedToken(Token<'i>),
    ClassNeedsIdent(Token<'i>),
}

impl<'i> From<SelectorParseErrorKind<'i>> for SelectorError<'i> {
    fn from(err: SelectorParseErrorKind<'i>) -> Self {
        match &err {
            SelectorParseErrorKind::NoQualifiedNameInAttributeSelector(t) => {
                SelectorError::NoQualifiedNameInAttributeSelector(t.clone())
            }
            SelectorParseErrorKind::EmptySelector => SelectorError::EmptySelector,
            SelectorParseErrorKind::DanglingCombinator => SelectorError::DanglingCombinator,
            SelectorParseErrorKind::NonCompoundSelector => SelectorError::NonCompoundSelector,
            SelectorParseErrorKind::NonPseudoElementAfterSlotted => {
                SelectorError::NonPseudoElementAfterSlotted
            }
            SelectorParseErrorKind::InvalidPseudoElementAfterSlotted => {
                SelectorError::InvalidPseudoElementAfterSlotted
            }
            SelectorParseErrorKind::InvalidPseudoElementInsideWhere => {
                SelectorError::InvalidPseudoElementInsideWhere
            }
            SelectorParseErrorKind::InvalidState => SelectorError::InvalidState,
            //SelectorParseErrorKind::MissingNestingSelector => SelectorError::MissingNestingSelector,
            //SelectorParseErrorKind::MissingNestingPrefix => SelectorError::MissingNestingPrefix,
            SelectorParseErrorKind::UnexpectedTokenInAttributeSelector(t) => {
                SelectorError::UnexpectedTokenInAttributeSelector(t.clone())
            }
            SelectorParseErrorKind::PseudoElementExpectedColon(t) => {
                SelectorError::PseudoElementExpectedColon(t.clone())
            }
            SelectorParseErrorKind::PseudoElementExpectedIdent(t) => {
                SelectorError::PseudoElementExpectedIdent(t.clone())
            }
            SelectorParseErrorKind::NoIdentForPseudo(t) => {
                SelectorError::NoIdentForPseudo(t.clone())
            }
            SelectorParseErrorKind::UnsupportedPseudoClassOrElement(t) => {
                SelectorError::UnsupportedPseudoClassOrElement(t.clone())
            }
            SelectorParseErrorKind::UnexpectedIdent(t) => SelectorError::UnexpectedIdent(t.clone()),
            SelectorParseErrorKind::ExpectedNamespace(t) => {
                SelectorError::ExpectedNamespace(t.clone())
            }
            SelectorParseErrorKind::ExpectedBarInAttr(t) => {
                SelectorError::ExpectedBarInAttr(t.clone())
            }
            SelectorParseErrorKind::BadValueInAttr(t) => SelectorError::BadValueInAttr(t.clone()),
            SelectorParseErrorKind::InvalidQualNameInAttr(t) => {
                SelectorError::InvalidQualNameInAttr(t.clone())
            }
            SelectorParseErrorKind::ExplicitNamespaceUnexpectedToken(t) => {
                SelectorError::ExplicitNamespaceUnexpectedToken(t.clone())
            }
            SelectorParseErrorKind::ClassNeedsIdent(t) => SelectorError::ClassNeedsIdent(t.clone()),
        }
    }
}

impl<'i> SelectorError<'i> {
    fn _reason(&self) -> String {
        use SelectorError::*;
        match self {
        NoQualifiedNameInAttributeSelector(token) => format!("No qualified name in attribute selector: {:?}.", token),
        EmptySelector => "Invalid empty selector.".into(),
        DanglingCombinator => "Invalid dangling combinator in selector.".into(),
        MissingNestingSelector => "A nesting selector (&) is required in each selector of a @nest rule.".into(),
        MissingNestingPrefix => "A nesting selector (&) is required as a prefix of each selector in a nested style rule.".into(),
        UnexpectedTokenInAttributeSelector(token) => format!("Unexpected token in attribute selector: {:?}", token),
        PseudoElementExpectedIdent(token) => format!("Invalid token in pseudo element: {:?}", token),
        UnsupportedPseudoClassOrElement(name) => format!("Unsupported pseudo class or element: {}", name),
        UnexpectedIdent(name) => format!("Unexpected identifier: {}", name),
        ExpectedNamespace(name) => format!("Expected namespace: {}", name),
        ExpectedBarInAttr(name) => format!("Expected | in attribute, got {:?}", name),
        BadValueInAttr(token) => format!("Invalid value in attribute selector: {:?}", token),
        InvalidQualNameInAttr(token) => format!("Invalid qualified name in attribute selector: {:?}", token),
        ExplicitNamespaceUnexpectedToken(token) => format!("Unexpected token in namespace selector: {:?}", token),
        ClassNeedsIdent(token) => format!("Expected identifier in class selector, got {:?}", token),
        InvalidPseudoClassBeforeWebKitScrollbar => "Pseudo class must be prefixed by a ::-webkit-scrollbar pseudo element".into(),
        InvalidPseudoClassAfterWebKitScrollbar => "Invalid pseudo class after ::-webkit-scrollbar pseudo element".into(),
        InvalidPseudoClassAfterPseudoElement => "Invalid pseudo class after pseudo element. Only user action pseudo classes (e.g. :hover, :active) are allowed.".into(),
        err => format!("Error parsing selector: {:?}", err)
        }
    }
}

impl<'i> Error<CustomParseError<'i>> {
    /// Creates an error from a cssparser error.
    pub fn from(
        err: ParseError<'i, CustomParseError<'i>>,
        filename: String,
    ) -> Error<CustomParseError<'i>> {
        let kind = match err.kind {
            ParseErrorKind::Basic(b) => match &b {
                BasicParseErrorKind::UnexpectedToken(t) => {
                    CustomParseError::UnexpectedToken(t.clone())
                }
                BasicParseErrorKind::EndOfInput => CustomParseError::EndOfInput,
                BasicParseErrorKind::AtRuleInvalid(a) => CustomParseError::AtRuleInvalid(a.clone()),
                BasicParseErrorKind::AtRuleBodyInvalid => CustomParseError::AtRuleBodyInvalid,
                BasicParseErrorKind::QualifiedRuleInvalid => CustomParseError::QualifiedRuleInvalid,
            },
            ParseErrorKind::Custom(c) => c,
        };

        Error {
            kind,
            location: Some(ErrorLocation {
                filename,
                line: err.location.line,
                column: err.location.column,
            }),
        }
    }
}
