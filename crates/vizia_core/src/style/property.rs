use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PropType {
    Units(Units),
    String(String),
}
