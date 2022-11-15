use cssparser::*;

use crate::{CustomParseError, DashedIdent, Parse, ParserOptions};

#[derive(Debug, Clone, PartialEq)]
pub struct CustomProperty<'i> {
    pub name: CowRcStr<'i>,
    pub value: TokenList<'i>,
}

impl<'i> CustomProperty<'i> {
    pub fn parse<'t>(
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        //let value = TokenList::parse(input)?;
        // Ok(CustomProperty {
        //     name: CowRcStr::from("TODO"),
        //     value: TokenList(vec![TokenOrValue::Color(Color::CurrentColor)]),
        // })

        let value = TokenList::parse(input)?;
        Ok(CustomProperty { name, value })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnparsedProperty<'i> {
    pub name: CowRcStr<'i>,
    pub value: TokenList<'i>,
}

impl<'i> UnparsedProperty<'i> {
    pub fn parse<'t>(
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let value = TokenList::parse(input)?;
        Ok(UnparsedProperty { name, value })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenList<'i>(#[cfg_attr(feature = "serde", serde(borrow))] pub Vec<TokenOrValue<'i>>);

#[derive(Debug, Clone, PartialEq)]
pub enum TokenOrValue<'i> {
    /// A token.
    Token(Token<'i>),
    /// A parsed CSS color.
    Color(Color),
    /// A parsed CSS url. (TODO)
    //Url(Url<'i>),
    /// A CSS variable reference.
    Var(Variable<'i>),
}

impl<'i> From<Token<'i>> for TokenOrValue<'i> {
    fn from(token: Token<'i>) -> TokenOrValue<'i> {
        TokenOrValue::Token(token)
    }
}

impl<'i> TokenOrValue<'i> {
    /// Returns whether the token is whitespace.
    pub fn is_whitespace(&self) -> bool {
        matches!(self, TokenOrValue::Token(Token::WhiteSpace(_)))
    }
}

impl<'i> TokenList<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_until_before(Delimiter::Bang | Delimiter::Semicolon, |input| {
            let mut tokens = vec![];
            TokenList::parse_into(input, &mut tokens)?;

            // Slice off leading and trailing whitespace if there are at least two tokens.
            // If there is only one token, we must preserve it. e.g. `--foo: ;` is valid.
            if tokens.len() >= 2 {
                let mut slice = &tokens[..];
                if matches!(tokens.first(), Some(token) if token.is_whitespace()) {
                    slice = &slice[1..];
                }
                if matches!(tokens.last(), Some(token) if token.is_whitespace()) {
                    slice = &slice[..slice.len() - 1];
                }
                return Ok(TokenList(slice.to_vec()));
            }

            return Ok(TokenList(tokens));
        })
    }

    fn parse_into<'t>(
        input: &mut Parser<'i, 't>,
        tokens: &mut Vec<TokenOrValue<'i>>,
    ) -> Result<(), ParseError<'i, CustomParseError<'i>>> {
        let mut last_is_delim = false;
        let mut last_is_whitespace = false;
        loop {
            let state = input.state();
            match input.next_including_whitespace_and_comments() {
                Ok(&cssparser::Token::WhiteSpace(..)) | Ok(&cssparser::Token::Comment(..)) => {
                    // Skip whitespace if the last token was a delimeter.
                    // Otherwise, replace all whitespace and comments with a single space character.
                    if !last_is_delim {
                        tokens.push(Token::WhiteSpace(" ").into());
                        last_is_whitespace = true;
                    }
                }
                Ok(&cssparser::Token::Function(ref f)) => {
                    // Attempt to parse embedded color values into hex tokens.
                    let f = f.clone();
                    if let Some(color) = try_parse_color_token(&f, &state, input) {
                        tokens.push(TokenOrValue::Color(color));
                        last_is_delim = false;
                        last_is_whitespace = false;
                    // } else if f == "url" {
                    //     input.reset(&state);
                    //     tokens.push(TokenOrValue::Url(Url::parse(input)?));
                    //     last_is_delim = false;
                    //     last_is_whitespace = false;
                    } else if f == "var" {
                        let var = input.parse_nested_block(|input| {
                            let var = Variable::parse(input)?;
                            Ok(TokenOrValue::Var(var))
                        })?;
                        tokens.push(var);
                        last_is_delim = true;
                        last_is_whitespace = false;
                    } else {
                        tokens.push(Token::Function(f).into());
                        input.parse_nested_block(|input| TokenList::parse_into(input, tokens))?;
                        tokens.push(Token::CloseParenthesis.into());
                        last_is_delim = true; // Whitespace is not required after any of these chars.
                        last_is_whitespace = false;
                    }
                }
                Ok(&cssparser::Token::Hash(ref h)) | Ok(&cssparser::Token::IDHash(ref h)) => {
                    if let Ok(color) = Color::parse_hash(h.as_bytes()) {
                        tokens.push(TokenOrValue::Color(color.into()));
                    } else {
                        tokens.push(Token::Hash(h.clone()).into());
                    }
                    last_is_delim = false;
                    last_is_whitespace = false;
                }
                Ok(&cssparser::Token::UnquotedUrl(_)) => {
                    input.reset(&state);
                    //tokens.push(TokenOrValue::Url(Url::parse(input)?));
                    last_is_delim = false;
                    last_is_whitespace = false;
                }
                Ok(token @ &cssparser::Token::ParenthesisBlock)
                | Ok(token @ &cssparser::Token::SquareBracketBlock)
                | Ok(token @ &cssparser::Token::CurlyBracketBlock) => {
                    tokens.push(Token::from(token.clone()).into());
                    let closing_delimiter = match token {
                        cssparser::Token::ParenthesisBlock => Token::CloseParenthesis,
                        cssparser::Token::SquareBracketBlock => Token::CloseSquareBracket,
                        cssparser::Token::CurlyBracketBlock => Token::CloseCurlyBracket,
                        _ => unreachable!(),
                    };

                    input.parse_nested_block(|input| TokenList::parse_into(input, tokens))?;

                    tokens.push(closing_delimiter.into());
                    last_is_delim = true; // Whitespace is not required after any of these chars.
                    last_is_whitespace = false;
                }
                Ok(token) => {
                    last_is_delim =
                        matches!(token, cssparser::Token::Delim(_) | cssparser::Token::Comma);

                    // If this is a delimeter, and the last token was whitespace,
                    // replace the whitespace with the delimeter since both are not required.
                    if last_is_delim && last_is_whitespace {
                        let last = tokens.last_mut().unwrap();
                        *last = Token::from(token.clone()).into();
                    } else {
                        tokens.push(Token::from(token.clone()).into());
                    }

                    last_is_whitespace = false;
                }
                Err(_) => break,
            }
        }

        Ok(())
    }
}

