mod test {
    use std::{
        collections::{HashMap, HashSet},
        hash::Hash,
    };

    use cssparser::*;
    use parcel_selectors::{
        context::{MatchingContext, MatchingMode, QuirksMode},
        matching::{matches_selector, matches_selector_list},
        OpaqueElement, SelectorList,
    };

    use crate::{pseudoclass, CustomParseError, SelectorIdent, SelectorParser, Selectors};

    fn parse<'i>(
        input: &'i str,
    ) -> Result<SelectorList<Selectors>, ParseError<'i, CustomParseError<'i>>> {
        let mut parser_input = ParserInput::new(input);
        let mut parser = Parser::new(&mut parser_input);
        SelectorList::parse(
            &SelectorParser {
                default_namespace: &None,
                is_nesting_allowed: true,
            },
            &mut parser,
            parcel_selectors::parser::NestingRequirement::None,
        )
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Entity(u32);

    use bitflags::bitflags;

    bitflags! {
        /// A bitflag of possible pseudoclasses.
        ///
        /// This type is part of the prelude.
        pub struct PseudoClass: u8 {
            const HOVER = 1;
            const OVER = 1 << 1;
            const ACTIVE = 1 << 2;
            const FOCUS = 1 << 3;
            const DISABLED = 1 << 4;
            const CHECKED = 1 << 5;
        }
    }

    #[derive(Debug)]
    pub struct Store {
        element: HashMap<Entity, String>,
        classes: HashMap<Entity, HashSet<String>>,
        pseudo_class: HashMap<Entity, PseudoClass>,
    }

    #[derive(Debug, Clone)]
    pub struct Node<'s> {
        entity: Entity,
        store: &'s Store,
    }

    impl<'i, 's> parcel_selectors::Element<'i> for Node<'s> {
        type Impl = Selectors;

        fn opaque(&self) -> parcel_selectors::OpaqueElement {
            OpaqueElement::new(self)
        }

        fn is_html_slot_element(&self) -> bool {
            false
        }

        fn parent_node_is_shadow_root(&self) -> bool {
            false
        }

        fn containing_shadow_host(&self) -> Option<Self> {
            None
        }

        fn parent_element(&self) -> Option<Self> {
            None
        }

        fn prev_sibling_element(&self) -> Option<Self> {
            None
        }

        fn next_sibling_element(&self) -> Option<Self> {
            None
        }

        fn is_empty(&self) -> bool {
            true
        }

        fn is_root(&self) -> bool {
            false
        }

        fn is_html_element_in_html_document(&self) -> bool {
            false
        }

        fn has_local_name(&self, local_name: &SelectorIdent<'i>) -> bool {
            if let Some(element) = self.store.element.get(&self.entity) {
                return element == local_name.0.as_ref();
            }

            false
        }

        fn has_namespace(
            &self,
            ns: &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::BorrowedNamespaceUrl,
        ) -> bool {
            false
        }

        fn is_part(
            &self,
            name: &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::Identifier,
        ) -> bool {
            false
        }

        fn imported_part(
            &self,
            name: &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::Identifier,
        ) -> Option<<Self::Impl as parcel_selectors::SelectorImpl<'i>>::Identifier> {
            None
        }

        fn is_pseudo_element(&self) -> bool {
            false
        }

        fn is_same_type(&self, other: &Self) -> bool {
            self.store.element.get(&self.entity) == other.store.element.get(&self.entity)
        }

        fn is_link(&self) -> bool {
            false
        }

        fn has_id(
            &self,
            id: &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::Identifier,
            case_sensitivity: parcel_selectors::attr::CaseSensitivity,
        ) -> bool {
            false
        }

        fn has_class(
            &self,
            name: &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::Identifier,
            case_sensitivity: parcel_selectors::attr::CaseSensitivity,
        ) -> bool {
            if let Some(classes) = self.store.classes.get(&self.entity) {
                return classes.contains(name.0.as_ref());
            }

            false
        }

        fn attr_matches(
            &self,
            ns: &parcel_selectors::attr::NamespaceConstraint<
                &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::NamespaceUrl,
            >,
            local_name: &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::LocalName,
            operation: &parcel_selectors::attr::AttrSelectorOperation<
                &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::AttrValue,
            >,
        ) -> bool {
            false
        }

        fn match_pseudo_element(
            &self,
            pe: &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::PseudoElement,
            context: &mut parcel_selectors::context::MatchingContext<'_, 'i, Self::Impl>,
        ) -> bool {
            false
        }

        fn match_non_ts_pseudo_class<F>(
            &self,
            pc: &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::NonTSPseudoClass,
            context: &mut parcel_selectors::context::MatchingContext<'_, 'i, Self::Impl>,
            flags_setter: &mut F,
        ) -> bool
        where
            F: FnMut(&Self, parcel_selectors::matching::ElementSelectorFlags),
        {
            if let Some(psudeo_class_flag) = self.store.pseudo_class.get(&self.entity) {
                match pc {
                    crate::PseudoClass::Hover => psudeo_class_flag.contains(PseudoClass::HOVER),
                    crate::PseudoClass::Active => todo!(),
                    crate::PseudoClass::Focus => todo!(),
                    crate::PseudoClass::FocusVisible => todo!(),
                    crate::PseudoClass::FocusWithin => todo!(),
                    crate::PseudoClass::Enabled => todo!(),
                    crate::PseudoClass::Disabled => todo!(),
                    crate::PseudoClass::ReadOnly => todo!(),
                    crate::PseudoClass::ReadWrite => todo!(),
                    crate::PseudoClass::PlaceHolderShown => todo!(),
                    crate::PseudoClass::Default => todo!(),
                    crate::PseudoClass::Checked => todo!(),
                    crate::PseudoClass::Indeterminate => todo!(),
                    crate::PseudoClass::Blank => todo!(),
                    crate::PseudoClass::Valid => todo!(),
                    crate::PseudoClass::Invalid => todo!(),
                    crate::PseudoClass::InRange => todo!(),
                    crate::PseudoClass::OutOfRange => todo!(),
                    crate::PseudoClass::Required => todo!(),
                    crate::PseudoClass::Optional => todo!(),
                    crate::PseudoClass::UserValid => todo!(),
                    crate::PseudoClass::UserInvalid => todo!(),
                    crate::PseudoClass::Lang(_) => todo!(),
                    crate::PseudoClass::Dir(_) => todo!(),
                    crate::PseudoClass::Custom(_) => todo!(),
                }
            } else {
                false
            }
        }
    }

    #[test]
    fn asterisk_match() {
        let mut store = Store {
            element: HashMap::new(),
            classes: HashMap::new(),
            pseudo_class: HashMap::new(),
        };

        let root = Entity(0);
        let child = Entity(1);

        store.element.insert(root, String::from("window"));
        store.element.insert(child, String::from("button"));

        let root_node = Node {
            entity: root,
            store: &store,
        };

        let child_node = Node {
            entity: child,
            store: &store,
        };

        if let Ok(selector_list) = parse("*") {
            let mut context =
                MatchingContext::new(MatchingMode::Normal, None, None, QuirksMode::NoQuirks);

            let result = matches_selector_list(&selector_list, &root_node, &mut context);

            println!("Result: {}", result);
        }
    }

    #[test]
    fn element_match() {
        let mut store = Store {
            element: HashMap::new(),
            classes: HashMap::new(),
            pseudo_class: HashMap::new(),
        };

        let root = Entity(0);
        let child = Entity(1);

        store.element.insert(root, String::from("window"));
        store.element.insert(child, String::from("button"));

        let root_node = Node {
            entity: root,
            store: &store,
        };

        let child_node = Node {
            entity: child,
            store: &store,
        };

        if let Ok(selector_list) = parse("window") {
            let mut context =
                MatchingContext::new(MatchingMode::Normal, None, None, QuirksMode::NoQuirks);

            let result = matches_selector_list(&selector_list, &root_node, &mut context);

            println!("Result: {}", result);
            assert_eq!(result, true);

            let result = matches_selector_list(&selector_list, &child_node, &mut context);

            println!("Result: {}", result);
            assert_eq!(result, false);
        }
    }

    #[test]
    fn class_match() {
        let mut store = Store {
            element: HashMap::new(),
            classes: HashMap::new(),
            pseudo_class: HashMap::new(),
        };

        let root = Entity(0);
        let child = Entity(1);

        store.classes.insert(root, HashSet::new());

        if let Some(classes) = store.classes.get_mut(&root) {
            classes.insert(String::from("foo"));
            classes.insert(String::from("bar"));
        }

        store.classes.insert(child, HashSet::new());

        if let Some(classes) = store.classes.get_mut(&child) {
            classes.insert(String::from("bar"));
        }

        let root_node = Node {
            entity: root,
            store: &store,
        };

        let child_node = Node {
            entity: child,
            store: &store,
        };

        if let Ok(selector_list) = parse(".foo") {
            let mut context =
                MatchingContext::new(MatchingMode::Normal, None, None, QuirksMode::NoQuirks);

            let result = matches_selector_list(&selector_list, &root_node, &mut context);
            assert_eq!(result, true);

            let result = matches_selector_list(&selector_list, &child_node, &mut context);
            assert_eq!(result, false);
        }

        if let Ok(selector_list) = parse(".bar") {
            let mut context =
                MatchingContext::new(MatchingMode::Normal, None, None, QuirksMode::NoQuirks);

            let result = matches_selector_list(&selector_list, &root_node, &mut context);
            assert_eq!(result, true);

            let result = matches_selector_list(&selector_list, &child_node, &mut context);
            assert_eq!(result, true);
        }

        if let Ok(selector_list) = parse(".foo.bar") {
            let mut context =
                MatchingContext::new(MatchingMode::Normal, None, None, QuirksMode::NoQuirks);

            let result = matches_selector_list(&selector_list, &root_node, &mut context);
            assert_eq!(result, true);

            let result = matches_selector_list(&selector_list, &child_node, &mut context);
            assert_eq!(result, false);
        }

        if let Ok(selector_list) = parse(".foo, .bar") {
            let mut context =
                MatchingContext::new(MatchingMode::Normal, None, None, QuirksMode::NoQuirks);

            let result = matches_selector_list(&selector_list, &root_node, &mut context);
            assert_eq!(result, true);

            let result = matches_selector_list(&selector_list, &child_node, &mut context);
            assert_eq!(result, true);
        }
    }

    #[test]
    fn pseudoclass_match() {
        let mut store = Store {
            element: HashMap::new(),
            classes: HashMap::new(),
            pseudo_class: HashMap::new(),
        };

        let root = Entity(0);
        let child = Entity(1);

        store.element.insert(root, String::from("window"));
        store.pseudo_class.insert(root, PseudoClass::empty());

        if let Some(pseudoclass) = store.pseudo_class.get_mut(&root) {
            pseudoclass.set(PseudoClass::HOVER, true);
        }

        store.element.insert(child, String::from("child"));

        let root_node = Node {
            entity: root,
            store: &store,
        };

        let child_node = Node {
            entity: child,
            store: &store,
        };

        if let Ok(selector_list) = parse("window:hover") {
            let mut context =
                MatchingContext::new(MatchingMode::Normal, None, None, QuirksMode::NoQuirks);

            let result = matches_selector_list(&selector_list, &root_node, &mut context);
            assert_eq!(result, true);

            let result = matches_selector_list(&selector_list, &child_node, &mut context);
            assert_eq!(result, false);
        }
    }
}
