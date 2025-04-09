use crate::{cache::CachedData, prelude::*};
#[cfg(feature = "rayon")]
use dashmap::{DashMap, ReadOnlyView};
use hashbrown::HashMap;
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use vizia_storage::{LayoutParentIterator, TreeBreadthIterator};
use vizia_style::{
    matches_selector,
    precomputed_hash::PrecomputedHash,
    selectors::{
        attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint},
        bloom::BloomFilter,
        context::{MatchingForInvalidation, NeedsSelectorFlags, SelectorCaches},
        matching::ElementSelectorFlags,
        parser::{Component, NthType},
        OpaqueElement, SelectorImpl,
    },
    Element, MatchingContext, MatchingMode, PseudoClass, QuirksMode, SelectorIdent, Selectors,
};

/// A node used for style matching.
#[derive(Clone)]
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

/// Used for selector matching.
impl Element for Node<'_, '_> {
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
        if let Some(element) = self.store.element.get(self.entity) {
            if let Some(other_element) = self.store.element.get(other.entity) {
                return element == other_element;
            }
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
        if let Some(id) = self.store.ids.get(self.entity) {
            *id == name.0
        } else {
            false
        }
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
        if let Some(psudeo_class_flag) = self.store.pseudo_classes.get(self.entity) {
            match pc {
                PseudoClass::Hover => psudeo_class_flag.contains(PseudoClassFlags::HOVER),
                PseudoClass::Active => psudeo_class_flag.contains(PseudoClassFlags::ACTIVE),
                PseudoClass::Over => psudeo_class_flag.contains(PseudoClassFlags::OVER),
                PseudoClass::Focus => psudeo_class_flag.contains(PseudoClassFlags::FOCUS),
                PseudoClass::FocusVisible => {
                    psudeo_class_flag.contains(PseudoClassFlags::FOCUS_VISIBLE)
                }
                PseudoClass::FocusWithin => {
                    psudeo_class_flag.contains(PseudoClassFlags::FOCUS_WITHIN)
                }
                PseudoClass::Enabled => {
                    self.store.disabled.get(self.entity).map(|disabled| !*disabled).unwrap_or(true)
                }
                PseudoClass::Disabled => {
                    self.store.disabled.get(self.entity).copied().unwrap_or_default()
                }
                PseudoClass::ReadOnly => psudeo_class_flag.contains(PseudoClassFlags::READ_ONLY),
                PseudoClass::ReadWrite => psudeo_class_flag.contains(PseudoClassFlags::READ_WRITE),
                PseudoClass::PlaceholderShown => {
                    psudeo_class_flag.contains(PseudoClassFlags::PLACEHOLDER_SHOWN)
                }
                PseudoClass::Default => psudeo_class_flag.contains(PseudoClassFlags::DEFAULT),
                PseudoClass::Checked => psudeo_class_flag.contains(PseudoClassFlags::CHECKED),
                PseudoClass::Indeterminate => {
                    psudeo_class_flag.contains(PseudoClassFlags::INDETERMINATE)
                }
                PseudoClass::Blank => psudeo_class_flag.contains(PseudoClassFlags::BLANK),
                PseudoClass::Valid => psudeo_class_flag.contains(PseudoClassFlags::VALID),
                PseudoClass::Invalid => psudeo_class_flag.contains(PseudoClassFlags::INVALID),
                PseudoClass::InRange => psudeo_class_flag.contains(PseudoClassFlags::IN_RANGE),
                PseudoClass::OutOfRange => {
                    psudeo_class_flag.contains(PseudoClassFlags::OUT_OF_RANGE)
                }
                PseudoClass::Required => psudeo_class_flag.contains(PseudoClassFlags::REQUIRED),
                PseudoClass::Optional => psudeo_class_flag.contains(PseudoClassFlags::OPTIONAL),
                PseudoClass::UserValid => psudeo_class_flag.contains(PseudoClassFlags::USER_VALID),
                PseudoClass::UserInvalid => {
                    psudeo_class_flag.contains(PseudoClassFlags::USER_INVALID)
                }
                PseudoClass::Lang(_) => todo!(),
                PseudoClass::Dir(_) => todo!(),
                PseudoClass::Custom(name) => {
                    println!("custom: {}", name);
                    todo!()
                }
            }
        } else {
            false
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

/// Link inheritable inline properties to their parent.
pub(crate) fn inline_inheritance_system(cx: &mut Context, redraw_entities: &mut Vec<Entity>) {
    for entity in cx.tree.into_iter() {
        if let Some(parent) = cx.tree.get_layout_parent(entity) {
            if cx.style.disabled.inherit_inline(entity, parent)
                | cx.style.caret_color.inherit_inline(entity, parent)
                | cx.style.selection_color.inherit_inline(entity, parent)
            {
                redraw_entities.push(entity);
            }

            if cx.style.font_color.inherit_inline(entity, parent)
                | cx.style.font_size.inherit_inline(entity, parent)
                | cx.style.font_family.inherit_inline(entity, parent)
                | cx.style.font_weight.inherit_inline(entity, parent)
                | cx.style.font_slant.inherit_inline(entity, parent)
                | cx.style.font_width.inherit_inline(entity, parent)
                | cx.style.text_decoration_line.inherit_inline(entity, parent)
                | cx.style.text_stroke_width.inherit_inline(entity, parent)
                | cx.style.text_stroke_style.inherit_inline(entity, parent)
                | cx.style.font_variation_settings.inherit_inline(entity, parent)
            {
                cx.style.needs_text_update(entity);
            }
        }
    }
}

/// Link inheritable shared properties to their parent.
pub(crate) fn shared_inheritance_system(cx: &mut Context, redraw_entities: &mut Vec<Entity>) {
    for entity in cx.tree.into_iter() {
        if let Some(parent) = cx.tree.get_layout_parent(entity) {
            if cx.style.font_color.inherit_shared(entity, parent)
                | cx.style.font_size.inherit_shared(entity, parent)
                | cx.style.font_family.inherit_shared(entity, parent)
                | cx.style.font_weight.inherit_shared(entity, parent)
                | cx.style.font_slant.inherit_shared(entity, parent)
                | cx.style.font_width.inherit_shared(entity, parent)
                | cx.style.text_decoration_line.inherit_shared(entity, parent)
                | cx.style.text_stroke_width.inherit_shared(entity, parent)
                | cx.style.text_stroke_style.inherit_shared(entity, parent)
                | cx.style.font_variation_settings.inherit_shared(entity, parent)
            {
                cx.style.needs_text_update(entity);
            }

            if cx.style.caret_color.inherit_shared(entity, parent)
                | cx.style.selection_color.inherit_shared(entity, parent)
            {
                redraw_entities.push(entity);
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

    // Display
    if style.display.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.visibility.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.z_index.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.overflowx.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.overflowy.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.clip_path.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.backdrop_filter.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.blend_mode.link(entity, matched_rules) {
        should_redraw = true;
    }

    // Opacity
    if style.opacity.link(entity, matched_rules) {
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

    if style.left.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.right.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.top.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.bottom.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    // Size
    if style.width.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.height.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    // Size Constraints
    if style.max_width.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.min_width.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.max_height.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.min_height.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    // Gap Constraints
    if style.max_horizontal_gap.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.min_horizontal_gap.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.max_vertical_gap.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.min_vertical_gap.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    // Border
    if style.border_width.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
        cache.path.remove(entity);
    }

    if style.border_color.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.border_style.link(entity, matched_rules) {
        should_redraw = true;
    }

    // Corner

    if style.corner_top_left_shape.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.corner_top_right_shape.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.corner_bottom_left_shape.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.corner_bottom_right_shape.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.corner_top_left_radius.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.corner_top_right_radius.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.corner_bottom_left_radius.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.corner_bottom_right_radius.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.outline_width.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.outline_color.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.outline_offset.link(entity, matched_rules) {
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

    // Background
    if style.background_color.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.background_image.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.background_size.link(entity, matched_rules) {
        should_redraw = true;
    }

    // Font
    if style.font_color.link(entity, matched_rules) {
        should_redraw = true;
        should_reflow = true;
    }

    if style.font_size.link(entity, matched_rules) {
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

    if style.line_clamp.link(entity, matched_rules) {
        should_redraw = true;
        should_reflow = true;
    }

    if style.selection_color.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.caret_color.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.text_decoration_line.link(entity, matched_rules) {
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

    if style.underline_style.link(entity, matched_rules) {
        should_redraw = true;
        should_reflow = true;
    }

    if style.underline_color.link(entity, matched_rules) {
        should_redraw = true;
        should_reflow = true;
    }

    if style.overline_style.link(entity, matched_rules) {
        should_redraw = true;
        should_reflow = true;
    }

    if style.overline_color.link(entity, matched_rules) {
        should_redraw = true;
        should_reflow = true;
    }

    if style.strikethrough_style.link(entity, matched_rules) {
        should_redraw = true;
        should_reflow = true;
    }

    if style.strikethrough_color.link(entity, matched_rules) {
        should_redraw = true;
        should_reflow = true;
    }

    // Outer Shadow
    if style.shadow.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.padding_left.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.padding_right.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.padding_top.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.padding_bottom.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.vertical_gap.link(entity, matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.horizontal_gap.link(entity, matched_rules) {
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
        should_redraw = true;
    }

    if style.transform_origin.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.translate.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.rotate.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.scale.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.fill.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.animation_delay.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.animation_duration.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.animation_fill_mode.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.animation_direction.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.animation_iteration_count.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.animation_timing_function.link(entity, matched_rules) {
        should_redraw = true;
    }

    if style.animation_name.link(entity, matched_rules) {
        // Get the animation name
        if let Some(name) = style.animation_name.get(entity) {
            // Get the animation id from the animation name.
            if let Some(animation_id) = style.animations.get(name) {
                let animation = style.get_animation_for_entity(entity);
                style.enqueue_animation(entity, *animation_id, animation);
            }
        }

        should_redraw = true;
    }

    //
    if should_relayout {
        style.system_flags.set(SystemFlags::RELAYOUT, true);
    }

    if should_redraw {
        redraw_entities.push(entity);
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
        let matches = matches_selector(&rule.selector, 0, Some(&rule.hashes), &node, &mut context);

        if matches {
            matched_rules.push((*rule_id, rule.selector.specificity()));
        }
    }

    matched_rules.sort_by_key(|(_, s)| *s);
    matched_rules.reverse();
    matched_rules
}

fn has_same_selector(style: &Style, entity1: Entity, entity2: Entity) -> bool {
    if let Some(element1) = style.element.get(entity1) {
        if let Some(element2) = style.element.get(entity2) {
            if element1 != element2 {
                return false;
            };
        }
    }

    let id1 = if let Some(id) = style.ids.get(entity1) { id } else { "" };
    let id2 = if let Some(id) = style.ids.get(entity2) { id } else { "" };

    if id1 != id2 {
        return false;
    }

    if let Some(classes1) = style.classes.get(entity1) {
        if let Some(classes2) = style.classes.get(entity2) {
            if !classes2.is_subset(classes1) || !classes1.is_subset(classes2) {
                return false;
            }
        }
    }

    if let Some(psudeo_class_flag1) = style.pseudo_classes.get(entity1) {
        if let Some(psudeo_class_flag2) = style.pseudo_classes.get(entity2) {
            if psudeo_class_flag2.bits() != psudeo_class_flag1.bits() {
                return false;
            }
        }
    }

    true
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
    fn build(entities: &[Entity], style: &Style, tree: &Tree<Entity>) -> Self {
        let filter = &mut BloomFilter::default();

        let mut cache = HashMap::new();
        let rules = entities
            .iter()
            .filter_map(|entity| Self::build_inner(*entity, style, tree, filter, &mut cache))
            .collect();

        Self { rules, cache }
    }

    #[cfg(feature = "rayon")]
    fn build_parallel(entities: &[Entity], style: &Style, tree: &Tree<Entity>) -> Self {
        let num_threads = std::thread::available_parallelism().map_or(1, |n| n.get());

        // Potential tuning oppertunity:
        // Lower values make the BloomFilter more effective.
        // Higher values allow more work be done in parellel.
        let min_len = entities.len().div_ceil(num_threads);

        let cache = DashMap::new();
        let rules = entities
            .par_iter()
            .with_min_len(min_len)
            .map_init(BloomFilter::default, |filter, entity| {
                Self::build_inner(*entity, style, tree, filter, &cache)
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
        #[cfg(feature = "rayon")] rule_cache: &DashMap<Entity, Vec<MatchedRulesCache>>,
        #[cfg(not(feature = "rayon"))] rule_cache: &mut HashMap<Entity, Vec<MatchedRulesCache>>,
    ) -> Option<(Entity, (Entity, usize))> {
        compute_element_hash(entity, tree, style, filter);

        let parent = tree.get_layout_parent(entity).unwrap_or(Entity::root());

        let mut matched_index = None;

        if !tree.is_first_child(entity) && !tree.is_last_child(entity) {
            if let Some(cache) = rule_cache.get(&parent) {
                matched_index = cache.iter().position(|entry| {
                    has_same_selector(style, entry.entity, entity)
                        && !has_nth_child_rule(style, &entry.rules)
                });
            }
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
        if entry.rules.is_empty() {
            None
        } else {
            Some(&entry.rules)
        }
    }
}

// Iterates the tree and determines the matching style rules for each entity, then links the entity to the corresponding style rule data.
pub(crate) fn style_system(cx: &mut Context) {
    let mut redraw_entities = Vec::new();

    inline_inheritance_system(cx, &mut redraw_entities);

    if cx.style.restyle.is_empty() {
        return;
    }

    let entities = TreeBreadthIterator::full(&cx.tree)
        .filter(|e| cx.style.restyle.contains(*e))
        .collect::<Vec<_>>();

    let matched_rules = {
        #[cfg(feature = "rayon")]
        {
            MatchedRules::build_parallel(&entities, &cx.style, &cx.tree)
        }
        #[cfg(not(feature = "rayon"))]
        {
            MatchedRules::build(&entities, &cx.style, &cx.tree)
        }
    };

    //  Apply matched rules to entities
    for entity in entities {
        if let Some(matched_rules) = matched_rules.get(&entity) {
            link_style_data(
                &mut cx.style,
                &mut cx.cache,
                &cx.tree,
                entity,
                &mut redraw_entities,
                matched_rules,
            );
        }
    }
    cx.style.restyle.clear();

    shared_inheritance_system(cx, &mut redraw_entities);

    for entity in redraw_entities {
        cx.needs_redraw(entity);
    }
}
