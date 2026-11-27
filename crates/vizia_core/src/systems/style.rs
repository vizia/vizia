use crate::{cache::CachedData, prelude::*, text::resolved_text_direction};
#[cfg(feature = "rayon")]
use dashmap::{DashMap, ReadOnlyView};
use hashbrown::{HashMap, HashSet};
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use vizia_storage::{LayoutChildIterator, LayoutParentIterator, LayoutTreeIterator, TreeIterator};
use vizia_style::{
    Element, MatchingContext, MatchingMode, PseudoClass, QuirksMode, SelectorIdent, Selectors,
    matches_selector,
    precomputed_hash::PrecomputedHash,
    selectors::{
        OpaqueElement, SelectorImpl,
        attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint},
        bloom::BloomFilter,
        context::{MatchingForInvalidation, NeedsSelectorFlags, SelectorCaches},
        matching::ElementSelectorFlags,
        parser::{Component, NthType},
    },
};

/// A node used for style matching.
#[derive(Clone, Copy)]
pub(crate) struct Node<'s, 't> {
    entity: Entity,
    store: &'s Style,
    tree: &'t Tree<Entity>,
}

impl std::fmt::Debug for Node<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.entity)
    }
}

impl Node<'_, '_> {
    fn matches_direction(&self, expected: &Direction) -> bool {
        match expected {
            Direction::LeftToRight | Direction::RightToLeft => {
                resolved_text_direction(self.store, self.entity) == *expected
            }
            Direction::Auto => {
                self.store.direction.get(self.entity).copied() == Some(Direction::Auto)
            }
        }
    }
}

fn matches_language_range(language: &str, range: &str) -> bool {
    let mut language = language.split('-');
    let mut range = range.split('-');
    let Some(first_range) = range.next() else {
        return false;
    };
    let Some(first_language) = language.next() else {
        return false;
    };
    if first_range != "*"
        && (first_range.is_empty()
            || first_range.len() > 8
            || !first_range.bytes().all(|c| c.is_ascii_alphabetic())
            || !first_range.eq_ignore_ascii_case(first_language))
    {
        return false;
    }

    for subtag in range {
        if subtag == "*" {
            continue;
        }
        if subtag.is_empty()
            || subtag.len() > 8
            || !subtag.bytes().all(|c| c.is_ascii_alphanumeric())
        {
            return false;
        }
        loop {
            let Some(candidate) = language.next() else {
                return false;
            };
            if subtag.eq_ignore_ascii_case(candidate) {
                break;
            }
            // Extended language-range filtering cannot skip an extension singleton.
            if candidate.len() == 1 {
                return false;
            }
        }
    }
    true
}

/// Used for selector matching.
impl Element for Node<'_, '_> {
    type Impl = Selectors;

    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(&self.tree.parent[self.entity.index()])
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
        self.tree.get_layout_parent(self.entity).map(|parent| Node {
            entity: parent,
            store: self.store,
            tree: self.tree,
        })
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.tree.get_prev_layout_sibling(self.entity).map(|parent| Node {
            entity: parent,
            store: self.store,
            tree: self.tree,
        })
    }

    fn next_sibling_element(&self) -> Option<Self> {
        self.tree.get_next_layout_sibling(self.entity).map(|parent| Node {
            entity: parent,
            store: self.store,
            tree: self.tree,
        })
    }

    fn is_empty(&self) -> bool {
        !self.tree.has_children(self.entity)
    }

    fn is_root(&self) -> bool {
        self.entity == Entity::root()
    }

    fn is_html_element_in_html_document(&self) -> bool {
        false
    }

    fn has_local_name(&self, local_name: &SelectorIdent) -> bool {
        if let Some(element) = self.store.element.get(self.entity) {
            return element == &local_name.precomputed_hash();
        }

        false
    }

    fn has_namespace(&self, _ns: &<Self::Impl as SelectorImpl>::BorrowedNamespaceUrl) -> bool {
        false
    }

    fn is_part(&self, _name: &<Self::Impl as SelectorImpl>::Identifier) -> bool {
        false
    }

    fn imported_part(
        &self,
        _name: &<Self::Impl as SelectorImpl>::Identifier,
    ) -> Option<<Self::Impl as SelectorImpl>::Identifier> {
        None
    }

    fn is_pseudo_element(&self) -> bool {
        false
    }

    fn is_same_type(&self, other: &Self) -> bool {
        if let Some(element) = self.store.element.get(self.entity)
            && let Some(other_element) = self.store.element.get(other.entity)
        {
            return element == other_element;
        }

        false
    }

    fn is_link(&self) -> bool {
        false
    }

    fn has_id(
        &self,
        name: &<Self::Impl as SelectorImpl>::Identifier,
        _case_sensitivity: CaseSensitivity,
    ) -> bool {
        if let Some(id) = self.store.ids.get(self.entity) { *id == name.0 } else { false }
    }

    fn has_class(
        &self,
        name: &<Self::Impl as SelectorImpl>::Identifier,
        _case_sensitivity: CaseSensitivity,
    ) -> bool {
        if let Some(classes) = self.store.classes.get(self.entity) {
            return classes.contains(&name.0);
        }

        false
    }

    fn attr_matches(
        &self,
        _ns: &NamespaceConstraint<&<Self::Impl as SelectorImpl>::NamespaceUrl>,
        _local_name: &<Self::Impl as SelectorImpl>::LocalName,
        _operation: &AttrSelectorOperation<&<Self::Impl as SelectorImpl>::AttrValue>,
    ) -> bool {
        false
    }

    fn match_pseudo_element(
        &self,
        _pe: &<Self::Impl as SelectorImpl>::PseudoElement,
        _context: &mut MatchingContext<'_, Self::Impl>,
    ) -> bool {
        false
    }

    fn match_non_ts_pseudo_class(
        &self,
        pc: &<Self::Impl as SelectorImpl>::NonTSPseudoClass,
        _context: &mut MatchingContext<'_, Self::Impl>,
    ) -> bool {
        match pc {
            PseudoClass::Hover => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::HOVER)),
            PseudoClass::Active => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::ACTIVE)),
            PseudoClass::Over => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::OVER)),
            PseudoClass::Focus => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::FOCUS)),
            PseudoClass::FocusVisible => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::FOCUS_VISIBLE)),
            PseudoClass::FocusWithin => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::FOCUS_WITHIN)),
            PseudoClass::Enabled => {
                self.store.disabled.get(self.entity).map(|disabled| !*disabled).unwrap_or(true)
            }
            PseudoClass::Disabled => {
                self.store.disabled.get(self.entity).copied().unwrap_or_default()
            }
            PseudoClass::ReadOnly => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::READ_ONLY)),
            PseudoClass::ReadWrite => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::READ_WRITE)),
            PseudoClass::PlaceholderShown => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::PLACEHOLDER_SHOWN)),
            PseudoClass::Default => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::DEFAULT)),
            PseudoClass::Checked => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::CHECKED)),
            PseudoClass::Indeterminate => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::INDETERMINATE)),
            PseudoClass::Blank => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::BLANK)),
            PseudoClass::Valid => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::VALID)),
            PseudoClass::Invalid => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::INVALID)),
            PseudoClass::InRange => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::IN_RANGE)),
            PseudoClass::OutOfRange => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::OUT_OF_RANGE)),
            PseudoClass::Required => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::REQUIRED)),
            PseudoClass::Optional => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::OPTIONAL)),
            PseudoClass::UserValid => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::USER_VALID)),
            PseudoClass::UserInvalid => self
                .store
                .pseudo_classes
                .get(self.entity)
                .is_some_and(|flags| flags.contains(PseudoClassFlags::USER_INVALID)),
            PseudoClass::Lang(ranges) => {
                self.store.selector_language.as_ref().is_some_and(|(_, language)| {
                    ranges.iter().any(|range| matches_language_range(language, range))
                })
            }
            PseudoClass::Dir(direction) => self.matches_direction(direction),
            PseudoClass::Has(selector_list) => self.match_has(selector_list, _context),
            PseudoClass::Custom(name) => {
                log::warn!("Unsupported custom pseudo-class :{name}");
                false
            }
        }
    }

    fn first_element_child(&self) -> Option<Self> {
        None
    }

    fn apply_selector_flags(&self, _flags: ElementSelectorFlags) {}

    fn has_custom_state(&self, _name: &<Self::Impl as SelectorImpl>::Identifier) -> bool {
        false
    }

    fn add_element_unique_hashes(
        &self,
        _filter: &mut vizia_style::selectors::bloom::BloomFilter,
    ) -> bool {
        false
    }
}

impl Node<'_, '_> {
    fn match_has(
        &self,
        selector_list: &vizia_style::SelectorList<Selectors>,
        context: &mut MatchingContext<'_, Selectors>,
    ) -> bool {
        use vizia_style::selectors::parser::Combinator;

        // Each selector in the list was parsed as a relative selector with ParseRelative::ForHas.
        // The leading combinator is stored at parse-order index 1 (index 0 is RelativeSelectorAnchor).
        // We dispatch to the correct traversal based on that combinator.
        selector_list.slice().iter().any(|selector| {
            let leading = selector.combinator_at_parse_order(1);
            match leading {
                // > — only direct children
                Combinator::Child => self
                    .any_child(|n| vizia_style::matches_selector(selector, 0, None, *n, context)),
                // space — all descendants
                Combinator::Descendant => self.any_descendant(|n| {
                    vizia_style::matches_selector(selector, 0, None, *n, context)
                }),
                // + — next sibling only
                Combinator::NextSibling => self
                    .next_sibling_element()
                    .is_some_and(|n| vizia_style::matches_selector(selector, 0, None, n, context)),
                // ~ — any following sibling
                Combinator::LaterSibling => self.any_following_sibling(|n| {
                    vizia_style::matches_selector(selector, 0, None, *n, context)
                }),
                // Unexpected in :has() context — fall back to descendants
                _ => self.any_descendant(|n| {
                    vizia_style::matches_selector(selector, 0, None, *n, context)
                }),
            }
        })
    }

    fn any_child(&self, mut f: impl FnMut(&Node) -> bool) -> bool {
        let Some(mut child) = self.tree.get_layout_first_child(self.entity).map(|e| Node {
            entity: e,
            store: self.store,
            tree: self.tree,
        }) else {
            return false;
        };
        loop {
            if f(&child) {
                return true;
            }
            match child.next_sibling_element() {
                Some(next) => child = next,
                None => return false,
            }
        }
    }

    fn any_descendant(&self, mut f: impl FnMut(&Node) -> bool) -> bool {
        self.any_descendant_inner(&mut f)
    }

    fn any_descendant_inner(&self, f: &mut impl FnMut(&Node) -> bool) -> bool {
        let Some(mut child) = self.tree.get_layout_first_child(self.entity).map(|e| Node {
            entity: e,
            store: self.store,
            tree: self.tree,
        }) else {
            return false;
        };
        loop {
            if f(&child) || child.any_descendant_inner(f) {
                return true;
            }
            match child.next_sibling_element() {
                Some(next) => child = next,
                None => return false,
            }
        }
    }

    fn any_following_sibling(&self, mut f: impl FnMut(&Node) -> bool) -> bool {
        let Some(mut sibling) = self.next_sibling_element() else {
            return false;
        };
        loop {
            if f(&sibling) {
                return true;
            }
            match sibling.next_sibling_element() {
                Some(next) => sibling = next,
                None => return false,
            }
        }
    }
}

/// Iterates the entities in the subtrees of the given roots, parents before children, skipping
/// ignored entities such as bindings.
fn subtrees<'a>(tree: &'a Tree<Entity>, roots: &'a [Entity]) -> impl Iterator<Item = Entity> + 'a {
    roots.iter().flat_map(move |root| LayoutTreeIterator::subtree(tree, *root))
}

fn inheritance_roots(
    tree: &Tree<Entity>,
    entities: &[Entity],
    restyle: &HashSet<Entity>,
) -> Vec<Entity> {
    entities
        .iter()
        .copied()
        .filter(|entity| {
            let mut current = *entity;
            while let Some(parent) = tree.get_parent(current) {
                if restyle.contains(&parent) {
                    return false;
                }
                current = parent;
            }
            true
        })
        .collect()
}

/// Link inheritable inline properties to their parent within the subtrees of the given roots.
pub(crate) fn inline_inheritance_system(
    cx: &mut Context,
    roots: &[Entity],
    redraw_entities: &mut Vec<Entity>,
) {
    for entity in subtrees(&cx.tree, roots) {
        if let Some(parent) = cx.tree.get_layout_parent(entity) {
            if cx.style.disabled.inherit_inline(entity, parent)
                | cx.style.caret_color.inherit_inline(entity, parent)
                | cx.style.selection_color.inherit_inline(entity, parent)
                | cx.style.fill.inherit_inline(entity, parent)
            {
                redraw_entities.push(entity);
            }

            if cx.style.font_color.inherit_inline(entity, parent)
                | cx.style.font_size.inherit_inline(entity, parent)
                | cx.style.letter_spacing.inherit_inline(entity, parent)
                | cx.style.line_height.inherit_inline(entity, parent)
                | cx.style.font_family.inherit_inline(entity, parent)
                | cx.style.font_weight.inherit_inline(entity, parent)
                | cx.style.font_slant.inherit_inline(entity, parent)
                | cx.style.font_width.inherit_inline(entity, parent)
                | cx.style.text_decoration_line.inherit_inline(entity, parent)
                | cx.style.text_decoration_style.inherit_inline(entity, parent)
                | cx.style.text_decoration_color.inherit_inline(entity, parent)
                | cx.style.text_stroke_width.inherit_inline(entity, parent)
                | cx.style.text_stroke_style.inherit_inline(entity, parent)
                | cx.style.font_variation_settings.inherit_inline(entity, parent)
                | cx.style.direction.inherit_inline(entity, parent)
            {
                cx.style.needs_text_update(entity);
                // Only this entity's text changed; relayout incrementally from it (morphorm
                // restarts from at least its parent) rather than forcing a full tree relayout.
                cx.style.needs_relayout(entity);
            }
        }
    }
}

