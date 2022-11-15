use std::marker::PhantomData;

use cssparser::{ParseError, Parser};

use crate::{CustomParseError, DashedIdent};

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyRule<'i> {
    pub name: DashedIdent<'i>,
}

// impl<'i> PropertyRule<'i> {
//     pub fn parse<'t>(
//         name: DashedIdent<'i>,
//         input: &mut Parser<'i, 't>,
//         loc: Location,
//     ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {

//     }
// }
