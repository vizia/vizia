use crate::error::Error;
use crate::ParserOptions;

use crate::rule::TopLevelRuleParser;
use crate::{CssRuleList, CustomParseError};
use cssparser::*;

#[derive(Debug)]
pub struct StyleSheet<'i> {
    // List of top level rules
    pub rules: CssRuleList<'i>,

    pub options: ParserOptions<'i>,
}

impl<'i> StyleSheet<'i> {
    pub fn parse(
        code: &'i str,
        options: ParserOptions<'i>,
    ) -> Result<Self, Error<CustomParseError<'i>>> {
        let mut input = ParserInput::new(code);
        let mut parser = Parser::new(&mut input);

        let mut rules = CssRuleList(vec![]);
        let mut rule_parser = TopLevelRuleParser::new(&options, &mut rules);
        let mut rule_list_parser = StyleSheetParser::new(&mut parser, &mut rule_parser);

        while let Some(rule) = rule_list_parser.next() {
            match rule {
                Ok(_) => {}
                Err((e, _)) => {
                    let options = &mut rule_list_parser.parser.options;
                    if options.error_recovery {
                        options.warn(e);
                        continue;
                    }

                    return Err(Error::from(e, options.filename.clone()));
                }
            }
        }

        Ok(StyleSheet { rules, options })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CSS_EXAMPLE: &str = r#"
button label {
    left: 10%;
    right: 20px;
    top: 30in;
    bottom: 40cm;
    min-left: 50mm;
    max-left: 60q;
    background-color: white;
    width: auto;
    height: 30px;
    corner-radius: 3px;
    padding: 1st;
    padding-left: 10px;
    padding-right: 10px;
    border-width: 1px;
    border-color: #e5e5e5;
    outer-shadow: 0px 1px 1px #00000055;
    overflow: visible;
    position: relative;
    left: 10%;
    position: absolute;
    opacity: 10%;
    opacity: 000.10;
    background-color: red;
    layout-type: grid;
    layout-type: column;
    layout-type: row;
    font-size: 10;
    font-size: large;
    font-size: medium;
    font-size: small;
    font: "test 234234 2332 4";
    font: "";
    background-image: "23487";
    display: none;
    display: flex;
    overflow: hidden;
    visibility: invisible;
    overflow: visible;
    visibility: visible;
    text-wrap: false;
    text-wrap: yes;
    text-wrap: on;
    cursor: default;
    cursor: move;
    cursor: crosshair;
    corner-top-right-shape: bevel;
    outer-shadow: 10px 8px 10px #123456;
    z-index: 9999900;
    transition: test 1s, test2 2s, test3 3s 4s;
    background-color: weriwrgba(12, 13, 14, 0.1);
    backgrond-color: hsla(120, 100%, 50%, 0.1);
    outline-color: red;
    outline-bottom-color: #00FF00;
    outline-corner-shape: round;
    outline-top-right-shape: bevel;
    outline-radius: 2px;
    outline-bottom-left-radius: 10px;
    transform: rotate(10deg);
    transform: scale(20%, 30);
    transform: scale(20%, 30) rotate(50rad);
    transform: scale(20%, 30) rotate(50rad) skew(50deg, 30turn);
    translate: 10px, 20px;
    rotate: 20deg;
    scale: 20%, 10;
    border-corner-shape: bevel round round bevel;
    corner-top-left-shape: round;
    corner-top-right-shape: bevel;
}

test {
    background-color: #123123;
}
"#;

    #[test]
    fn parse_stylsheet() {
        let style_sheet = StyleSheet::parse(CSS_EXAMPLE, ParserOptions::default());
        println!("{:#?}", style_sheet);
    }
}

// use cssparser::*;

// use crate::{CssRule, CssRuleList, ParserOptions};

// #[derive(Debug)]
// pub struct StyleSheet<'i> {
//     pub rules: CssRuleList<'i>,
//     pub sources: Vec<String>,
//     options: ParserOptions,
// }

// impl<'i> StyleSheet<'i> {
//     pub fn new(sources: Vec<String>, rules: CssRuleList, options: ParserOptions) -> StyleSheet {
//         StyleSheet {
//             sources,
//             rules,
//             options,
//         }
//     }

//     pub fn parse(
//         filename: String,
//         code: &'i str,
//         options: ParserOptions,
//     ) -> Result<StyleSheet<'i>, Error<ParserError<'i>>> {
//         let mut input = ParserInput::new(&code);
//         let mut parser = Parser::new(&mut input);
//         let rule_list_parser =
//             RuleListParser::new_for_stylesheet(&mut parser, TopLevelRuleParser::new(&options));

//         let mut rules = vec![];
//         for rule in rule_list_parser {
//             let rule = match rule {
//                 Ok((_, CssRule::Ignored)) => continue,
//                 Ok((_, rule)) => rule,
//                 Err((e, _)) => return Err(Error::from(e, filename)),
//             };

//             rules.push(rule)
//         }

//         Ok(StyleSheet {
//             sources: vec![filename],
//             rules: CssRuleList(rules),
//             options,
//         })
//     }
// }