/// Carries each entity's text invalidation down to its descendants. An inline text value can change
/// without changing the inherited storage index, so the inheritance walk does not see the change.
///
/// Runs once per frame at the start of `text_system`, after binding, style and animation updates
/// have all marked entities, so it sees every invalidation the paragraphs are built from.
pub(crate) fn text_invalidation_system(cx: &mut Context) {
    let root = Entity::root();
    let invalidated = cx
        .style
        .text_construction
        .iter()
        .copied()
        .filter(|entity| {
            cx.entity_manager.is_alive(*entity)
                && (*entity == root || cx.tree.get_parent(*entity).is_some())
        })
        .collect::<Vec<_>>();
    let roots = inheritance_roots(&cx.tree, &invalidated, &cx.style.text_construction);
    // A span is shaped as part of its enclosing paragraph, so its change rebuilds that paragraph.
    // `add_block` includes spans whatever their `display`, so the chain of spans is followed to the
    // first non-span parent, rebuilding each displayed entity on the way: a displayed span builds its
    // own paragraph as well.
    for entity in invalidated {
        if cx.style.text_span.get(entity).copied().unwrap_or_default() {
            for current in LayoutParentIterator::new(&cx.tree, entity) {
                if cx.style.display.get(current).copied().unwrap_or_default() != Display::None {
                    cx.style.needs_text_update(current);
                }
                if !cx.style.text_span.get(current).copied().unwrap_or_default() {
                    break;
                }
            }
        }
    }
    for entity in subtrees(&cx.tree, &roots) {
        if let Some(parent) = cx.tree.get_layout_parent(entity)
            && cx.style.text_construction.contains(&parent)
        {
            // The text system relayouts every paragraph it rebuilds, so no relayout is needed here.
            cx.style.needs_text_update(entity);
        }
    }
}

/// Link inheritable shared properties to their parent within the subtrees of the given roots.
pub(crate) fn shared_inheritance_system(
    cx: &mut Context,
    roots: &[Entity],
    redraw_entities: &mut Vec<Entity>,
) {
    for entity in subtrees(&cx.tree, roots) {
        if let Some(parent) = cx.tree.get_layout_parent(entity) {
            if cx.style.font_color.inherit_shared(entity, parent)
                | cx.style.font_size.inherit_shared(entity, parent)
                | cx.style.letter_spacing.inherit_shared(entity, parent)
                | cx.style.line_height.inherit_shared(entity, parent)
                | cx.style.font_family.inherit_shared(entity, parent)
                | cx.style.font_weight.inherit_shared(entity, parent)
                | cx.style.font_slant.inherit_shared(entity, parent)
                | cx.style.font_width.inherit_shared(entity, parent)
                | cx.style.text_decoration_line.inherit_shared(entity, parent)
                | cx.style.text_decoration_style.inherit_shared(entity, parent)
                | cx.style.text_decoration_color.inherit_shared(entity, parent)
                | cx.style.text_stroke_width.inherit_shared(entity, parent)
                | cx.style.text_stroke_style.inherit_shared(entity, parent)
                | cx.style.font_variation_settings.inherit_shared(entity, parent)
                | cx.style.direction.inherit_shared(entity, parent)
            {
                cx.style.needs_text_update(entity);
                // Only this entity's text changed; relayout incrementally from it.
                cx.style.needs_relayout(entity);
            }

            if cx.style.disabled.inherit_shared(entity, parent)
                | cx.style.caret_color.inherit_shared(entity, parent)
                | cx.style.selection_color.inherit_shared(entity, parent)
                | cx.style.fill.inherit_shared(entity, parent)
            {
                redraw_entities.push(entity);
            }
        }
    }
}

fn link_variable_data(
    style: &mut Style,
    entity: Entity,
    redraw_entities: &mut Vec<Entity>,
    matched_rules: &[(Rule, u32)],
) {
    // Fast path: no rules matched, nothing to link.
    if matched_rules.is_empty() {
        return;
    }

    // For each custom-property type we need to:
    //   1. Resolve all variable values (pure immutable read of the whole map).
    //   2. Iterate the map mutably to call link_with_resolved().
    //
    // Each block-scoped `props` reference lets NLL end the immutable borrow before
    // the subsequent values_mut() call, avoiding the old remove/re-insert
    // borrow-splitting trick and eliminating the per-entity Vec allocation.

    // -- Color variables --
    let resolved_colors: HashMap<u64, _> = {
        let props = &style.custom_color_props;
        props
            .iter()
            .filter_map(|(h, p)| p.get_with_variables(entity, props).map(|v| (*h, v)))
            .collect()
    };
    for prop in style.custom_color_props.values_mut() {
        if prop.link_with_resolved(entity, matched_rules, &resolved_colors) {
            redraw_entities.push(entity);
        }
    }

    // -- Length variables --
    let resolved_lengths: HashMap<u64, _> = {
        let props = &style.custom_length_props;
        props
            .iter()
            .filter_map(|(h, p)| p.get_with_variables(entity, props).map(|v| (*h, v)))
            .collect()
    };
    for prop in style.custom_length_props.values_mut() {
        if prop.link_with_resolved(entity, matched_rules, &resolved_lengths) {
            redraw_entities.push(entity);
        }
    }

    // -- Font-size variables --
    let resolved_font_sizes: HashMap<u64, _> = {
        let props = &style.custom_font_size_props;
        props
            .iter()
            .filter_map(|(h, p)| p.get_with_variables(entity, props).map(|v| (*h, v)))
            .collect()
    };
    for prop in style.custom_font_size_props.values_mut() {
        if prop.link_with_resolved(entity, matched_rules, &resolved_font_sizes) {
            redraw_entities.push(entity);
        }
    }

    // -- Letter-spacing variables --
    let resolved_letter_spacings: HashMap<u64, _> = {
        let props = &style.custom_letter_spacing_props;
        props
            .iter()
            .filter_map(|(h, p)| p.get_with_variables(entity, props).map(|v| (*h, v)))
            .collect()
    };
    for prop in style.custom_letter_spacing_props.values_mut() {
        if prop.link_with_resolved(entity, matched_rules, &resolved_letter_spacings) {
            redraw_entities.push(entity);
        }
    }

    // -- Line-height variables --
    let resolved_line_heights: HashMap<u64, _> = {
        let props = &style.custom_line_height_props;
        props
            .iter()
            .filter_map(|(h, p)| p.get_with_variables(entity, props).map(|v| (*h, v)))
            .collect()
    };
    for prop in style.custom_line_height_props.values_mut() {
        if prop.link_with_resolved(entity, matched_rules, &resolved_line_heights) {
            redraw_entities.push(entity);
        }
    }

    // -- Units variables --
    let resolved_units: HashMap<u64, _> = {
        let props = &style.custom_units_props;
        props
            .iter()
            .filter_map(|(h, p)| p.get_with_variables(entity, props).map(|v| (*h, v)))
            .collect()
    };
    for prop in style.custom_units_props.values_mut() {
        if prop.link_with_resolved(entity, matched_rules, &resolved_units) {
            redraw_entities.push(entity);
        }
    }

    // -- Opacity variables --
    let resolved_opacities: HashMap<u64, _> = {
        let props = &style.custom_opacity_props;
        props
            .iter()
            .filter_map(|(h, p)| p.get_with_variables(entity, props).map(|v| (*h, v)))
            .collect()
    };
    for prop in style.custom_opacity_props.values_mut() {
        if prop.link_with_resolved(entity, matched_rules, &resolved_opacities) {
            redraw_entities.push(entity);
        }
    }

    let resolved_shadows: HashMap<u64, _> = {
        let props = &style.custom_shadow_props;
        props
            .iter()
            .filter_map(|(h, p)| p.get_with_variables(entity, props).map(|v| (*h, v)))
            .collect()
    };
    for prop in style.custom_shadow_props.values_mut() {
        if prop.link_with_resolved(entity, matched_rules, &resolved_shadows) {
            redraw_entities.push(entity);
        }
    }
}

fn inherit_variable_data(
    style: &mut Style,
    tree: &Tree<Entity>,
    roots: &[Entity],
    redraw_entities: &mut Vec<Entity>,
) {
    for entity in subtrees(tree, roots) {
        if let Some(parent) = tree.get_layout_parent(entity) {
            for prop in style.custom_color_props.values_mut() {
                if prop.inherit_shared(entity, parent) {
                    redraw_entities.push(entity);
                }
            }
            for prop in style.custom_length_props.values_mut() {
                if prop.inherit_shared(entity, parent) {
                    redraw_entities.push(entity);
                }
            }
            for prop in style.custom_font_size_props.values_mut() {
                if prop.inherit_shared(entity, parent) {
                    redraw_entities.push(entity);
                }
            }
            for prop in style.custom_letter_spacing_props.values_mut() {
                if prop.inherit_shared(entity, parent) {
                    redraw_entities.push(entity);
                }
            }
            for prop in style.custom_line_height_props.values_mut() {
                if prop.inherit_shared(entity, parent) {
                    redraw_entities.push(entity);
                }
            }
            for prop in style.custom_units_props.values_mut() {
                if prop.inherit_shared(entity, parent) {
                    redraw_entities.push(entity);
                }
            }
            for prop in style.custom_opacity_props.values_mut() {
                if prop.inherit_shared(entity, parent) {
                    redraw_entities.push(entity);
                }
            }
            for prop in style.custom_shadow_props.values_mut() {
                if prop.inherit_shared(entity, parent) {
                    redraw_entities.push(entity);
                }
            }
        }
    }
}

fn link_style_data(
    style: &mut Style,
    cache: &mut CachedData,
    tree: &Tree<Entity>,
    entity: Entity,
    redraw_entities: &mut Vec<Entity>,
    matched_rules: &[(Rule, u32)],
) {
    let mut should_relayout = false;
    let mut should_redraw = false;
    let mut should_reflow = false;
    let mut should_retransform = false;
    let mut should_reclip = false;

    // Display
    if style.display.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
        should_reflow = true;
    }

    if style.visibility.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.z_index.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.ignore_clipping.link(entity, matched_rules) {
        should_reclip = true;
        should_redraw = true;
    }

    if style.overflowx.link(entity, matched_rules) {
        should_reclip = true;
        should_redraw = true;
    }

    if style.overflowy.link(entity, matched_rules) {
        should_reclip = true;
        should_redraw = true;
    }

    if style.clip_path.link(entity, matched_rules) {
        should_reclip = true;
        should_redraw = true;
    }

    if style.filter.link(entity, matched_rules) {
        style.filter_entities.insert(entity);
        should_redraw = true;
    }

    if style.backdrop_filter.link(entity, matched_rules) {
        style.filter_entities.insert(entity);
        should_redraw = true;
    }

    if style.blend_mode.link(entity, matched_rules) {
        should_redraw = true;
    }

    // Opacity
    if style.opacity.link(entity, matched_rules, &style.custom_opacity_props) {
        should_redraw = true;
    }

    // Grid
    if style.grid_columns.link(entity, matched_rules) {
        should_relayout = true;
    }

    if style.grid_rows.link(entity, matched_rules) {
        should_relayout = true;
    }

    if style.column_start.link(entity, matched_rules) {
        should_relayout = true;
    }

    if style.column_span.link(entity, matched_rules) {
        should_relayout = true;
    }

    if style.row_start.link(entity, matched_rules) {
        should_relayout = true;
    }

    if style.row_span.link(entity, matched_rules) {
        should_relayout = true;
    }

    // Position

    if style.left.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.right.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.top.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.bottom.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    // Size
    if style.width.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.height.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.aspect_ratio.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    // Size Constraints
    if style.max_width.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.min_width.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.max_height.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.min_height.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    // Gap Constraints
    if style.max_horizontal_gap.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.min_horizontal_gap.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.max_vertical_gap.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.min_vertical_gap.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    // Border widths
    if style.border_top_width.link(entity, matched_rules, &style.custom_length_props) {
        should_relayout = true;
        should_redraw = true;
        cache.path.remove(entity);
    }

    if style.border_right_width.link(entity, matched_rules, &style.custom_length_props) {
        should_relayout = true;
        should_redraw = true;
        cache.path.remove(entity);
    }

    if style.border_bottom_width.link(entity, matched_rules, &style.custom_length_props) {
        should_relayout = true;
        should_redraw = true;
        cache.path.remove(entity);
    }

    if style.border_left_width.link(entity, matched_rules, &style.custom_length_props) {
        should_relayout = true;
        should_redraw = true;
        cache.path.remove(entity);
    }

    // Border colors
    if style.border_top_color.link(entity, matched_rules, &style.custom_color_props) {
        should_redraw = true;
    }

    if style.border_right_color.link(entity, matched_rules, &style.custom_color_props) {
        should_redraw = true;
    }

    if style.border_bottom_color.link(entity, matched_rules, &style.custom_color_props) {
        should_redraw = true;
    }

    if style.border_left_color.link(entity, matched_rules, &style.custom_color_props) {
        should_redraw = true;
    }

    // Border styles
    if style.border_top_style.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.border_right_style.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.border_bottom_style.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.border_left_style.link(entity, matched_rules) {
        should_redraw = true;
    }

    // Corner

    if style.corner_top_left_radius.link(entity, matched_rules, &style.custom_length_props) {
        should_redraw = true;
        cache.path.remove(entity);
    }

    if style.corner_top_right_radius.link(entity, matched_rules, &style.custom_length_props) {
        should_redraw = true;
        cache.path.remove(entity);
    }

    if style.corner_bottom_left_radius.link(entity, matched_rules, &style.custom_length_props) {
        should_redraw = true;
        cache.path.remove(entity);
    }

    if style.corner_bottom_right_radius.link(entity, matched_rules, &style.custom_length_props) {
        should_redraw = true;
        cache.path.remove(entity);
    }

    if style.outline_width.link(entity, matched_rules, &style.custom_length_props) {
        should_redraw = true;
    }

    if style.outline_color.link(entity, matched_rules, &style.custom_color_props) {
        should_redraw = true;
    }

    if style.outline_offset.link(entity, matched_rules, &style.custom_length_props) {
        should_redraw = true;
    }

    if style.layout_type.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.position_type.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.alignment.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.wrap.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    // Background
    if style.background_color.link(entity, matched_rules, &style.custom_color_props) {
        should_redraw = true;
    }

    if style.background_image.link(entity, matched_rules) {
        style.background_image_entities.insert(entity);
        should_redraw = true;
    }

    if style.background_position.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.background_size.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.background_repeat.link(entity, matched_rules) {
        should_redraw = true;
    }

    // Font
    if style.font_color.link(entity, matched_rules, &style.custom_color_props) {
        should_redraw = true;
        should_reflow = true;
    }

    if style.font_size.link(entity, matched_rules, &style.custom_font_size_props) {
        should_relayout = true;
        should_redraw = true;
        should_reflow = true;
    }

    if style.letter_spacing.link(entity, matched_rules, &style.custom_letter_spacing_props) {
        should_relayout = true;
        should_redraw = true;
        should_reflow = true;
    }

    if style.font_family.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
        should_reflow = true;
    }

    if style.font_weight.link(entity, matched_rules) {
        should_redraw = true;
        should_relayout = true;
        should_reflow = true;
        style.needs_text_update(entity);
    }

    if style.font_slant.link(entity, matched_rules) {
        should_redraw = true;
        should_relayout = true;
        should_reflow = true;
    }

    if style.font_width.link(entity, matched_rules) {
        should_redraw = true;
        should_relayout = true;
        should_reflow = true;
    }

    if style.font_variation_settings.link(entity, matched_rules) {
        should_redraw = true;
        should_relayout = true;
        should_reflow = true;
    }

    if style.text_wrap.link(entity, matched_rules) {
        should_redraw = true;
        should_relayout = true;
        should_reflow = true;
    }

    if style.text_align.link(entity, matched_rules) {
        should_redraw = true;
        should_reflow = true;
    }

    if style.text_overflow.link(entity, matched_rules) {
        should_redraw = true;
        should_reflow = true;
    }

    if style.line_height.link(entity, matched_rules, &style.custom_line_height_props) {
        should_redraw = true;
        should_relayout = true;
        should_reflow = true;
    }

    if style.line_clamp.link(entity, matched_rules) {
        should_redraw = true;
        should_reflow = true;
    }

    if style.selection_color.link(entity, matched_rules, &style.custom_color_props) {
        should_redraw = true;
    }

    if style.caret_color.link(entity, matched_rules, &style.custom_color_props) {
        should_redraw = true;
    }

    if style.text_decoration_line.link(entity, matched_rules) {
        should_redraw = true;
        should_reflow = true;
    }

    if style.text_decoration_style.link(entity, matched_rules) {
        should_redraw = true;
        should_reflow = true;
    }

    if style.text_decoration_color.link(entity, matched_rules, &style.custom_color_props) {
        should_redraw = true;
        should_reflow = true;
    }

    if style.text_stroke_width.link(entity, matched_rules) {
        should_redraw = true;
        should_reflow = true;
    }

    if style.text_stroke_style.link(entity, matched_rules) {
        should_redraw = true;
        should_reflow = true;
    }

    // Outer Shadow
    if style.shadow.link(entity, matched_rules, &style.custom_shadow_props) {
        should_redraw = true;
    }

    if style.padding_left.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.padding_right.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.padding_top.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.padding_bottom.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.vertical_gap.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.horizontal_gap.link(entity, matched_rules, &style.custom_units_props) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.cursor.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.pointer_events.link(entity, matched_rules) {
        should_redraw = true;
    }

    // Transform
    if style.transform.link(entity, matched_rules) {
        should_retransform = true;
        should_redraw = true;
    }

    if style.transform_origin.link(entity, matched_rules) {
        should_retransform = true;
        should_redraw = true;
    }

    if style.translate.link(entity, matched_rules) {
        should_retransform = true;
        should_redraw = true;
    }

    if style.rotate.link(entity, matched_rules) {
        should_retransform = true;
        should_redraw = true;
    }

    if style.scale.link(entity, matched_rules) {
        should_retransform = true;
        should_redraw = true;
    }

    if style.fill.link(entity, matched_rules, &style.custom_color_props) {
        should_redraw = true;
    }

    //
    if should_relayout {
        // Register the specific entity for incremental relayout. Setting the RELAYOUT system flag
        // alone would leave `style.relayout` empty and force a full tree relayout (see
        // `layout_system`), e.g. every time a popover toggles its `display`.
        style.needs_relayout(entity);
    }

    if should_redraw {
        redraw_entities.push(entity);
    }

    if should_retransform {
        style.needs_retransform(entity);
    }

    if should_reclip {
        style.needs_reclip(entity);
    }

    if should_reflow {
        let iter = LayoutParentIterator::new(tree, entity);
        for parent in iter {
            if style.display.get(parent).copied().unwrap_or_default() != Display::None {
                style.needs_text_update(parent);
                break;
            }
        }
    }
}

