use crate::{
    events::ViewHandler,
    prelude::*,
    style::{Rule, Style, SystemFlags},
};
use fnv::FnvHashMap;
use vizia_id::GenerationalId;
use vizia_storage::{DrawIterator, LayoutTreeIterator};
use vizia_style::{
    matches_selector_list,
    selectors::{
        attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint},
        SelectorImpl,
    },
    selectors::{matching::ElementSelectorFlags, OpaqueElement},
    Element, MatchingContext, MatchingMode, PseudoClass, QuirksMode, SelectorIdent, Selectors,
};

#[derive(Clone)]
pub struct Node<'s, 't, 'v> {
    entity: Entity,
    store: &'s Style,
    tree: &'t Tree<Entity>,
    views: &'v FnvHashMap<Entity, Box<dyn ViewHandler>>,
}

impl<'s, 't, 'v> std::fmt::Debug for Node<'s, 't, 'v> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.entity)
    }
}

impl<'s, 't, 'v> Element for Node<'s, 't, 'v> {
    type Impl = Selectors;

    fn opaque(&self) -> OpaqueElement {
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
        self.tree.get_parent(self.entity).map(|parent| Node {
            entity: parent,
            store: self.store,
            tree: self.tree,
            views: self.views,
        })
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.tree.get_prev_sibling(self.entity).map(|parent| Node {
            entity: parent,
            store: self.store,
            tree: self.tree,
            views: self.views,
        })
    }

