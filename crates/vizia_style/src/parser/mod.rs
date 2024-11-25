use std::{
    marker::PhantomData,
    sync::{Arc, RwLock},
};

use cssparser::ParseError;

use crate::{CustomParseError, Error};

pub mod rule;

#[derive(Debug, Default, Clone)]
pub struct ParserOptions<'i> {
    /// Filename to use in error messages.
    pub filename: String,
    /// Whether to ignore invalid rules and declarations rather than erroring.
    pub error_recovery: bool,
    /// A list that will be appended to when a warning occurs.
    pub warnings: Option<Arc<RwLock<Vec<Error<CustomParseError<'i>>>>>>,
    p: PhantomData<&'i Self>,
}

impl<'i> ParserOptions<'i> {
    pub fn new() -> Self {
        ParserOptions { error_recovery: true, ..Default::default() }
    }

    #[inline]
    pub(crate) fn warn(&self, warning: ParseError<'i, CustomParseError<'i>>) {
        if let Some(warnings) = &self.warnings {
            if let Ok(mut warnings) = warnings.write() {
                warnings.push(Error::from(warning, self.filename.clone()));
            }
        }
    }
}