fn compute_matched_direction_rules(
    entity: Entity,
    store: &Style,
    tree: &Tree<Entity>,
    bloom: &BloomFilter,
) -> Vec<(Rule, u32)> {
    let mut matched_rules = Vec::with_capacity(4);

    let mut cache = SelectorCaches::default();
    let mut context = MatchingContext::new(
        MatchingMode::Normal,
        Some(bloom),
        &mut cache,
        QuirksMode::NoQuirks,
        NeedsSelectorFlags::Yes,
        MatchingForInvalidation::No,
    );

    let node = Node { entity, store, tree };

    for (rule_id, rule) in store.rules.iter() {
        if !store.direction.has_rule(*rule_id) {
            continue;
        }

        let matches = matches_selector(&rule.selector, 0, Some(&rule.hashes), node, &mut context);

        if matches {
            matched_rules.push((*rule_id, rule.selector.specificity()));
        }
    }

    sort_matched_rules(&mut matched_rules);
    matched_rules
}

fn sort_matched_rules(matched_rules: &mut [(Rule, u32)]) {
    matched_rules.sort_by(|(rule_a, specificity_a), (rule_b, specificity_b)| {
        specificity_b.cmp(specificity_a).then_with(|| rule_b.index().cmp(&rule_a.index()))
    });
}

/// Compute a list of matching style rules for a given entity.
pub(crate) fn compute_matched_rules(
    entity: Entity,
    store: &Style,
    tree: &Tree<Entity>,
    bloom: &BloomFilter,
) -> Vec<(Rule, u32)> {
    let mut matched_rules = Vec::with_capacity(16);

    let mut cache = SelectorCaches::default();
    let mut context = MatchingContext::new(
        MatchingMode::Normal,
        Some(bloom),
        &mut cache,
        QuirksMode::NoQuirks,
        NeedsSelectorFlags::Yes,
        MatchingForInvalidation::No,
    );

    let node = Node { entity, store, tree };

    for (rule_id, rule) in store.rules.iter() {
        let matches = matches_selector(&rule.selector, 0, Some(&rule.hashes), node, &mut context);

        if matches {
            matched_rules.push((*rule_id, rule.selector.specificity()));
        }
    }

    sort_matched_rules(&mut matched_rules);
    matched_rules
}

// Two siblings can share matched rules only if every local input to selector matching is equal.
// A missing element, class set or pseudo-class set is a value of its own: an entity without an
// element does not match type selectors, and an entity without style data must not take on the
// rules matched by a sibling that has some.
fn has_same_selector(style: &Style, entity1: Entity, entity2: Entity) -> bool {
    let id1 = if let Some(id) = style.ids.get(entity1) { id } else { "" };
    let id2 = if let Some(id) = style.ids.get(entity2) { id } else { "" };

    style.element.get(entity1) == style.element.get(entity2)
        && id1 == id2
        && style.classes.get(entity1) == style.classes.get(entity2)
        && style.pseudo_classes.get(entity1).map(|flags| flags.bits())
            == style.pseudo_classes.get(entity2).map(|flags| flags.bits())
        && style.disabled.get(entity1).copied().unwrap_or_default()
            == style.disabled.get(entity2).copied().unwrap_or_default()
        && style.direction.get(entity1) == style.direction.get(entity2)
        && (!matches!(style.direction.get(entity1), Some(Direction::Auto))
            || resolved_text_direction(style, entity1) == resolved_text_direction(style, entity2))
}

fn has_nth_child_rule(style: &Style, rules: &[(Rule, u32)]) -> bool {
    for (rule, _) in rules {
        let Some(style_rule) = style.rules.get(rule) else { continue };
        for component in style_rule.selector.iter() {
            let Component::Nth(n) = component else { continue };
            if let NthType::Child | NthType::LastChild | NthType::OnlyChild = n.ty {
                return true;
            }
        }
    }
    false
}

pub(crate) fn compute_element_hash(
    entity: Entity,
    tree: &Tree<Entity>,
    style: &Style,
    bloom: &mut BloomFilter,
) {
    let parent_iter = LayoutParentIterator::new(tree, entity);

    for ancestor in parent_iter {
        if let Some(element) = style.element.get(ancestor) {
            bloom.insert_hash(*element);
        }

        if let Some(id) = style.ids.get(ancestor) {
            bloom.insert_hash(fxhash::hash32(id));
        }

        if let Some(classes) = style.classes.get(ancestor) {
            for class in classes {
                bloom.insert_hash(fxhash::hash32(class));
            }
        }
    }
}

struct MatchedRulesCache {
    pub entity: Entity,
    pub rules: Vec<(Rule, u32)>,
}

struct MatchedRules {
    #[cfg(feature = "rayon")]
    cache: ReadOnlyView<Entity, Vec<MatchedRulesCache>>,
    #[cfg(not(feature = "rayon"))]
    cache: HashMap<Entity, Vec<MatchedRulesCache>>,
    // Stores the key/index into the cache to get the rules for a given entity.
    rules: HashMap<Entity, (Entity, usize)>,
}

impl MatchedRules {
    #[cfg(not(feature = "rayon"))]
    fn build(entities: &[Entity], style: &Style, tree: &Tree<Entity>, share_matches: bool) -> Self {
        let filter = &mut BloomFilter::default();

        let mut cache = HashMap::new();
        let rules = entities
            .iter()
            .filter_map(|entity| {
                Self::build_inner(*entity, style, tree, filter, share_matches, &mut cache)
            })
            .collect();

        Self { rules, cache }
    }

    #[cfg(feature = "rayon")]
    fn build_parallel(
        entities: &[Entity],
        style: &Style,
        tree: &Tree<Entity>,
        share_matches: bool,
    ) -> Self {
        let num_threads = std::thread::available_parallelism().map_or(1, |n| n.get());

        // Potential tuning oppertunity:
        // Lower values make the BloomFilter more effective.
        // Higher values allow more work be done in parellel.
        let min_len = if entities.len() > num_threads * 4 {
            entities.len().div_ceil(num_threads)
        } else {
            entities.len()
        };

        let cache = DashMap::new();
        let rules = entities
            .par_iter()
            .with_min_len(min_len)
            .map_init(BloomFilter::default, |filter, entity| {
                Self::build_inner(*entity, style, tree, filter, share_matches, &cache)
            })
            .flatten_iter()
            .collect();

        Self { rules, cache: cache.into_read_only() }
    }

    fn build_inner(
        entity: Entity,
        style: &Style,
        tree: &Tree<Entity>,
        filter: &mut BloomFilter,
        share_matches: bool,
        #[cfg(feature = "rayon")] rule_cache: &DashMap<Entity, Vec<MatchedRulesCache>>,
        #[cfg(not(feature = "rayon"))] rule_cache: &mut HashMap<Entity, Vec<MatchedRulesCache>>,
    ) -> Option<(Entity, (Entity, usize))> {
        compute_element_hash(entity, tree, style, filter);

        let parent = tree.get_layout_parent(entity).unwrap_or(Entity::root());

        let mut matched_index = None;

        if share_matches
            && !tree.is_first_child(entity)
            && !tree.is_last_child(entity)
            && let Some(cache) = rule_cache.get(&parent)
        {
            matched_index = cache.iter().position(|entry| {
                has_same_selector(style, entry.entity, entity)
                    && !has_nth_child_rule(style, &entry.rules)
            });
        }

        if matched_index.is_none() {
            let rules = compute_matched_rules(entity, style, tree, filter);
            if !rules.is_empty() {
                #[cfg(feature = "rayon")]
                {
                    let mut entry = rule_cache.entry(parent).or_default();
                    entry.value_mut().push(MatchedRulesCache { entity, rules });
                    matched_index = Some(entry.value().len() - 1);
                }
                #[cfg(not(feature = "rayon"))]
                {
                    let entry = rule_cache.entry(parent).or_default();
                    entry.push(MatchedRulesCache { entity, rules });
                    matched_index = Some(entry.len() - 1);
                }
            }
        }

        matched_index.map(|i| (entity, (parent, i)))
    }

    fn get(&self, entity: &Entity) -> Option<&[(Rule, u32)]> {
        let (parent, i) = self.rules.get(entity)?;
        let parent_cache = self.cache.get(parent)?;
        let entry = parent_cache.get(*i)?;
        if entry.rules.is_empty() { None } else { Some(&entry.rules) }
    }
}