    fn next_sibling_element(&self) -> Option<Self> {
        self.tree.get_next_sibling(self.entity).map(|parent| Node {
            entity: parent,
            store: self.store,
            tree: self.tree,
            views: self.views,
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
        if let Some(element) = self.views.get(&self.entity).and_then(|view| view.element()) {
            return element == &local_name.0;
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
        self.store.elements.get(self.entity) == other.store.elements.get(self.entity)
    }

    fn is_link(&self) -> bool {
        false
    }

    // TODO
    fn has_id(
        &self,
        _id: &<Self::Impl as SelectorImpl>::Identifier,
        _case_sensitivity: CaseSensitivity,
    ) -> bool {
        false
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

    fn match_non_ts_pseudo_class<F>(
        &self,
        pc: &<Self::Impl as SelectorImpl>::NonTSPseudoClass,
        _context: &mut MatchingContext<'_, Self::Impl>,
        _flags_setter: &mut F,
    ) -> bool
    where
        F: FnMut(&Self, ElementSelectorFlags),
    {
        if let Some(psudeo_class_flag) = self.store.pseudo_classes.get(self.entity) {
            match pc {
                PseudoClass::Hover => psudeo_class_flag.contains(PseudoClassFlags::HOVER),
                PseudoClass::Active => psudeo_class_flag.contains(PseudoClassFlags::ACTIVE),
                PseudoClass::Over => psudeo_class_flag.contains(PseudoClassFlags::OVER),
                PseudoClass::Focus => psudeo_class_flag.contains(PseudoClassFlags::FOCUS),
                PseudoClass::FocusVisible => {
                    psudeo_class_flag.contains(PseudoClassFlags::FOCUS_VISIBLE)
                }
                PseudoClass::FocusWithin => todo!(),
                PseudoClass::Enabled => todo!(),
                PseudoClass::Disabled => {
                    self.store.disabled.get(self.entity).copied().unwrap_or_default()
                }
                PseudoClass::ReadOnly => todo!(),
                PseudoClass::ReadWrite => todo!(),
                PseudoClass::PlaceHolderShown => todo!(),
                PseudoClass::Default => todo!(),
                PseudoClass::Checked => psudeo_class_flag.contains(PseudoClassFlags::CHECKED),
                PseudoClass::Indeterminate => todo!(),
                PseudoClass::Blank => todo!(),
                PseudoClass::Valid => todo!(),
                PseudoClass::Invalid => todo!(),
                PseudoClass::InRange => todo!(),
                PseudoClass::OutOfRange => todo!(),
                PseudoClass::Required => todo!(),
                PseudoClass::Optional => todo!(),
                PseudoClass::UserValid => todo!(),
                PseudoClass::UserInvalid => todo!(),
                PseudoClass::Lang(_) => todo!(),
                PseudoClass::Dir(_) => todo!(),
                PseudoClass::Custom(name) => {
                    println!("{}", name);
                    todo!()
                }
            }
        } else {
            false
        }
    }
}

pub fn inline_inheritance_system(cx: &mut Context) {
    for entity in cx.tree.into_iter() {
        if let Some(parent) = cx.tree.get_layout_parent(entity) {
            cx.style.disabled.inherit_inline(entity, parent);

            cx.style.font_color.inherit_inline(entity, parent);
            cx.style.font_size.inherit_inline(entity, parent);
            cx.style.font_family.inherit_inline(entity, parent);
            cx.style.font_weight.inherit_inline(entity, parent);
            cx.style.font_style.inherit_inline(entity, parent);
            cx.style.caret_color.inherit_inline(entity, parent);
            cx.style.selection_color.inherit_inline(entity, parent);
        }
    }
}

pub fn shared_inheritance_system(cx: &mut Context) {
    for entity in cx.tree.into_iter() {
        if let Some(parent) = cx.tree.get_layout_parent(entity) {
            cx.style.font_color.inherit_shared(entity, parent);
            cx.style.font_size.inherit_shared(entity, parent);
            cx.style.font_family.inherit_shared(entity, parent);
            cx.style.font_weight.inherit_shared(entity, parent);
            cx.style.font_style.inherit_shared(entity, parent);
            cx.style.caret_color.inherit_shared(entity, parent);
            cx.style.selection_color.inherit_shared(entity, parent);
        }
    }
}

pub fn hoverability_system(cx: &mut Context) {
    let draw_tree = DrawIterator::full(&cx.tree);

    for entity in draw_tree {
        if entity == Entity::root() {
            continue;
        }

        if cx.tree.is_ignored(entity) {
            continue;
        }

        let parent = cx.tree.get_layout_parent(entity).unwrap();

        if !cx.cache.get_hoverability(parent) {
            cx.cache.set_hoverability(entity, false);
        } else {
            if let Some(abilities) = cx.style.abilities.get(entity) {
                cx.cache.set_hoverability(entity, abilities.contains(Abilities::HOVERABLE));
            } else {
                cx.cache.set_hoverability(entity, false);
            }
        }
    }
}

fn link_style_data(style: &mut Style, entity: Entity, matched_rules: &Vec<Rule>) {
    let mut should_relayout = false;
    let mut should_redraw = false;

    // Display
    if style.display.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.visibility.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.z_index.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.overflowx.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.overflowy.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.clip.link(entity, &matched_rules) {
        should_redraw = true;
    }

    // Opacity
    if style.opacity.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.left.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.right.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.top.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.bottom.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.min_left.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.min_right.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.min_top.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.min_bottom.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.max_left.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.max_right.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.max_top.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.max_bottom.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    // Size
    if style.width.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.height.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    // Size Constraints
    if style.max_width.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.min_width.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.max_height.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.min_height.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    // Border
    if style.border_width.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.border_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.border_top_left_shape.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.border_top_right_shape.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.border_bottom_left_shape.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.border_bottom_right_shape.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.border_top_left_radius.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.border_top_right_radius.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.border_bottom_left_radius.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.border_bottom_right_radius.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.outline_width.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.outline_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.outline_offset.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.layout_type.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.position_type.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    // Background
    if style.background_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.background_image.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.background_gradient.link(entity, &matched_rules) {
        should_redraw = true;
    }

    // Font
    if style.font_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.font_size.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.font_family.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.font_weight.link(entity, &matched_rules) {
        should_redraw = true;
        should_relayout = true;
    }

    if style.font_style.link(entity, &matched_rules) {
        should_redraw = true;
        should_relayout = true;
    }

    if style.font_stretch.link(entity, &matched_rules) {
        should_redraw = true;
        should_relayout = true;
    }

    if style.text_wrap.link(entity, &matched_rules) {
        should_redraw = true;
        should_relayout = true;
    }

    if style.selection_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.caret_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    // Outer Shadow
    if style.box_shadow.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.child_left.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.child_right.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.child_top.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.child_bottom.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.row_between.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.col_between.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.cursor.link(entity, &matched_rules) {
        should_redraw = true;
    }

    // Transform
    if style.transform.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.transform_origin.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.translate.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.rotate.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.scale.link(entity, &matched_rules) {
        should_redraw = true;
    }

    //
    if should_relayout {
        style.system_flags.set(SystemFlags::RELAYOUT, true);
    }

    if should_redraw {
        style.system_flags.set(SystemFlags::REDRAW, true);
    }
}

// Iterate tree and determine the matched style rules for each entity. Link the entity to the style data.
pub fn style_system(cx: &mut Context) {
    if cx.style.system_flags.contains(SystemFlags::RESTYLE) {
        hoverability_system(cx);

        // let mut prev_entity = None;

        // let mut matched_rule_ids = Vec::with_capacity(100);
        // let mut prev_matched_rule_ids = Vec::with_capacity(100);

        let iterator = LayoutTreeIterator::full(&cx.tree);

        // Loop through all entities
        for entity in iterator {
            let mut matched_rules = Vec::with_capacity(100);
            for (rule, (specificity, selector_list)) in cx.style.selectors.iter() {
                // println!("selector_list: {:?} {}", selector_list, entity);
                let mut context =
                    MatchingContext::new(MatchingMode::Normal, None, None, QuirksMode::NoQuirks);
                if matches_selector_list(
                    selector_list,
                    &Node { entity, store: &cx.style, tree: &cx.tree, views: &cx.views },
                    &mut context,
                ) {
                    matched_rules.push((*rule, *specificity));
                }
            }

            matched_rules.sort_by_cached_key(|(_, s)| *s);
            matched_rules.reverse();

            // println!("Matched rules: {} {:?}", entity, matched_rules);
            link_style_data(
                &mut cx.style,
                entity,
                &matched_rules.iter().map(|(rule, _)| *rule).collect::<Vec<_>>(),
            );
        }

        cx.style.system_flags.set(SystemFlags::RESTYLE, false);
    }
}
