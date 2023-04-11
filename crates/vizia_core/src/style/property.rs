use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PropType {
    Units(Units),
    String(String),
}