// Determines the matching style rules for each entity marked for restyle, then links the entity to
// the corresponding style rule data.
pub(crate) fn style_system(cx: &mut Context) {
    let _scope = crate::profiling::scope("style_system");
    let mut redraw_entities = Vec::new();

    let locale = cx.with_current(Entity::root(), |cx| cx.environment().locale.get_untracked());
    if cx.style.selector_language.as_ref().map(|(language, _)| language) != Some(&locale) {
        cx.style.selector_language = Some((locale.clone(), locale.to_string()));
        cx.style.restyle.extend(LayoutTreeIterator::full(&cx.tree));
    }

    // Inline inheritance links the inline data of entities to the inline data of their parent, so
    // it only needs to rerun when the tree or which entities have inline data changes.
    let structure_key = (cx.tree.generation, crate::storage::inline_structure_generation());
    if cx.style.inline_inheritance_key != Some(structure_key) {
        inline_inheritance_system(cx, &[Entity::root()], &mut redraw_entities);
        cx.style.inline_inheritance_key = Some(structure_key);
    }

    if cx.style.restyle.is_empty() {
        for entity in redraw_entities {
            cx.needs_redraw(entity);
        }
        return;
    }

    let rules_changed = cx.style.matched_rules_generation != cx.style.rules_generation;
    if rules_changed {
        cx.style.matched_rules.clear();
        cx.style.matched_rules_generation = cx.style.rules_generation;
    }

    let root = Entity::root();
    let mut restyle = cx.style.take_restyle();
    // Ignored entities, such as bindings, are skipped by styling as they are by layout. Selector
    // matching and inheritance already look through them to layout parents and siblings.
    restyle.retain(|entity| {
        cx.entity_manager.is_alive(*entity)
            && (*entity == root || cx.tree.get_parent(*entity).is_some())
            && !cx.tree.is_ignored(*entity)
    });

    // Positional selectors (e.g. `:first-child`) of the siblings of new entities may have changed.
    if cx.style.has_positional_rules() {
        let parents = restyle
            .iter()
            .filter(|entity| !cx.style.matched_rules.contains_key(*entity))
            .filter_map(|entity| cx.tree.get_layout_parent(*entity))
            .collect::<HashSet<_>>();
        for parent in parents {
            for sibling in LayoutChildIterator::new(&cx.tree, parent) {
                restyle.extend(LayoutTreeIterator::subtree(&cx.tree, sibling));
            }
        }
    }

    // Parents before children.
    let mut entities = restyle.iter().copied().collect::<Vec<_>>();
    cx.tree.sort_by_depth(&mut entities);

    // Inherited data only needs to be propagated through the subtrees of the restyled entities,
    // unless the tree or inline data changed since the last pass.
    let full_walk = rules_changed || cx.style.style_structure_key != Some(structure_key);
    let mut walk_roots =
        if full_walk { vec![root] } else { inheritance_roots(&cx.tree, &entities, &restyle) };

    let mut direction_bloom = BloomFilter::default();
    let mut direction_states = Vec::new();
    loop {
        let mut state = entities
            .iter()
            .map(|entity| (*entity, cx.style.direction.get(*entity).copied()))
            .collect::<Vec<_>>();
        state.sort_unstable_by_key(|(entity, _)| entity.index());
        // A :dir() selector can change direction itself. Stop on a repeated state, keeping
        // this resolved direction snapshot for the final matching pass rather than oscillating.
        if direction_states.contains(&state) {
            log::warn!("Cyclic direction selectors detected; keeping the last resolved directions");
            break;
        }
        direction_states.push(state);

        let mut direction_changed = HashSet::new();
        for entity in entities.iter().copied() {
            compute_element_hash(entity, &cx.tree, &cx.style, &mut direction_bloom);
            let matched_direction_rules =
                compute_matched_direction_rules(entity, &cx.style, &cx.tree, &direction_bloom);
            let previous = cx.style.direction.get(entity).copied();

            if cx.style.direction.link(entity, &matched_direction_rules) {
                if previous != cx.style.direction.get(entity).copied() {
                    direction_changed.insert(entity);
                }
                cx.style.needs_relayout(entity);
                redraw_entities.push(entity);

                let iter = LayoutParentIterator::new(&cx.tree, entity);
                for parent in iter {
                    if cx.style.display.get(parent).copied().unwrap_or_default() != Display::None {
                        cx.style.needs_text_update(parent);
                        break;
                    }
                }
            }
        }

        for entity in subtrees(&cx.tree, &walk_roots) {
            if let Some(parent) = cx.tree.get_layout_parent(entity) {
                let previous = cx.style.direction.get(entity).copied();
                if cx.style.direction.inherit_shared(entity, parent) {
                    if previous != cx.style.direction.get(entity).copied() {
                        direction_changed.insert(entity);
                    }
                    cx.style.needs_text_update(entity);
                    cx.style.needs_relayout(entity);
                }
            }
        }

        if direction_changed.is_empty() {
            break;
        }
        for entity in direction_changed {
            cx.style.needs_restyle_with_dependents(&cx.tree, entity);
        }
        restyle.extend(cx.style.take_restyle());
        entities = restyle.iter().copied().collect();
        cx.tree.sort_by_depth(&mut entities);
        if !full_walk {
            walk_roots = inheritance_roots(&cx.tree, &entities, &restyle);
        }
    }

    // Direction is now resolved; linking the remaining properties must not change the
    // direction snapshot against which their :dir() selectors were matched.
    let share_matches = cx.style.can_share_selector_list_matches();
    let matched_rules = {
        #[cfg(feature = "rayon")]
        {
            MatchedRules::build_parallel(&entities, &cx.style, &cx.tree, share_matches)
        }
        #[cfg(not(feature = "rayon"))]
        {
            MatchedRules::build(&entities, &cx.style, &cx.tree, share_matches)
        }
    };

    // Style data only needs to be relinked for entities whose matched rules changed. Linking also
    // resolves variables, so the descendants of an entity whose variable-defining rules changed
    // must be relinked too.
    let mut to_link = Vec::new();
    let mut force = HashSet::new();
    let mut new_rules = Vec::with_capacity(entities.len());
    for entity in entities.iter().copied() {
        let rules = matched_rules.get(&entity).map(<[_]>::to_vec).unwrap_or_default();
        let previous = cx.style.matched_rules.get(&entity);
        let changed = previous != Some(&rules);
        if changed {
            let defines_variables = |rules: &[(Rule, u32)]| {
                rules.iter().any(|(rule, _)| cx.style.variable_rules.contains(rule))
            };
            if defines_variables(&rules) || previous.is_some_and(|p| defines_variables(p)) {
                force.extend(TreeIterator::subtree(&cx.tree, entity).skip(1));
            }
        }

        // Entities which have never matched any rules have no rule data to unlink.
        let had_rules = previous.is_some_and(|previous| !previous.is_empty());
        if (full_walk || changed) && (!rules.is_empty() || had_rules) {
            to_link.push(entity);
        }

        new_rules.push((entity, rules));
    }

    for (entity, rules) in new_rules {
        cx.style.matched_rules.insert(entity, rules);
    }

    if !force.is_empty() {
        force.retain(|entity| {
            cx.style.matched_rules.get(entity).is_some_and(|rules| !rules.is_empty())
        });
        force.extend(to_link.drain(..));
        to_link = force.into_iter().collect();
        cx.tree.sort_by_depth(&mut to_link);
    }

    let matched_rules = std::mem::take(&mut cx.style.matched_rules);

    for entity in to_link.iter() {
        if let Some(rules) = matched_rules.get(entity) {
            link_variable_data(&mut cx.style, *entity, &mut redraw_entities, rules);
        }
    }

    inherit_variable_data(&mut cx.style, &cx.tree, &walk_roots, &mut redraw_entities);

    for entity in to_link.iter().copied() {
        if let Some(rules) = matched_rules.get(&entity) {
            link_style_data(
                &mut cx.style,
                &mut cx.cache,
                &cx.tree,
                entity,
                &mut redraw_entities,
                rules,
            );
        }
    }

    cx.style.matched_rules = matched_rules;

    shared_inheritance_system(cx, &walk_roots, &mut redraw_entities);

    // Linking can replace inline data inherited from the parent, so relink it.
    inline_inheritance_system(cx, &walk_roots, &mut redraw_entities);

    cx.style.style_structure_key = Some(structure_key);

    for entity in redraw_entities {
        cx.needs_redraw(entity);
    }

    invalidate_focus_on_hidden(cx);
}
/// Re-validates `cx.focused` after style resolution.
///
/// `WindowEvent::KeyDown` dispatch in `event_manager` routes to
/// `cx.focused` regardless of the focused view's visibility. Without
/// re-validation, an entity that becomes `Display::None` (or whose
/// ancestor does) via either a CSS rule or the `.display(...)` modifier
/// keeps focus indefinitely; the existing cleanup in `Context::remove`
/// only fires when an entity is *removed from the tree*, not when it
/// merely becomes non-visible. The most user-visible symptom is a
/// plug-in `Textbox` inside a modal that gets hidden via
/// `Display::None` — every subsequent host keystroke continues to be
/// routed to the invisible textbox.
///
/// Runs at the end of [`style_system`]: at this point `display` and
/// `disabled` are authoritative for the frame, but layout / draw
/// haven't consumed them yet, so re-routing here avoids one frame of
/// incorrect dispatch.
///
/// When the focused entity (or any ancestor) is `Display::None`, or
/// the focused entity is `disabled`, focus moves to the previous
/// focusable element in tab order via [`focus_backward`]. If
/// `focus_backward` returns `None` (no prior focusable in scope),
/// focus falls back to [`Entity::root`] so keystrokes route to the
/// window rather than to an invisible view.
fn invalidate_focus_on_hidden(cx: &mut Context) {
    use crate::style::Display;
    use crate::tree::focus_backward;

    let focused = cx.focused;
    if focused == Entity::null() || focused == Entity::root() {
        return;
    }

    let hidden = std::iter::once(focused)
        .chain(focused.parent_iter(&cx.tree))
        .any(|entity| cx.style.display.get(entity).copied().unwrap_or_default() == Display::None);
    let disabled = cx.style.disabled.get(focused).cloned().unwrap_or_default();

    if !hidden && !disabled {
        return;
    }

    // `lock_focus_to` matches the scope used by `focus_prev` / `focus_next`
    // for keyboard navigation: respect any active modal-focus lock from
    // `lock_focus_to_within`, otherwise scope to the entire tree.
    let lock_focus_to = cx.focus_stack.last().copied().unwrap_or(Entity::root());
    let new_target =
        focus_backward(&cx.tree, &cx.style, focused, lock_focus_to).unwrap_or(Entity::root());

    cx.with_current(new_target, |cx| cx.focus());
}

#[cfg(test)]
mod tests {
    use super::{matches_language_range, sort_matched_rules, style_system};
    use crate::prelude::*;
    use crate::style::Rule;
    use vizia_id::GenerationalId;

    #[test]
    fn sort_matched_rules_uses_rule_order_for_specificity_ties() {
        let mut matched_rules = vec![
            (Rule::new(1, 0), 10),
            (Rule::new(3, 0), 10),
            (Rule::new(2, 0), 11),
            (Rule::new(4, 0), 10),
        ];

        sort_matched_rules(&mut matched_rules);

        assert_eq!(
            matched_rules,
            vec![
                (Rule::new(2, 0), 11),
                (Rule::new(4, 0), 10),
                (Rule::new(3, 0), 10),
                (Rule::new(1, 0), 10),
            ]
        );
    }

    struct Unnamed;

    impl View for Unnamed {}

