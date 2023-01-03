use super::Property;

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StyleRule {
    pub(crate) id: Rule,
    pub(crate) selectors: Vec<Selector>,
    pub(crate) properties: Vec<Property>,
}

impl std::fmt::Display for StyleRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for selector in self.selectors.iter() {
            write!(f, "{}", selector)?;
        }

        write!(f, " {{\n")?;

        for property in self.properties.iter() {
            write!(f, "    {}\n", property)?;
        }

        write!(f, "}}\n")?;

        Ok(())
    }
}

impl StyleRule {
    pub(crate) fn specificity(&self) -> Specificity {
        let mut specificity = Specificity([0, 0, 0]);
        for selector in self.selectors.iter() {
            specificity += selector.specificity();
        }

        return specificity;
    }
}
