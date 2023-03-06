use crate::DashedIdent;

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyRule<'i> {
    pub name: DashedIdent<'i>,
}