    #[test]
    fn is_and_where_match_with_standard_specificity() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".item { width: 10px; height: 10px; top: 1px; bottom: 1px; left: 1px; }
             :is(.item, #absent) { width: 20px; }
             .item { width: 30px; }
             :where(#absent, .item) { height: 30px; }
             :is(.item, :made-up) { top: 20px; }
             :where(:made-up, .item) { bottom: 20px; }
             :where(:made-up, .item) { right: 20px; }
             :is(:made-up) { left: 99px; }
             :is(.item, #absent:made-up) { left: 42px; }
             .item { left: 1px; }",
        )
        .unwrap();
        let item = Element::new(&mut cx).class("item").entity();
        style_system(&mut cx);

        assert_eq!(cx.style.width.get(item), Some(&Units::Pixels(20.0)));
        assert_eq!(cx.style.height.get(item), Some(&Units::Pixels(10.0)));
        assert_eq!(cx.style.top.get(item), Some(&Units::Pixels(20.0)));
        assert_eq!(cx.style.bottom.get(item), Some(&Units::Pixels(1.0)));
        assert_eq!(cx.style.right.get(item), Some(&Units::Pixels(20.0)));
        assert_eq!(cx.style.left.get(item), Some(&Units::Pixels(1.0)));
    }

    #[test]
    fn is_and_where_nested_positional_selectors_distinguish_siblings() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".item { width: 10px; }
             .item:is(:nth-child(2)) { width: 20px; }
             .item:where(:nth-child(3)) { width: 30px; }",
        )
        .unwrap();
        let mut items = Vec::new();
        VStack::new(&mut cx, |cx| {
            for _ in 0..5 {
                items.push(Element::new(cx).class("item").entity());
            }
        });
        style_system(&mut cx);
        for (entity, width) in items.into_iter().zip([10.0, 20.0, 30.0, 10.0, 10.0]) {
            assert_eq!(cx.style.width.get(entity), Some(&Units::Pixels(width)));
        }
    }

    #[test]
    fn is_nested_has_distinguishes_siblings_with_different_children() {
        let mut cx = Context::default();
        cx.add_stylesheet(".item { width: 10px; } .item:is(:has(> .trigger)) { width: 20px; }")
            .unwrap();
        let mut items = Vec::new();
        VStack::new(&mut cx, |cx| {
            for index in 0..5 {
                items.push(
                    VStack::new(cx, |cx| {
                        if index == 1 {
                            Element::new(cx).class("trigger");
                        }
                    })
                    .class("item")
                    .entity(),
                );
            }
        });
        style_system(&mut cx);
        for (entity, width) in items.into_iter().zip([10.0, 20.0, 10.0, 10.0, 10.0]) {
            assert_eq!(cx.style.width.get(entity), Some(&Units::Pixels(width)));
        }
    }

    #[test]
    fn is_nested_sibling_selector_distinguishes_siblings() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".item { width: 10px; }
             :is(.trigger + .item, .other) { width: 20px; }",
        )
        .unwrap();
        let mut items = Vec::new();
        VStack::new(&mut cx, |cx| {
            for index in 0..5 {
                let item = Element::new(cx).class("item");
                let item = if index == 0 { item.class("trigger") } else { item };
                items.push(item.entity());
            }
        });
        style_system(&mut cx);
        for (entity, width) in items.into_iter().zip([10.0, 20.0, 10.0, 10.0, 10.0]) {
            assert_eq!(cx.style.width.get(entity), Some(&Units::Pixels(width)));
        }
    }

    #[test]
    fn is_and_where_distinguish_sibling_direction_and_disabled_state() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".item { width: 10px; height: 10px; }
             .item:is(:dir(rtl)) { width: 20px; }
             .item:where(:disabled) { height: 20px; }",
        )
        .unwrap();
        let mut items = Vec::new();
        VStack::new(&mut cx, |cx| {
            for index in 0..5 {
                let item = Element::new(cx).class("item");
                let item = if index == 1 { item.direction(Direction::RightToLeft) } else { item };
                let item = if index == 2 { item.disabled(true) } else { item };
                items.push(item.entity());
            }
        });
        style_system(&mut cx);
        for (entity, (width, height)) in items.into_iter().zip([
            (10.0, 10.0),
            (20.0, 10.0),
            (10.0, 20.0),
            (10.0, 10.0),
            (10.0, 10.0),
        ]) {
            assert_eq!(cx.style.width.get(entity), Some(&Units::Pixels(width)));
            assert_eq!(cx.style.height.get(entity), Some(&Units::Pixels(height)));
        }
    }

    #[test]
    fn is_direction_auto_distinguishes_sibling_text() {
        let mut cx = Context::default();
        cx.add_stylesheet(".item { width: 10px; } .item:is(:dir(rtl)) { width: 20px; }").unwrap();
        let mut items = Vec::new();
        VStack::new(&mut cx, |cx| {
            for text in ["left", "\u{05d0}", "left", "\u{05d0}", "left"] {
                items.push(Label::new(cx, text).class("item").direction(Direction::Auto).entity());
            }
        });
        style_system(&mut cx);
        for (entity, width) in items.into_iter().zip([10.0, 20.0, 10.0, 20.0, 10.0]) {
            assert_eq!(cx.style.width.get(entity), Some(&Units::Pixels(width)));
        }
    }

    #[test]
    fn is_direction_auto_restyles_after_text_changes() {
        let mut cx = Context::default();
        cx.add_stylesheet(".item { width: 10px; } .item:is(:dir(rtl)) { width: 20px; }").unwrap();
        let text = Signal::new("left".to_string());
        let item = Label::new(&mut cx, text).class("item").direction(Direction::Auto).entity();
        crate::systems::binding_system(&mut cx);
        style_system(&mut cx);
        assert_eq!(cx.style.width.get(item), Some(&Units::Pixels(10.0)));

        text.set("\u{05d0}".to_string());
        crate::systems::binding_system(&mut cx);
        style_system(&mut cx);
        assert_eq!(cx.style.width.get(item), Some(&Units::Pixels(20.0)));

        text.set("left".to_string());
        crate::systems::binding_system(&mut cx);
        style_system(&mut cx);
        assert_eq!(cx.style.width.get(item), Some(&Units::Pixels(10.0)));
    }

    #[test]
    fn is_direction_auto_restyles_dependents_after_set_text() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".hit { width: 10px; } .item:is(:dir(rtl)) + .hit { width: 20px; }
             .parent { opacity: 1; } .parent:has(> .item:where(:dir(rtl))) { opacity: 0.5; }",
        )
        .unwrap();
        let mut item = Entity::null();
        let mut hit = Entity::null();
        let parent = VStack::new(&mut cx, |cx| {
            item = Label::new(cx, "left").class("item").direction(Direction::Auto).entity();
            hit = Element::new(cx).class("hit").entity();
        })
        .class("parent")
        .entity();
        style_system(&mut cx);
        for (text, width, opacity) in [("\u{05d0}", 20.0, 0.5), ("left", 10.0, 1.0)] {
            EventContext::new_with_current(&mut cx, item).set_text(text);
            style_system(&mut cx);
            assert_eq!(cx.style.width.get(hit), Some(&Units::Pixels(width)));
            assert_eq!(cx.style.opacity.get(parent), Some(&Opacity(opacity)));
        }
    }

    #[test]
    fn is_and_where_restyle_after_nested_selector_changes() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".hit { width: 10px; height: 10px; top: 10px; }
             :is(.panel.active > .hit, .other) { width: 20px; }
             :where(.panel:hover) .hit { height: 20px; }
             :is(.trigger + .hit, .other) { top: 20px; }
             .panel { opacity: 1; }
             .panel:has(> :is(.switch, .other)) { opacity: 0.5; }",
        )
        .unwrap();
        let mut trigger = Entity::null();
        let mut hit = Entity::null();
        let panel = VStack::new(&mut cx, |cx| {
            trigger = Element::new(cx).entity();
            hit = Element::new(cx).class("hit").entity();
        })
        .class("panel")
        .entity();
        style_system(&mut cx);

        for (enabled, value) in [(true, 20.0), (false, 10.0)] {
            cx.style.set_class(&cx.tree, panel, "active", enabled);
            style_system(&mut cx);
            assert_eq!(cx.style.width.get(hit), Some(&Units::Pixels(value)));
        }
        for (enabled, value) in [(true, 20.0), (false, 10.0)] {
            cx.set_pseudo_class(panel, PseudoClassFlags::HOVER, enabled);
            style_system(&mut cx);
            assert_eq!(cx.style.height.get(hit), Some(&Units::Pixels(value)));
        }
        for (enabled, value) in [(true, 20.0), (false, 10.0)] {
            cx.style.set_class(&cx.tree, trigger, "trigger", enabled);
            style_system(&mut cx);
            assert_eq!(cx.style.top.get(hit), Some(&Units::Pixels(value)));
        }
        for (enabled, opacity) in [(true, 0.5), (false, 1.0)] {
            cx.style.set_class(&cx.tree, trigger, "switch", enabled);
            style_system(&mut cx);
            assert_eq!(cx.style.opacity.get(panel), Some(&Opacity(opacity)));
        }
    }

    #[test]
    fn language_ranges_match_locale_subtags() {
        for (locale, range, expected) in [
            ("en-US", "en", true),
            ("en-US", "EN-us", true),
            ("en-US", "e", false),
            ("en-US", "en-u", false),
            ("en-US", "en-US-GB", false),
            ("en-US", "fr", false),
            ("zh-Hant-TW", "zh-TW", true),
            ("zh-Hant-TW", "zh-*-TW", true),
            ("en", "en-*", true),
            ("en-US", "*", true),
            ("de-u-co-phonebk", "de-phonebk", false),
            ("en-US", "", false),
            ("en-US", "en--US", false),
            ("en-US", "en-**", false),
            ("en-US", "englishxx", false),
        ] {
            assert_eq!(matches_language_range(locale, range), expected, "{locale}, {range}");
        }
    }

    #[test]
    fn language_selectors_follow_application_locale_changes() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            r#"
            .probe { width: 10px; height: 10px; top: 1px; right: 1px; }
            .probe:lang(en) { width: 20px; }
            .probe:lang(en-US) { height: 30px; }
            .probe:lang(fr, de) { width: 40px; }
            .probe:lang(zh-TW) { width: 60px; }
            .probe:not(:lang(en)) { height: 50px; }
            .probe:lang("en-*") { top: 3px; }
            .probe:lang(fr) { direction: rtl; }
            .probe:dir(rtl) { right: 17px; }
            .ancestor { opacity: 1; }
            .ancestor:has(> .probe:lang(en)) { opacity: 0.5; }
            "#,
        )
        .unwrap();
        let mut probe = Entity::null();
        let ancestor = VStack::new(&mut cx, |cx| {
            probe = Element::new(cx).class("probe").entity();
        })
        .class("ancestor")
        .entity();

        for (locale, width, height, top, right, opacity) in [
            ("en-US", 20.0, 30.0, 3.0, 1.0, 0.5),
            ("en-GB", 20.0, 10.0, 3.0, 1.0, 0.5),
            ("fr-CA", 40.0, 50.0, 1.0, 17.0, 1.0),
            ("zh-Hant-TW", 60.0, 50.0, 1.0, 1.0, 1.0),
            ("en-US", 20.0, 30.0, 3.0, 1.0, 0.5),
        ] {
            cx.environment().locale.set(locale.parse().unwrap());
            style_system(&mut cx);
            assert_eq!(cx.style.width.get(probe), Some(&Units::Pixels(width)), "{locale}");
            assert_eq!(cx.style.height.get(probe), Some(&Units::Pixels(height)), "{locale}");
            assert_eq!(cx.style.top.get(probe), Some(&Units::Pixels(top)), "{locale}");
            assert_eq!(cx.style.right.get(probe), Some(&Units::Pixels(right)), "{locale}");
            assert_eq!(cx.style.opacity.get(ancestor), Some(&Opacity(opacity)), "{locale}");
        }

        EventContext::new_with_current(&mut cx, Entity::root())
            .set_language("fr-CA".parse().unwrap());
        style_system(&mut cx);
        assert_eq!(cx.style.width.get(probe), Some(&Units::Pixels(40.0)));
        assert_eq!(cx.style.right.get(probe), Some(&Units::Pixels(17.0)));
    }

    #[test]
    fn unknown_pseudo_classes_do_not_apply_invalid_rules() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".probe { width: 10px; }
             .probe:made-up { width: 80px; }
             .probe, .other:made-up { width: 90px; }
             .probe:not(:made-up) { width: 100px; }
             .probe:has(> :made-up) { width: 110px; }
             .probe { height: 20px; }",
        )
        .unwrap();
        let probe = Element::new(&mut cx).class("probe").entity();
        style_system(&mut cx);

        assert_eq!(cx.style.width.get(probe), Some(&Units::Pixels(10.0)));
        assert_eq!(cx.style.height.get(probe), Some(&Units::Pixels(20.0)));
    }

    #[test]
    fn custom_pseudo_class_does_not_match() {
        use super::{MatchingContext, MatchingMode, Node, PseudoClass, QuirksMode};
        use vizia_style::selectors::context::{
            MatchingForInvalidation, NeedsSelectorFlags, SelectorCaches,
        };

        let cx = Context::default();
        let node = Node { entity: Entity::root(), store: &cx.style, tree: &cx.tree };
        let mut cache = SelectorCaches::default();
        let mut context = MatchingContext::new(
            MatchingMode::Normal,
            None,
            &mut cache,
            QuirksMode::NoQuirks,
            NeedsSelectorFlags::Yes,
            MatchingForInvalidation::No,
        );
        assert!(!vizia_style::Element::match_non_ts_pseudo_class(
            &node,
            &PseudoClass::Custom("made-up".into()),
            &mut context,
        ));
    }

    #[test]
    fn unitless_zero_positions_apply_without_accepting_nonzero_numbers() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".probe { background-position: 0 0, right 0; transform-origin: 0 0; }
             .invalid { background-position: left; background-position: 0 1;
                        transform-origin: right; transform-origin: 0 1; }",
        )
        .unwrap();
        let probe = Element::new(&mut cx).class("probe").entity();
        let invalid = Element::new(&mut cx).class("invalid").entity();
        style_system(&mut cx);

        let positions = cx.style.background_position.get(probe).unwrap();
        assert_eq!(positions.len(), 2);
        assert_eq!(positions[0].x.to_length_or_percentage(), LengthOrPercentage::px(0.0));
        assert_eq!(positions[0].y.to_length_or_percentage(), LengthOrPercentage::px(0.0));
        assert_eq!(positions[1].x.to_length_or_percentage(), LengthOrPercentage::Percentage(100.0));
        assert_eq!(positions[1].y.to_length_or_percentage(), LengthOrPercentage::px(0.0));
        let origin = cx.style.transform_origin.get(probe).unwrap();
        assert_eq!(origin.x, LengthOrPercentage::px(0.0));
        assert_eq!(origin.y, LengthOrPercentage::px(0.0));

        let position = &cx.style.background_position.get(invalid).unwrap()[0];
        assert_eq!(position.x.to_length_or_percentage(), LengthOrPercentage::px(0.0));
        assert_eq!(position.y.to_length_or_percentage(), LengthOrPercentage::Percentage(50.0));
        let origin = cx.style.transform_origin.get(invalid).unwrap();
        assert_eq!(origin.x, LengthOrPercentage::Percentage(100.0));
        assert_eq!(origin.y, LengthOrPercentage::Percentage(50.0));
    }

    #[test]
    fn single_value_positions_apply_to_background_and_transform_origin() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".probe { background-position: left, bottom, 10px; transform-origin: top; }",
        )
        .unwrap();
        let entity = Element::new(&mut cx).class("probe").entity();
        style_system(&mut cx);

        let positions = cx.style.background_position.get(entity).unwrap();
        let expected = [
            (LengthOrPercentage::px(0.0), LengthOrPercentage::Percentage(50.0)),
            (LengthOrPercentage::Percentage(50.0), LengthOrPercentage::Percentage(100.0)),
            (LengthOrPercentage::px(10.0), LengthOrPercentage::Percentage(50.0)),
        ];
        assert_eq!(positions.len(), expected.len());
        for (position, (x, y)) in positions.iter().zip(expected) {
            assert_eq!(position.x.to_length_or_percentage(), x);
            assert_eq!(position.y.to_length_or_percentage(), y);
        }

        let origin = cx.style.transform_origin.get(entity).unwrap();
        assert_eq!(origin.x, LengthOrPercentage::Percentage(50.0));
        assert_eq!(origin.y, LengthOrPercentage::px(0.0));
    }

    #[test]
    fn border_width_unitless_zero_applies() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".widths { border-width: 0 2px; border-right-width: -0; border-left: 0; }
             .mixed { border-width: 0 2px; }
             .shorthand { border: 0 solid red; outline: 0; }
             .outline-width { outline-width: 0; }
             .variable { --zero: 0; border-width: var(--zero); border-left-width: var(--missing, 0); }
             .invalid { border: 5px solid red; border: 1 solid red; }
             .invalid-mixed { border-width: 5px; border-width: 0 2px 1; }
             .invalid-side { border-top-width: 5px; border-top-width: 0 1px; }",
        )
        .unwrap();
        let widths = Element::new(&mut cx).class("widths").entity();
        let mixed = Element::new(&mut cx).class("mixed").entity();
        let shorthand = Element::new(&mut cx).class("shorthand").entity();
        let outline_width = Element::new(&mut cx).class("outline-width").entity();
        let variable = Element::new(&mut cx).class("variable").entity();
        let invalid = Element::new(&mut cx).class("invalid").entity();
        let invalid_mixed = Element::new(&mut cx).class("invalid-mixed").entity();
        let invalid_side = Element::new(&mut cx).class("invalid-side").entity();
        style_system(&mut cx);

        let resolved =
            |store: &crate::storage::animatable_var_set::AnimatableVarSet<LengthOrPercentage>,
             entity| store.get_resolved(entity, &cx.style.custom_length_props);
        let zero = Some(LengthOrPercentage::px(0.0));
        for entity in [widths, shorthand, variable] {
            for store in [
                &cx.style.border_top_width,
                &cx.style.border_right_width,
                &cx.style.border_bottom_width,
                &cx.style.border_left_width,
            ] {
                assert_eq!(resolved(store, entity), zero);
            }
        }
        assert_eq!(resolved(&cx.style.outline_width, shorthand), zero);
        assert_eq!(resolved(&cx.style.outline_width, outline_width), zero);
        assert_eq!(resolved(&cx.style.border_top_width, mixed), zero);
        assert_eq!(
            resolved(&cx.style.border_right_width, mixed),
            Some(LengthOrPercentage::px(2.0))
        );
        assert_eq!(resolved(&cx.style.border_left_width, mixed), Some(LengthOrPercentage::px(2.0)));
        assert_eq!(
            resolved(&cx.style.border_top_width, invalid),
            Some(LengthOrPercentage::px(5.0))
        );
        assert_eq!(
            resolved(&cx.style.border_top_width, invalid_mixed),
            Some(LengthOrPercentage::px(5.0))
        );
        assert_eq!(
            resolved(&cx.style.border_top_width, invalid_side),
            Some(LengthOrPercentage::px(5.0))
        );
    }

    #[test]
    fn border_width_unitless_zero_works_in_keyframes() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            "@keyframes borders { from { border-top-width: 0; } to { border-top-width: 5px; } }",
        )
        .unwrap();
        let animation = *cx.style.get_animation("borders").unwrap();
        let frames = &cx.style.border_top_width.get_animation_mut(animation).unwrap().keyframes;
        assert_eq!(
            frames.iter().map(|frame| frame.value.clone()).collect::<Vec<_>>(),
            vec![LengthOrPercentage::px(0.0), LengthOrPercentage::px(5.0)]
        );
    }

    #[test]
    fn border_radius_unitless_zero_applies_and_resolves_variables() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".direct { border-radius: 0 8px; border-bottom-left-radius: 0; }
             .variable { --zero: 0; corner-radius: var(--zero); }
             .fallback { border-radius: var(--missing, 0); border-top-right-radius: var(--missing, 0); }
             .invalid { border-radius: 6px; border-radius: 1; }
             .mixed { border-radius: 6px; border-radius: var(--missing, 0) 8px; }
             .bad-fallback { border-top-left-radius: var(--missing, 0 8px); }
             .nonzero { --value: 1; border-top-left-radius: var(--value, 7px); }",
        )
        .unwrap();
        let direct = Element::new(&mut cx).class("direct").entity();
        let variable = Element::new(&mut cx).class("variable").entity();
        let fallback = Element::new(&mut cx).class("fallback").entity();
        let invalid = Element::new(&mut cx).class("invalid").entity();
        let mixed = Element::new(&mut cx).class("mixed").entity();
        let bad_fallback = Element::new(&mut cx).class("bad-fallback").entity();
        let nonzero = Element::new(&mut cx).class("nonzero").entity();
        style_system(&mut cx);

        let resolved =
            |store: &crate::storage::animatable_var_set::AnimatableVarSet<LengthOrPercentage>,
             entity| store.get_resolved(entity, &cx.style.custom_length_props);
        assert_eq!(
            resolved(&cx.style.corner_top_left_radius, direct),
            Some(LengthOrPercentage::px(0.0))
        );
        assert_eq!(
            resolved(&cx.style.corner_top_right_radius, direct),
            Some(LengthOrPercentage::px(8.0))
        );
        assert_eq!(
            resolved(&cx.style.corner_bottom_right_radius, direct),
            Some(LengthOrPercentage::px(0.0))
        );
        assert_eq!(
            resolved(&cx.style.corner_bottom_left_radius, direct),
            Some(LengthOrPercentage::px(0.0))
        );
        for entity in [variable, fallback] {
            assert_eq!(
                resolved(&cx.style.corner_top_left_radius, entity),
                Some(LengthOrPercentage::px(0.0))
            );
            assert_eq!(
                resolved(&cx.style.corner_top_right_radius, entity),
                Some(LengthOrPercentage::px(0.0))
            );
        }
        assert_eq!(
            resolved(&cx.style.corner_top_left_radius, invalid),
            Some(LengthOrPercentage::px(6.0))
        );
        assert_eq!(
            resolved(&cx.style.corner_top_left_radius, mixed),
            Some(LengthOrPercentage::px(6.0))
        );
        assert_eq!(resolved(&cx.style.corner_top_left_radius, bad_fallback), None);
        assert_eq!(
            resolved(&cx.style.corner_top_left_radius, nonzero),
            Some(LengthOrPercentage::px(7.0))
        );
    }

    #[test]
    fn border_radius_unitless_zero_works_in_keyframes() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            "@keyframes rounded { from { border-top-left-radius: 0; } to { border-top-left-radius: 6px; } }",
        )
        .unwrap();
        let animation = *cx.style.get_animation("rounded").unwrap();
        let frames =
            &cx.style.corner_top_left_radius.get_animation_mut(animation).unwrap().keyframes;
        assert_eq!(
            frames.iter().map(|frame| frame.value.clone()).collect::<Vec<_>>(),
            vec![LengthOrPercentage::px(0.0), LengthOrPercentage::px(6.0)]
        );
    }

    #[test]
    fn background_repeat_axis_pairs_apply_to_stylesheets() {
        use vizia_style::BackgroundRepeat;

        let mut cx = Context::default();
        cx.add_stylesheet(
            ".pairs { background-repeat: repeat; background-repeat: repeat no-repeat, no-repeat repeat; }
             .legacy { background-repeat: repeat-x, repeat-y; }
             .invalid { background-repeat: no-repeat; background-repeat: repeat-x repeat; }",
        )
        .unwrap();
        let pairs = Element::new(&mut cx).class("pairs").entity();
        let legacy = Element::new(&mut cx).class("legacy").entity();
        let invalid = Element::new(&mut cx).class("invalid").entity();
        style_system(&mut cx);

        let expected = vec![BackgroundRepeat::RepeatX, BackgroundRepeat::RepeatY];
        assert_eq!(cx.style.background_repeat.get(pairs), Some(&expected));
        assert_eq!(cx.style.background_repeat.get(legacy), Some(&expected));
        assert_eq!(
            cx.style.background_repeat.get(invalid),
            Some(&vec![BackgroundRepeat::NoRepeat])
        );
    }

    #[test]
    fn background_size_unitless_zero_applies_in_longhand_and_shorthand() {
        use crate::style::ImageOrGradient;
        use vizia_style::LengthPercentageOrAuto;

        let mut cx = Context::default();
        cx.add_stylesheet(
            ".longhand { background-size: 0 0, auto -0, 0; }
             .shorthand { background: url(image.png) center / 0 0 no-repeat; }
             .invalid { background-size: cover; background-size: 0 1; }
             .invalid-shorthand { background-size: contain; background: center / 0 1; }",
        )
        .unwrap();
        let longhand = Element::new(&mut cx).class("longhand").entity();
        let shorthand = Element::new(&mut cx).class("shorthand").entity();
        let invalid = Element::new(&mut cx).class("invalid").entity();
        let invalid_shorthand = Element::new(&mut cx).class("invalid-shorthand").entity();
        style_system(&mut cx);

        let zero = LengthPercentageOrAuto::px(0.0);
        let auto = LengthPercentageOrAuto::Auto;
        let size = |width, height| BackgroundSize::Explicit { width, height };
        assert_eq!(
            cx.style.background_size.get(longhand),
            Some(&vec![
                size(zero.clone(), zero.clone()),
                size(auto.clone(), zero.clone()),
                size(zero.clone(), auto),
            ])
        );
        assert_eq!(cx.style.background_size.get(shorthand), Some(&vec![size(zero.clone(), zero)]));
        assert_eq!(
            cx.style.background_image.get(shorthand),
            Some(&vec![ImageOrGradient::Image("image.png".into())])
        );
        assert_eq!(cx.style.background_size.get(invalid), Some(&vec![BackgroundSize::Cover]));
        assert_eq!(
            cx.style.background_size.get(invalid_shorthand),
            Some(&vec![BackgroundSize::Contain])
        );
    }

    #[test]
    fn background_size_unitless_zero_works_in_keyframes() {
        use vizia_style::LengthPercentageOrAuto;

        let mut cx = Context::default();
        cx.add_stylesheet(
            "@keyframes background-sizing {
                 from { background: center / 0; }
                 to { background-size: 0 0; }
             }",
        )
        .unwrap();
        let animation = *cx.style.get_animation("background-sizing").unwrap();
        let frames = &cx.style.background_size.get_animation_mut(animation).unwrap().keyframes;
        let zero = LengthPercentageOrAuto::px(0.0);
        assert_eq!(
            frames.iter().map(|frame| frame.value.clone()).collect::<Vec<_>>(),
            vec![
                vec![BackgroundSize::Explicit {
                    width: zero.clone(),
                    height: LengthPercentageOrAuto::Auto,
                }],
                vec![BackgroundSize::Explicit { width: zero.clone(), height: zero }],
            ]
        );
    }

    #[test]
    fn background_image_keyframes_preserve_empty_layers() {
        use crate::style::ImageOrGradient;

        let mut cx = Context::default();
        cx.add_stylesheet(
            "@keyframes layers {
                 from { background-image: none, url(image.png); }
                 to { background-image: url(image.png), none; }
             }",
        )
        .unwrap();
        let animation = *cx.style.get_animation("layers").unwrap();
        let frames = &cx.style.background_image.get_animation_mut(animation).unwrap().keyframes;
        let empty = ImageOrGradient::Gradient(Gradient::None);
        let image = ImageOrGradient::Image("image.png".into());
        assert_eq!(
            frames.iter().map(|frame| frame.value.clone()).collect::<Vec<_>>(),
            vec![vec![empty.clone(), image.clone()], vec![image, empty]]
        );
    }

    #[test]
    fn rgba_alpha_rounding_applies_to_styles_and_variables() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".direct { background-color: rgba(12, 34, 56, 0.3); color: rgb(12 34 56 / 30%); }
             .variable { --shade: rgba(12, 34, 56, 0.3); background-color: var(--shade); }
             .fallback { background-color: var(--missing, rgba(12, 34, 56, 0.3)); }",
        )
        .unwrap();
        let direct = Element::new(&mut cx).class("direct").entity();
        let variable = Element::new(&mut cx).class("variable").entity();
        let fallback = Element::new(&mut cx).class("fallback").entity();
        style_system(&mut cx);

        let expected = Some(Color::rgba(12, 34, 56, 77));
        for entity in [direct, variable, fallback] {
            assert_eq!(
                cx.style.background_color.get_resolved(entity, &cx.style.custom_color_props),
                expected
            );
        }
        assert_eq!(
            cx.style.font_color.get_resolved(direct, &cx.style.custom_color_props),
            expected
        );
    }

    #[test]
    fn background_shorthand_resets_and_obeys_longhand_cascade() {
        use crate::style::ImageOrGradient;

        let mut cx = Context::default();
        cx.add_stylesheet(
            ".base {
                 background-color: green; background-image: url(old.png);
                 background-position: center; background-size: cover; background-repeat: no-repeat;
             }
             .reset { background: red; }
             .override {
                 background: url(new.png) center / cover no-repeat red;
                 background-color: blue; background-repeat: repeat-y;
             }
             .invalid { background: url(bad.png) / cover; }",
        )
        .unwrap();
        let reset = Element::new(&mut cx).class("base").class("reset").entity();
        let overridden = Element::new(&mut cx).class("base").class("override").entity();
        let invalid = Element::new(&mut cx).class("base").class("invalid").entity();
        style_system(&mut cx);

        let color =
            |entity| cx.style.background_color.get_resolved(entity, &cx.style.custom_color_props);
        assert_eq!(color(reset), Some(Color::rgb(255, 0, 0)));
        assert_eq!(cx.style.background_image.get(reset), Some(&vec![]));
        assert_eq!(cx.style.background_position.get(reset), Some(&vec![Position::default()]));
        assert_eq!(cx.style.background_size.get(reset), Some(&vec![BackgroundSize::default()]));
        assert_eq!(cx.style.background_repeat.get(reset), Some(&vec![BackgroundRepeat::Repeat]));

        assert_eq!(color(overridden), Some(Color::rgb(0, 0, 255)));
        assert_eq!(
            cx.style.background_image.get(overridden),
            Some(&vec![ImageOrGradient::Image("new.png".into())])
        );
        assert_eq!(cx.style.background_position.get(overridden), Some(&vec![Position::center()]));
        assert_eq!(cx.style.background_size.get(overridden), Some(&vec![BackgroundSize::Cover]));
        assert_eq!(
            cx.style.background_repeat.get(overridden),
            Some(&vec![BackgroundRepeat::RepeatY])
        );

        assert_eq!(color(invalid), Some(Color::rgb(0, 128, 0)));
        assert_eq!(
            cx.style.background_image.get(invalid),
            Some(&vec![ImageOrGradient::Image("old.png".into())])
        );
        assert_eq!(
            cx.style.background_repeat.get(invalid),
            Some(&vec![BackgroundRepeat::NoRepeat])
        );
    }

    #[test]
    fn background_shorthand_expands_in_keyframes() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            "@keyframes background-test {
                 from { background: red; }
                 to { background: blue center / cover no-repeat; }
             }",
        )
        .unwrap();
        let animation = *cx.style.get_animation("background-test").unwrap();
        let frames = &cx.style.background_repeat.get_animation_mut(animation).unwrap().keyframes;
        assert_eq!(
            frames.iter().map(|frame| frame.value.clone()).collect::<Vec<_>>(),
            vec![vec![BackgroundRepeat::Repeat], vec![BackgroundRepeat::NoRepeat]]
        );
        let frames = &cx.style.background_size.get_animation_mut(animation).unwrap().keyframes;
        assert_eq!(
            frames.iter().map(|frame| frame.value.clone()).collect::<Vec<_>>(),
            vec![vec![BackgroundSize::default()], vec![BackgroundSize::Cover]]
        );
    }

    #[test]
    fn text_wrap_standard_keywords_apply_like_legacy_booleans() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".standard { text-wrap: true; text-wrap: nowrap; }
             .legacy { text-wrap: false; }
             .wrapped { text-wrap: WRAP; }",
        )
        .unwrap();
        let standard = Element::new(&mut cx).class("standard").entity();
        let legacy = Element::new(&mut cx).class("legacy").entity();
        let wrapped = Element::new(&mut cx).class("wrapped").entity();
        style_system(&mut cx);

        assert_eq!(cx.style.text_wrap.get(standard), Some(&false));
        assert_eq!(cx.style.text_wrap.get(legacy), Some(&false));
        assert_eq!(cx.style.text_wrap.get(wrapped), Some(&true));
    }

    #[test]
    fn unitless_zero_applies_to_layout_longhands_and_variables() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".direct { width: 0; left: -0; padding-left: 0.0; row-gap: 0; }
             .variable {
                 --zero: 0;
                 width: var(--zero);
                 left: var(--missing, 0);
                 padding-left: var(--zero);
             }
             .invalid { width: 7px; width: 1; padding-left: 8px; padding-left: -1; }",
        )
        .unwrap();
        let direct = Element::new(&mut cx).class("direct").entity();
        let variable = Element::new(&mut cx).class("variable").entity();
        let invalid = Element::new(&mut cx).class("invalid").entity();
        style_system(&mut cx);

        let resolved = |store: &crate::storage::animatable_var_set::AnimatableVarSet<Units>,
                        entity| {
            store.get_resolved(entity, &cx.style.custom_units_props)
        };
        for entity in [direct, variable] {
            assert_eq!(resolved(&cx.style.width, entity), Some(Units::Pixels(0.0)));
            assert_eq!(resolved(&cx.style.left, entity), Some(Units::Pixels(0.0)));
            assert_eq!(resolved(&cx.style.padding_left, entity), Some(Units::Pixels(0.0)));
        }
        assert_eq!(resolved(&cx.style.vertical_gap, direct), Some(Units::Pixels(0.0)));
        assert_eq!(resolved(&cx.style.width, invalid), Some(Units::Pixels(7.0)));
        assert_eq!(resolved(&cx.style.padding_left, invalid), Some(Units::Pixels(8.0)));
    }

    #[test]
    fn letter_spacing_unitless_zero_applies_to_stylesheets_and_variables() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".direct { letter-spacing: 4px; letter-spacing: -0; }
             .variables { --zero: 0; --mixed: 0 1; }
             .variable { letter-spacing: var(--zero); }
             .fallback { letter-spacing: var(--missing, 0.0); }
             .mixed { letter-spacing: var(--mixed, 3px); }
             .invalid-fallback { letter-spacing: var(--missing, 0 1); }
             .invalid { letter-spacing: 2px; letter-spacing: 1; }",
        )
        .unwrap();
        let direct = Element::new(&mut cx).class("direct").entity();
        let invalid = Element::new(&mut cx).class("invalid").entity();
        let invalid_fallback = Element::new(&mut cx).class("invalid-fallback").entity();
        let mut children = Vec::new();
        VStack::new(&mut cx, |cx| {
            for class in ["variable", "fallback", "mixed"] {
                children.push(Element::new(cx).class(class).entity());
            }
        })
        .class("variables");
        style_system(&mut cx);

        let resolved = |entity| {
            cx.style.letter_spacing.get_resolved(entity, &cx.style.custom_letter_spacing_props)
        };
        let zero = Some(LetterSpacing::Length(Length::zero()));
        assert_eq!(resolved(direct), zero);
        assert_eq!(resolved(children[0]), zero);
        assert_eq!(resolved(children[1]), zero);
        assert_eq!(resolved(children[2]), Some(LetterSpacing::Length(Length::px(3.0))));
        assert_eq!(resolved(invalid), Some(LetterSpacing::Length(Length::px(2.0))));
        assert_eq!(resolved(invalid_fallback), None);
    }

    #[test]
    fn letter_spacing_unitless_zero_works_in_keyframes() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            "@keyframes spacing { from { letter-spacing: 0; } to { letter-spacing: 2px; } }",
        )
        .unwrap();
        let animation = *cx.style.get_animation("spacing").unwrap();
        let frames = &cx.style.letter_spacing.get_animation_mut(animation).unwrap().keyframes;
        assert_eq!(
            frames.iter().map(|frame| frame.value.clone()).collect::<Vec<_>>(),
            vec![LetterSpacing::Length(Length::zero()), LetterSpacing::Length(Length::px(2.0))]
        );
    }

    #[test]
    fn font_size_unitless_zero_applies_to_stylesheets_and_variables() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".direct { font-size: 4px; font-size: -0; }
             .variables { --zero: 0; --mixed: 0 1; }
             .variable { font-size: var(--zero); }
             .fallback { font-size: var(--missing, 0.0); }
             .mixed { font-size: var(--mixed, 3px); }
             .invalid-fallback { font-size: var(--missing, 0 1); }
             .invalid { font-size: 12px; font-size: 1; }",
        )
        .unwrap();
        let direct = Element::new(&mut cx).class("direct").entity();
        let invalid = Element::new(&mut cx).class("invalid").entity();
        let invalid_fallback = Element::new(&mut cx).class("invalid-fallback").entity();
        let mut children = Vec::new();
        VStack::new(&mut cx, |cx| {
            for class in ["variable", "fallback", "mixed"] {
                children.push(Element::new(cx).class(class).entity());
            }
        })
        .class("variables");
        let mut inherited = Entity::null();
        VStack::new(&mut cx, |cx| {
            inherited = Element::new(cx).entity();
        })
        .class("direct");
        style_system(&mut cx);

        let resolved =
            |entity| cx.style.font_size.get_resolved(entity, &cx.style.custom_font_size_props);
        let zero = Some(FontSize(Length::zero()));
        assert_eq!(resolved(direct), zero);
        assert_eq!(resolved(inherited), zero);
        assert_eq!(resolved(children[0]), zero);
        assert_eq!(resolved(children[1]), zero);
        assert_eq!(resolved(children[2]), Some(FontSize(Length::px(3.0))));
        assert_eq!(resolved(invalid), Some(FontSize(Length::px(12.0))));
        assert_eq!(resolved(invalid_fallback), None);
    }

    #[test]
    fn font_size_unitless_zero_works_in_keyframes() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            "@keyframes font-sizing { from { font-size: 0; } to { font-size: 12px; } }",
        )
        .unwrap();
        let animation = *cx.style.get_animation("font-sizing").unwrap();
        let frames = &cx.style.font_size.get_animation_mut(animation).unwrap().keyframes;
        assert_eq!(
            frames.iter().map(|frame| frame.value.clone()).collect::<Vec<_>>(),
            vec![FontSize(Length::zero()), FontSize(Length::px(12.0))]
        );
    }

    #[test]
    fn unitless_zero_works_in_layout_animations() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            "@keyframes resize { from { width: 0; } to { width: 10px; } }
             .before { width: 10px; }
             .after { width: 0; transition: width 5s; }",
        )
        .unwrap();
        let animation = *cx.style.get_animation("resize").unwrap();
        let frames = &cx.style.width.get_animation_mut(animation).unwrap().keyframes;
        assert_eq!(
            frames.iter().map(|frame| frame.value).collect::<Vec<_>>(),
            vec![Units::Pixels(0.0), Units::Pixels(10.0)]
        );

        let entity = Element::new(&mut cx).class("before").entity();
        style_system(&mut cx);
        cx.style.set_class(&cx.tree, entity, "after", true);
        style_system(&mut cx);
        let transition = cx
            .style
            .width
            .get_active_animations()
            .unwrap()
            .iter()
            .find(|animation| animation.entities.contains(&entity))
            .unwrap();
        assert_eq!(transition.keyframes.last().unwrap().value, Units::Pixels(0.0));
    }

    #[test]
    fn spacing_shorthands_apply_in_order_and_preserve_variable_resolution() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".spacing {
                padding: 1px 2px 3px 4px;
                padding-left: 9px;
                gap: 5px 6px;
                column-gap: 7px;
            }
            .reverse {
                padding-left: 9px;
                padding: 1px 2px;
                row-gap: 9px;
                gap: 5px 6px;
            }
            .variable {
                --spacing: 8px;
                padding: var(--spacing);
                padding-top: 9px;
                gap: var(--missing, 3px);
            }
            .zero {
                --spacing: 0;
                padding: var(--spacing);
                gap: var(--missing, 0);
            }
            .invalid {
                padding: 1px;
                padding: var(--missing, 4px) 2px;
                gap: 5px;
                gap: 2px 3px 4px;
            }",
        )
        .unwrap();
        let spacing = Element::new(&mut cx).class("spacing").entity();
        let reverse = Element::new(&mut cx).class("reverse").entity();
        let variable = Element::new(&mut cx).class("variable").entity();
        let zero = Element::new(&mut cx).class("zero").entity();
        let invalid = Element::new(&mut cx).class("invalid").entity();
        style_system(&mut cx);

        let resolved = |store: &crate::storage::animatable_var_set::AnimatableVarSet<Units>,
                        entity| {
            store.get_resolved(entity, &cx.style.custom_units_props)
        };
        let px = |value| Some(Units::Pixels(value));
        assert_eq!(resolved(&cx.style.padding_top, spacing), px(1.0));
        assert_eq!(resolved(&cx.style.padding_right, spacing), px(2.0));
        assert_eq!(resolved(&cx.style.padding_bottom, spacing), px(3.0));
        assert_eq!(resolved(&cx.style.padding_left, spacing), px(9.0));
        assert_eq!(resolved(&cx.style.vertical_gap, spacing), px(5.0));
        assert_eq!(resolved(&cx.style.horizontal_gap, spacing), px(7.0));
        assert_eq!(resolved(&cx.style.padding_left, reverse), px(2.0));
        assert_eq!(resolved(&cx.style.vertical_gap, reverse), px(5.0));
        assert_eq!(resolved(&cx.style.horizontal_gap, reverse), px(6.0));
        assert_eq!(resolved(&cx.style.padding_top, variable), px(9.0));
        assert_eq!(resolved(&cx.style.padding_left, variable), px(8.0));
        assert_eq!(resolved(&cx.style.vertical_gap, variable), px(3.0));
        assert_eq!(resolved(&cx.style.horizontal_gap, variable), px(3.0));
        assert_eq!(resolved(&cx.style.padding_left, zero), px(0.0));
        assert_eq!(resolved(&cx.style.vertical_gap, zero), px(0.0));
        assert_eq!(resolved(&cx.style.horizontal_gap, zero), px(0.0));
        assert_eq!(resolved(&cx.style.padding_left, invalid), px(1.0));
        assert_eq!(resolved(&cx.style.horizontal_gap, invalid), px(5.0));
    }

    #[test]
    fn spacing_shorthands_expand_in_keyframes() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            "@keyframes spacing {
                from { padding: 1px 2px; gap: 3px 4px; }
                to { padding: 5px 6px 7px 8px; gap: 9px 10px; }
            }",
        )
        .unwrap();
        let animation = *cx.style.get_animation("spacing").unwrap();
        let values = |frames: &[crate::animation::Keyframe<Units>]| {
            frames.iter().map(|frame| frame.value).collect::<Vec<_>>()
        };
        assert_eq!(
            values(&cx.style.padding_top.get_animation_mut(animation).unwrap().keyframes),
            vec![Units::Pixels(1.0), Units::Pixels(5.0)]
        );
        assert_eq!(
            values(&cx.style.padding_left.get_animation_mut(animation).unwrap().keyframes),
            vec![Units::Pixels(2.0), Units::Pixels(8.0)]
        );
        assert_eq!(
            values(&cx.style.vertical_gap.get_animation_mut(animation).unwrap().keyframes),
            vec![Units::Pixels(3.0), Units::Pixels(9.0)]
        );
        assert_eq!(
            values(&cx.style.horizontal_gap.get_animation_mut(animation).unwrap().keyframes),
            vec![Units::Pixels(4.0), Units::Pixels(10.0)]
        );
    }

    #[test]
    fn standard_property_aliases_apply_and_resolve_variables() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".standard {
                --radius: 12px;
                border-radius: var(--radius);
                border-top-left-radius: 20px;
                row-gap: var(--missing, 4px);
                box-shadow: var(--missing-shadow, 1px 2px #f00);
                wrap: wrap;
                flex-wrap: nowrap;
            }
            .legacy {
                --radius: 12px;
                corner-radius: var(--radius);
                corner-top-left-radius: 20px;
                vertical-gap: var(--missing, 4px);
                shadow: var(--missing-shadow, 1px 2px #f00);
                wrap: no-wrap;
            }",
        )
        .unwrap();
        let standard = Element::new(&mut cx).class("standard").entity();
        let legacy = Element::new(&mut cx).class("legacy").entity();
        style_system(&mut cx);

        assert_eq!(cx.style.wrap.get(standard), Some(&LayoutWrap::NoWrap));
        assert_eq!(cx.style.wrap.get(standard), cx.style.wrap.get(legacy));
        assert_eq!(
            cx.style.corner_top_left_radius.get(standard),
            Some(&LengthOrPercentage::px(20.0))
        );
        let radius = |entity| {
            cx.style.corner_top_right_radius.get_resolved(entity, &cx.style.custom_length_props)
        };
        assert_eq!(radius(standard), Some(LengthOrPercentage::px(12.0)));
        assert_eq!(radius(standard), radius(legacy));
        let gap = |entity| cx.style.vertical_gap.get_resolved(entity, &cx.style.custom_units_props);
        assert_eq!(gap(standard), Some(Units::Pixels(4.0)));
        assert_eq!(gap(standard), gap(legacy));
        let shadow = cx
            .style
            .shadow
            .get_resolved(standard, &cx.style.custom_shadow_props)
            .expect("alias shadow should resolve");
        assert_eq!(shadow.len(), 1);
        assert_eq!(
            Some(shadow),
            cx.style.shadow.get_resolved(legacy, &cx.style.custom_shadow_props)
        );
    }

    #[test]
    fn standard_box_shadow_resolves_inherited_variables_and_fallbacks() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".parent { --shadow: inset rgba(255, 0, 0, 1) 0 2px; }
             .inherited { box-shadow: var(--shadow); }
             .fallback { shadow: var(--missing, inset #f00 0 2px); }",
        )
        .unwrap();
        let parent = Element::new(&mut cx).class("parent").entity();
        let inherited = cx.with_current(parent, |cx| Element::new(cx).class("inherited").entity());
        let fallback = Element::new(&mut cx).class("fallback").entity();
        style_system(&mut cx);

        let expected = vec![Shadow::new(
            Length::zero(),
            Length::px(2.0),
            None,
            None,
            Some(Color::rgb(255, 0, 0)),
            true,
        )];
        for entity in [inherited, fallback] {
            assert_eq!(
                cx.style.shadow.get_resolved(entity, &cx.style.custom_shadow_props),
                Some(expected.clone())
            );
        }
    }

    #[test]
    fn standard_property_aliases_work_in_keyframes_and_transitions() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            "@keyframes rounded { from { border-top-left-radius: 2px; } to { border-top-left-radius: 10px; } }
             .before { border-radius: 2px; row-gap: 2px; box-shadow: 1px 2px black; }
             .after {
                 corner-radius: 10px;
                 vertical-gap: 10px;
                 shadow: 3px 4px black;
                 transition: border-radius 5s, row-gap 5s, box-shadow 5s;
             }",
        )
        .unwrap();

        let animation = *cx.style.get_animation("rounded").unwrap();
        let keyframes =
            &cx.style.corner_top_left_radius.get_animation_mut(animation).unwrap().keyframes;
        assert_eq!(keyframes.len(), 2);

        let entity = Element::new(&mut cx).class("before").entity();
        style_system(&mut cx);
        cx.style.set_class(&cx.tree, entity, "after", true);
        style_system(&mut cx);
        assert!(
            cx.style
                .corner_top_left_radius
                .get_active_animations()
                .unwrap()
                .iter()
                .any(|animation| animation.entities.contains(&entity))
        );
        assert!(
            cx.style
                .vertical_gap
                .get_active_animations()
                .unwrap()
                .iter()
                .any(|animation| animation.entities.contains(&entity))
        );
        assert!(
            cx.style
                .shadow
                .get_active_animations()
                .unwrap()
                .iter()
                .any(|animation| animation.entities.contains(&entity))
        );
    }

    #[test]
    fn transition_shorthand_easing_before_delay_applies() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".before { width: 10px; }
             .after { width: 20px; transition: width 200ms ease 50ms; }",
        )
        .unwrap();
        let entity = Element::new(&mut cx).class("before").entity();
        style_system(&mut cx);
        cx.style.set_class(&cx.tree, entity, "after", true);
        style_system(&mut cx);

        let transition = cx
            .style
            .width
            .get_active_animations()
            .unwrap()
            .iter()
            .find(|animation| animation.entities.contains(&entity))
            .unwrap();
        assert_eq!(transition.duration, std::time::Duration::from_millis(200));
        assert_eq!(transition.delay, std::time::Duration::from_millis(50));
        assert_eq!(
            transition.keyframes[0].timing_function.value(0.25),
            crate::animation::TimingFunction::ease().value(0.25)
        );
    }

    #[test]
    fn transition_shorthand_reordered_components_apply() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".before { width: 10px; }
             .after { width: 20px; transition: 200ms ease width 50ms; }",
        )
        .unwrap();
        let entity = Element::new(&mut cx).class("before").entity();
        style_system(&mut cx);
        cx.style.set_class(&cx.tree, entity, "after", true);
        style_system(&mut cx);

        let transition = cx
            .style
            .width
            .get_active_animations()
            .unwrap()
            .iter()
            .find(|animation| animation.entities.contains(&entity))
            .unwrap();
        assert_eq!(transition.duration, std::time::Duration::from_millis(200));
        assert_eq!(transition.delay, std::time::Duration::from_millis(50));
        assert_eq!(
            transition.keyframes[0].timing_function.value(0.25),
            crate::animation::TimingFunction::ease().value(0.25)
        );
    }

    #[test]
    fn recycled_views_resolve_only_their_own_element_selector() {
        let mut cx = Context::default();
        cx.add_stylesheet("label { width: 111px; }").unwrap();

        let label = Label::new(&mut cx, "label").entity();
        style_system(&mut cx);
        assert_eq!(cx.style.element.get(label), Some(&fxhash::hash32("label")));
        assert_eq!(cx.style.width.get(label), Some(&Units::Pixels(111.0)));
        cx.remove(label);

        // The entity manager quarantines freed indices; cycle until it hands this one out again.
        let unnamed = loop {
            let candidate = Unnamed.build(&mut cx, |_| {}).entity();
            if candidate.index() == label.index() {
                break candidate;
            }
            cx.remove(candidate);
        };
        assert_ne!(unnamed.generation(), label.generation());
        assert!(cx.style.element.get(unnamed).is_none());
        style_system(&mut cx);
        assert_ne!(cx.style.width.get(unnamed), Some(&Units::Pixels(111.0)));
        cx.remove(unnamed);

        let replacement = loop {
            let candidate = Label::new(&mut cx, "replacement").entity();
            if candidate.index() == unnamed.index() {
                break candidate;
            }
            cx.remove(candidate);
        };
        assert_ne!(replacement.generation(), unnamed.generation());
        style_system(&mut cx);
        assert_eq!(cx.style.width.get(replacement), Some(&Units::Pixels(111.0)));
    }

    #[test]
    fn adding_a_sibling_restyles_descendants_of_the_previous_last_child() {
        let mut cx = Context::default();
        cx.add_stylesheet(".row:last-child > label { font-size: 30px; }").unwrap();
        let parent = Element::new(&mut cx).entity();
        let mut label = Entity::null();
        cx.with_current(parent, |cx| {
            VStack::new(cx, |cx| {
                label = Label::new(cx, "first").entity();
            })
            .class("row");
        });
        style_system(&mut cx);
        assert_eq!(cx.style.font_size.get(label), Some(&FontSize::from(30.0)));

        cx.with_current(parent, |cx| {
            VStack::new(cx, |_| {}).class("row");
        });
        style_system(&mut cx);
        assert_ne!(cx.style.font_size.get(label), Some(&FontSize::from(30.0)));
    }

    #[test]
    fn inherited_direction_changes_rematch_selectors_and_their_dependents() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".rtl { direction: rtl; }
             label:dir(rtl) { font-size: 30px; }
             .row:dir(rtl) > label { opacity: 0.5; }
             .row:dir(rtl) + .other { opacity: 0.25; }",
        )
        .unwrap();
        let parent = Element::new(&mut cx).entity();
        let mut label = Entity::null();
        let mut sibling = Entity::null();
        cx.with_current(parent, |cx| {
            VStack::new(cx, |cx| {
                label = Label::new(cx, "text").entity();
            })
            .class("row");
            sibling = Element::new(cx).class("other").entity();
        });
        style_system(&mut cx);
        cx.style.set_class(&cx.tree, parent, "rtl", true);
        style_system(&mut cx);
        assert_eq!(cx.style.font_size.get(label), Some(&FontSize::from(30.0)));
        assert_eq!(cx.style.opacity.get(label), Some(&Opacity(0.5)));
        assert_eq!(cx.style.opacity.get(sibling), Some(&Opacity(0.25)));

        cx.style.set_class(&cx.tree, parent, "rtl", false);
        style_system(&mut cx);
        assert_ne!(cx.style.font_size.get(label), Some(&FontSize::from(30.0)));
        assert_ne!(cx.style.opacity.get(label), Some(&Opacity(0.5)));
        assert_ne!(cx.style.opacity.get(sibling), Some(&Opacity(0.25)));
    }

    #[test]
    fn explicit_css_font_size_overrides_inherited_inline_font_size() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".unused { font-size: 10px; }
             label { font-size: 30px; }",
        )
        .unwrap();
        let mut label = Entity::null();
        VStack::new(&mut cx, |cx| {
            label = Label::new(cx, "text").entity();
        })
        .font_size(20.0);
        style_system(&mut cx);
        assert_eq!(cx.style.font_size.get(label), Some(&FontSize::from(30.0)));
    }

    #[test]
    fn direction_dependents_outside_the_changed_subtree_propagate_inheritance() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".trigger.rtl { direction: rtl; }
             .trigger:dir(rtl) + .other { font-size: 30px; }",
        )
        .unwrap();
        let trigger = Element::new(&mut cx).class("trigger").entity();
        let mut label = Entity::null();
        VStack::new(&mut cx, |cx| {
            label = Label::new(cx, "text").entity();
        })
        .class("other");
        style_system(&mut cx);

        cx.style.set_class(&cx.tree, trigger, "rtl", true);
        style_system(&mut cx);

        assert_eq!(cx.style.font_size.get(label), Some(&FontSize::from(30.0)));
    }

    #[test]
    fn chained_direction_dependents_are_resolved_before_matching() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".trigger.rtl { direction: rtl; }
             .trigger:dir(rtl) + .other { direction: rtl; }
             .other:dir(rtl) > label { font-size: 30px; }",
        )
        .unwrap();
        let trigger = Element::new(&mut cx).class("trigger").entity();
        let mut label = Entity::null();
        let other = VStack::new(&mut cx, |cx| {
            label = Label::new(cx, "text").entity();
        })
        .class("other")
        .entity();
        style_system(&mut cx);

        cx.style.set_class(&cx.tree, trigger, "rtl", true);
        style_system(&mut cx);
        assert_eq!(cx.style.direction.get(other), Some(&Direction::RightToLeft));
        assert_eq!(cx.style.font_size.get(label), Some(&FontSize::from(30.0)));

        cx.style.set_class(&cx.tree, trigger, "rtl", false);
        style_system(&mut cx);
        assert_ne!(cx.style.direction.get(other), Some(&Direction::RightToLeft));
        assert_ne!(cx.style.font_size.get(label), Some(&FontSize::from(30.0)));
    }

    #[test]
    fn explicit_font_weight_survives_removal_of_the_same_inherited_rule() {
        let mut cx = Context::default();
        cx.add_stylesheet(".bold { font-weight: 700; }").unwrap();
        let mut label = Entity::null();
        let parent = VStack::new(&mut cx, |cx| {
            label = Label::new(cx, "text").entity();
        })
        .class("bold")
        .entity();
        style_system(&mut cx);
        let weight = cx.style.font_weight.get(label).copied();
        assert!(weight.is_some());
        cx.style.set_class(&cx.tree, label, "bold", true);
        style_system(&mut cx);
        cx.style.set_class(&cx.tree, parent, "bold", false);
        style_system(&mut cx);
        assert_eq!(cx.style.font_weight.get(label).copied(), weight);
    }

    #[test]
    fn relative_font_weight_uses_css_weight_ranges() {
        let mut cx = Context::default();
        cx.add_stylesheet(".bolder { font-weight: bolder; } .lighter { font-weight: lighter; }")
            .unwrap();
        let mut cases = Vec::new();
        for (parent_weight, bolder, lighter) in [
            (100, 400, 100),
            (349, 400, 100),
            (350, 700, 100),
            (549, 700, 100),
            (550, 900, 400),
            (749, 900, 400),
            (750, 900, 700),
            (900, 900, 700),
            (1000, 900, 700),
        ] {
            let mut bold = Entity::null();
            let mut light = Entity::null();
            VStack::new(&mut cx, |cx| {
                bold = Element::new(cx).class("bolder").entity();
                light = Element::new(cx).class("lighter").entity();
            })
            .font_weight(FontWeight(parent_weight));
            cases.push((bold, bolder, light, lighter));
        }
        style_system(&mut cx);
        for (bold, bolder, light, lighter) in cases {
            assert_eq!(cx.style.resolved_font_weight(&cx.tree, bold), Some(FontWeight(bolder)));
            assert_eq!(cx.style.resolved_font_weight(&cx.tree, light), Some(FontWeight(lighter)));
        }
    }

    #[test]
    fn relative_font_weight_inherits_and_updates_with_parent_rules() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".base { font-weight: 400; } .base.heavy { font-weight: 700; }
             .bolder { font-weight: 300; font-weight: bolder; }
             .lighter { font-weight: lighter; }",
        )
        .unwrap();
        let default_weight = Element::new(&mut cx).class("bolder").entity();
        let mut bold = Entity::null();
        let mut nested = Entity::null();
        let mut inherited = Entity::null();
        let mut light = Entity::null();
        let mut inline = Entity::null();
        let parent = VStack::new(&mut cx, |cx| {
            bold = VStack::new(cx, |cx| {
                nested = Element::new(cx).class("bolder").entity();
                inherited = Element::new(cx).entity();
                light = Element::new(cx).class("lighter").entity();
            })
            .class("bolder")
            .entity();
            inline = Element::new(cx).class("bolder").font_weight(FontWeight(300)).entity();
        })
        .class("base")
        .entity();
        style_system(&mut cx);
        assert_eq!(cx.style.resolved_font_weight(&cx.tree, default_weight), Some(FontWeight(700)));

        for (heavy, expected) in [
            (false, [700, 900, 700, 400, 300]),
            (true, [900, 900, 900, 700, 300]),
            (false, [700, 900, 700, 400, 300]),
        ] {
            cx.style.set_class(&cx.tree, parent, "heavy", heavy);
            style_system(&mut cx);
            for (entity, weight) in
                [bold, nested, inherited, light, inline].into_iter().zip(expected)
            {
                assert_eq!(
                    cx.style.resolved_font_weight(&cx.tree, entity),
                    Some(FontWeight(weight)),
                    "heavy={heavy}, entity={entity:?}"
                );
            }
        }

        cx.style.set_class(&cx.tree, bold, "bolder", false);
        style_system(&mut cx);
        assert_eq!(cx.style.resolved_font_weight(&cx.tree, bold), Some(FontWeight(400)));
        assert_eq!(cx.style.resolved_font_weight(&cx.tree, nested), Some(FontWeight(700)));
    }

    #[test]
    fn direction_propagates_through_multiple_sibling_dependencies() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".rtl { direction: rtl; }
             .step:dir(rtl) + .step { direction: rtl; }
             .step:dir(rtl) > label { font-size: 30px; }",
        )
        .unwrap();
        let trigger = Element::new(&mut cx).class("step").entity();
        let mut labels = Vec::new();
        for _ in 0..4 {
            VStack::new(&mut cx, |cx| {
                labels.push(Label::new(cx, "text").entity());
            })
            .class("step");
        }
        style_system(&mut cx);

        for rtl in [true, false] {
            cx.style.set_class(&cx.tree, trigger, "rtl", rtl);
            style_system(&mut cx);
            for label in &labels {
                assert_eq!(cx.style.font_size.get(*label) == Some(&FontSize::from(30.0)), rtl);
            }
        }
    }

    #[test]
    fn cyclic_direction_selectors_terminate_with_consistent_final_matching() {
        let mut cx = Context::default();
        cx.add_stylesheet(
            ".cycle:dir(ltr) { direction: rtl; }
             .cycle:dir(rtl) { direction: ltr; font-size: 30px; }",
        )
        .unwrap();
        let entity = Element::new(&mut cx).class("cycle").entity();
        style_system(&mut cx);
        let direction = cx.style.direction.get(entity).copied();
        assert!(direction.is_some());
        assert_eq!(
            cx.style.font_size.get(entity) == Some(&FontSize::from(30.0)),
            direction == Some(Direction::RightToLeft)
        );
        assert!(cx.style.restyle.is_empty());
        style_system(&mut cx);
        assert_eq!(cx.style.direction.get(entity).copied(), direction);
    }

    #[test]
    fn binding_does_not_share_the_matched_rules_of_a_classed_sibling() {
        let mut cx = Context::default();
        // A positional rule (the default theme has several) restyles the subtrees of the
        // siblings of new entities, which contain the binding.
        cx.add_stylesheet(
            ".icon { opacity: 0.82; }
             .icon:hover { opacity: 1; }
             .unused:first-child { width: 1px; }",
        )
        .unwrap();
        let signal = Signal::new(0);
        let parent = Element::new(&mut cx).entity();
        let mut icon = Entity::null();
        cx.with_current(parent, |cx| {
            icon = Element::new(cx).class("icon").entity();
            Binding::new(cx, signal, |cx| {
                Label::new(cx, "content");
            });
            Element::new(cx);
        });
        let binding = cx.tree.get_child(parent, 1).unwrap();
        assert!(cx.tree.is_ignored(binding));

        style_system(&mut cx);
        assert_eq!(cx.style.opacity.get(icon), Some(&Opacity(0.82)));
        assert_eq!(cx.style.opacity.get(binding), None);

        cx.set_pseudo_class(icon, PseudoClassFlags::HOVER, true);
        style_system(&mut cx);
        assert_eq!(cx.style.opacity.get(icon), Some(&Opacity(1.0)));
        assert_eq!(cx.style.opacity.get(binding), None);
    }

    #[test]
    fn view_without_an_element_does_not_share_an_element_selector_of_its_sibling() {
        let mut cx = Context::default();
        cx.add_stylesheet("label.icon { opacity: 0.5; }").unwrap();
        let parent = Element::new(&mut cx).entity();
        let mut unnamed = Entity::null();
        cx.with_current(parent, |cx| {
            Label::new(cx, "icon").class("icon");
            unnamed = Unnamed.build(cx, |_| {}).class("icon").entity();
            Element::new(cx);
        });

        style_system(&mut cx);
        assert_eq!(cx.style.opacity.get(unnamed), None);
    }

    #[test]
    fn element_selector_applies_to_a_view_after_a_sibling_without_an_element() {
        let mut cx = Context::default();
        cx.add_stylesheet(".icon { width: 10px; } label.icon { opacity: 0.5; }").unwrap();
        let parent = Element::new(&mut cx).entity();
        let mut label = Entity::null();
        cx.with_current(parent, |cx| {
            Unnamed.build(cx, |_| {}).class("icon");
            label = Label::new(cx, "icon").class("icon").entity();
            Element::new(cx);
        });

        style_system(&mut cx);
        assert_eq!(cx.style.opacity.get(label), Some(&Opacity(0.5)));
    }

    #[test]
    fn bindings_are_skipped_by_styling() {
        let mut cx = Context::default();
        let signal = Signal::new(0);
        let parent = Element::new(&mut cx).class("parent").entity();
        cx.with_current(parent, |cx| {
            Binding::new(cx, signal, |cx| {
                Label::new(cx, "content");
            });
        });
        let binding = cx.tree.get_child(parent, 0).unwrap();
        let label = cx.tree.get_child(binding, 0).unwrap();
        assert!(cx.tree.is_ignored(binding));

        // Adding a stylesheet restyles every entity in the tree, including the binding.
        cx.add_stylesheet(
            "* { opacity: 0.5; }
             .parent { font-size: 30px; direction: rtl; }",
        )
        .unwrap();
        style_system(&mut cx);

        assert_eq!(cx.style.opacity.get(label), Some(&Opacity(0.5)));
        assert_eq!(cx.style.font_size.get(label), Some(&FontSize::from(30.0)));
        assert_eq!(cx.style.direction.get(label), Some(&Direction::RightToLeft));
        assert!(cx.style.matched_rules.get(&binding).is_none());
        assert_eq!(cx.style.opacity.get(binding), None);
        assert_eq!(cx.style.font_size.get(binding), None);
        assert_eq!(cx.style.direction.get(binding), None);
    }

    #[test]
    fn restyling_the_siblings_of_new_entities_skips_bindings() {
        let mut cx = Context::default();
        // A positional rule restyles the subtrees of the siblings of new entities.
        cx.add_stylesheet(
            "* { opacity: 0.5; }
             .unused:first-child { width: 1px; }",
        )
        .unwrap();
        let signal = Signal::new(0);
        let parent = Element::new(&mut cx).entity();
        cx.with_current(parent, |cx| {
            Binding::new(cx, signal, |cx| {
                Label::new(cx, "content");
            });
        });
        let binding = cx.tree.get_child(parent, 0).unwrap();
        let label = cx.tree.get_child(binding, 0).unwrap();

        style_system(&mut cx);

        assert_eq!(cx.style.opacity.get(label), Some(&Opacity(0.5)));
        assert!(cx.style.matched_rules.get(&binding).is_none());
        assert_eq!(cx.style.opacity.get(binding), None);
    }
}