#[inline]
fn try_parse_color_token<'i, 't>(
    f: &CowRcStr<'i>,
    state: &ParserState,
    input: &mut Parser<'i, 't>,
) -> Option<Color> {
    match_ignore_ascii_case! { &*f,
        "rgb" | "rgba" | "hsl" | "hsla" | "hwb" | "lab" | "lch" | "oklab" | "oklch" | "color" | "color-mix" => {
        let s = input.state();
        input.reset(&state);
        if let Ok(color) = Color::parse(input) {
            return Some(color)
        }
        input.reset(&s);
        },
        _ => {}
    }

    None
}

/// A CSS variable reference.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Variable<'i> {
    /// The variable name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: DashedIdent<'i>,
    /// A fallback value in case the variable is not defined.
    pub fallback: Option<TokenList<'i>>,
}

impl<'i> Variable<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let name = DashedIdent::parse(input)?;

        let fallback = if input.try_parse(|input| input.expect_comma()).is_ok() {
            Some(TokenList::parse(input)?)
        } else {
            None
        };

        Ok(Variable { name, fallback })
    }
}

#[cfg(test)]
mod tests {
    use cssparser::CowRcStr;

    use crate::{CustomProperty, Parse};

    #[test]
    fn parse_custom_ident() {
        let success_string = "left";
        let mut parser_input = cssparser::ParserInput::new(success_string);
        let mut parser = cssparser::Parser::new(&mut parser_input);
        let result = CustomProperty::parse(CowRcStr::from("custom"), &mut parser);
        println!("{:?}", result);
        // assert_eq!(result, Ok($value));
    }

    #[test]
    fn parse_custom_color() {
        let success_string = "#456789";
        let mut parser_input = cssparser::ParserInput::new(success_string);
        let mut parser = cssparser::Parser::new(&mut parser_input);
        let result = CustomProperty::parse(CowRcStr::from("custom"), &mut parser);
        println!("{:?}", result);
        // assert_eq!(result, Ok($value));
    }

    #[test]
    fn parse_custom_value() {
        let success_string = "3px";
        let mut parser_input = cssparser::ParserInput::new(success_string);
        let mut parser = cssparser::Parser::new(&mut parser_input);
        let result = CustomProperty::parse(CowRcStr::from("custom"), &mut parser);
        println!("{:?}", result);
        // assert_eq!(result, Ok($value));
    }

    #[test]
    fn parse_custom_complex() {
        let success_string = "left 3px rgb(100, 200, 50)";
        let mut parser_input = cssparser::ParserInput::new(success_string);
        let mut parser = cssparser::Parser::new(&mut parser_input);
        let result = CustomProperty::parse(CowRcStr::from("custom"), &mut parser);
        println!("{:?}", result);
        // assert_eq!(result, Ok($value));
    }
}
