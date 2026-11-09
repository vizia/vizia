//! Selector analysis used to limit restyling to the entities that a change can actually affect.
//!
//! When a pseudo-class state (e.g. `:hover`), a class, or the id of an entity changes, only the
//! entities whose matched rules may change need to be restyled. The [`InvalidationMap`] is built
//! from the style rules and describes, for every state flag, class and id, which compounds of which
//! selectors depend on it and how far from the changed entity the subject of the selector can be.

use hashbrown::HashSet;
use vizia_storage::{LayoutChildIterator, LayoutParentIterator, LayoutTreeIterator, Tree};
use vizia_style::{
    PseudoClass, Selectors,
    precomputed_hash::PrecomputedHash,
    selectors::parser::{Combinator, Component, Selector},
};

use super::{PseudoClassFlags, Rule, Style, StyleRule};
use crate::entity::Entity;

/// The set of entities, relative to a changed entity, which may need to be restyled.
///
/// Scopes are ordered such that each scope contains all of the entities of the previous one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Scope {
    /// Only the changed entity.
    Subject,
    /// The changed entity and its layout descendants.
    Descendants,
    /// The layout subtree of the parent of the changed entity, i.e. the entity, its siblings, and
    /// all of their descendants.
    Siblings,
    /// The whole tree.
    All,
}

impl Scope {
    /// The scope of a compound given the combinator immediately to its right.
    fn for_combinator(combinator: Combinator) -> Self {
        match combinator {
            Combinator::Child | Combinator::Descendant | Combinator::PseudoElement => {
                Scope::Descendants
            }
            Combinator::NextSibling
            | Combinator::LaterSibling
            | Combinator::SlotAssignment
            | Combinator::Part => Scope::Siblings,
        }
    }
}

/// A change to an entity which can affect which selectors it matches.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Change<'a> {
    /// Pseudo-class state flags were set or cleared.
    State(PseudoClassFlags),
    /// A class was added or removed.
    Class(&'a str),
    /// An id was set or removed.
    Id(&'a str),
}

/// The pseudo-class states, classes and ids which matching a compound selector depends on.
#[derive(Debug, Default, Clone)]
struct Dependencies {
    flags: PseudoClassFlags,
    classes: Vec<String>,
    ids: Vec<String>,
}

impl Dependencies {
    fn is_empty(&self) -> bool {
        self.flags.is_empty() && self.classes.is_empty() && self.ids.is_empty()
    }

    fn extend(&mut self, other: Dependencies) {
        self.flags |= other.flags;
        self.classes.extend(other.classes);
        self.ids.extend(other.ids);
    }

    fn depends_on(&self, change: Change) -> bool {
        match change {
            Change::State(flags) => self.flags.intersects(flags),
            Change::Class(class) => self.classes.iter().any(|c| c == class),
            Change::Id(id) => self.ids.iter().any(|i| i == id),
        }
    }
}

/// The element, id and class requirements of a compound selector. An entity which doesn't satisfy
/// them can't match the compound, so a change to it can't affect matching through the compound.
#[derive(Debug, Default, Clone)]
struct CompoundFilter {
    element: Option<u32>,
    id: Option<String>,
    classes: Vec<String>,
}

impl CompoundFilter {
    fn matches(&self, style: &Style, entity: Entity) -> bool {
        self.matches_except(style, entity, None)
    }

    /// Like [`matches`](Self::matches), but ignores the requirement on the attribute affected by
    /// `change`, since the entity may have matched it before the change or may match it after.
    fn matches_except(&self, style: &Style, entity: Entity, change: Option<Change>) -> bool {
        if let Some(element) = self.element {
            if style.element.get(entity) != Some(&element) {
                return false;
            }
        }

        if let Some(id) = &self.id {
            if !matches!(change, Some(Change::Id(_))) && style.ids.get(entity) != Some(id) {
                return false;
            }
        }

        let classes = style.classes.get(entity);
        self.classes.iter().all(|class| {
            matches!(change, Some(Change::Class(changed)) if changed == class)
                || classes.is_some_and(|classes| classes.contains(class))
        })
    }
}

#[derive(Debug, Clone)]
struct Dependency {
    dependencies: Dependencies,
    /// `None` if any entity may match the compound.
    filter: Option<CompoundFilter>,
    scope: Scope,
    /// For a compound separated from the subject only by child combinators, the filters of the
    /// compounds below it, ending with the subject. Only the descendants along matching paths can
    /// be affected, so this is used instead of `scope`.
    path: Option<Box<[CompoundFilter]>>,
}

/// The entities which may need to be restyled when an entity changes.
#[derive(Debug, Default)]
pub(crate) struct Invalidation {
    /// The scope, relative to the changed entity, to restyle.
    pub(crate) scope: Option<Scope>,
    /// Indices of dependencies whose paths below the changed entity should be restyled.
    paths: Vec<usize>,
}

/// Describes how the style rules depend on the state and attributes of entities.
#[derive(Debug, Default)]
pub(crate) struct InvalidationMap {
    /// The rules generation this map was built for.
    generation: u64,
    dependencies: Vec<Dependency>,
    /// Union of all state flags used by any selector.
    state_flags: PseudoClassFlags,
    /// All classes used by any selector.
    classes: HashSet<String>,
    /// All ids used by any selector.
    ids: HashSet<String>,
    /// Whether a change to an entity can affect the matching of its siblings, e.g. because of
    /// sibling combinators, `:nth-child(... of S)`, or `:has()` in a non-subject position.
    pub(crate) sibling_dependent: bool,
    /// Whether a change to an entity can affect the matching of its ancestors (`:has()` and
    /// `:empty`).
    pub(crate) ancestor_dependent: bool,
    /// Whether selectors depend on the position of an entity among its siblings, e.g.
    /// `:first-child` or `:nth-child()`.
    pub(crate) positional: bool,
}

/// State collected for a compound selector while walking a selector.
#[derive(Default)]
struct Compound {
    dependencies: Dependencies,
    filter: CompoundFilter,
}

impl InvalidationMap {
    pub(crate) fn new(
        generation: u64,
        rules: &indexmap::IndexMap<Rule, StyleRule>,
        variable_rules: &HashSet<Rule>,
    ) -> Self {
        let mut map = Self { generation, ..Default::default() };

        for (rule, style_rule) in rules.iter() {
            // Rules which define variables affect the descendants which inherit them, so the
            // subjects of paths would not be enough.
            let defines_variables = variable_rules.contains(rule);
            let (dependencies, filter) =
                map.walk_selector(&style_rule.selector, None, false, !defines_variables);
            let scope = if defines_variables { Scope::Descendants } else { Scope::Subject };
            map.push(dependencies, Some(filter), scope, None);
        }

        map
    }

    pub(crate) fn generation(&self) -> u64 {
        self.generation
    }

    fn push(
        &mut self,
        dependencies: Dependencies,
        filter: Option<CompoundFilter>,
        scope: Scope,
        path: Option<Box<[CompoundFilter]>>,
    ) {
        if dependencies.is_empty() {
            return;
        }

        self.state_flags |= dependencies.flags;
        self.classes.extend(dependencies.classes.iter().cloned());
        self.ids.extend(dependencies.ids.iter().cloned());
        self.dependencies.push(Dependency { dependencies, filter, scope, path });
    }

    /// Whether any selector depends on the attribute affected by a change.
    fn may_depend_on(&self, change: Change) -> bool {
        match change {
            Change::State(flags) => flags.intersects(self.state_flags),
            Change::Class(class) => self.classes.contains(class),
            Change::Id(id) => self.ids.contains(id),
        }
    }

    /// Walks a selector, recording dependencies for every compound except the subject compound,
    /// whose dependencies and filter are returned so they can be attributed by the caller.
    ///
    /// `min_scope` is the scope of the compound containing a nested selector (`:is()`, `:not()`,
    /// ...), which bounds the scope of any of the nested compounds. `in_has` is true for selectors
    /// nested in `:has()`, whose state can affect any ancestor, and so the whole tree. `paths`
    /// enables recording the paths from compounds to the subject through child combinators.
    fn walk_selector(
        &mut self,
        selector: &Selector<Selectors>,
        min_scope: Option<Scope>,
        in_has: bool,
        paths: bool,
    ) -> (Dependencies, CompoundFilter) {
        let paths = paths && min_scope.is_none() && !in_has;
        let mut compound = Compound::default();
        // `None` for the subject compound.
        let mut scope: Option<Scope> = None;
        let mut subject = None;
        // The filters of the compounds visited so far, starting with the subject.
        let mut filters = Vec::new();
        // Whether only child combinators have been visited so far.
        let mut child_chain = true;

        // Components are stored in match order, i.e. from the subject compound to the left.
        for component in selector.iter_raw_match_order() {
            if let Component::Combinator(combinator) = component {
                let finished = std::mem::take(&mut compound);
                let path = (paths && child_chain && scope.is_some())
                    .then(|| filters.iter().rev().cloned().collect());
                if paths {
                    filters.push(finished.filter.clone());
                }
                self.finish_compound(finished, scope, path, min_scope, in_has, &mut subject);

                child_chain &= matches!(combinator, Combinator::Child);

                if matches!(combinator, Combinator::NextSibling | Combinator::LaterSibling) {
                    self.sibling_dependent = true;
                }

                scope = Some(Scope::for_combinator(*combinator));
                continue;
            }

            let compound_scope = scope.max(min_scope);
            self.visit_component(component, &mut compound, compound_scope, in_has);
        }

        let path = (paths && child_chain && scope.is_some())
            .then(|| filters.iter().rev().cloned().collect());
        self.finish_compound(compound, scope, path, min_scope, in_has, &mut subject);

        subject.unwrap_or_default()
    }

    fn finish_compound(
        &mut self,
        compound: Compound,
        scope: Option<Scope>,
        path: Option<Box<[CompoundFilter]>>,
        min_scope: Option<Scope>,
        in_has: bool,
        subject: &mut Option<(Dependencies, CompoundFilter)>,
    ) {
        if in_has {
            self.push(compound.dependencies, None, Scope::All, None);
            return;
        }

        match scope {
            None => *subject = Some((compound.dependencies, compound.filter)),
            Some(scope) => {
                let scope = scope.max(min_scope.unwrap_or(Scope::Subject));
                // Nested compounds may be negated, so their filter can't be relied upon.
                let filter = if min_scope.is_none() { Some(compound.filter) } else { None };
                self.push(compound.dependencies, filter, scope, path);
            }
        }
    }

    fn visit_component(
        &mut self,
        component: &Component<Selectors>,
        compound: &mut Compound,
        scope: Option<Scope>,
        in_has: bool,
    ) {
        match component {
            Component::LocalName(local_name) => {
                compound.filter.element = Some(local_name.name.precomputed_hash());
            }

            Component::ID(id) => {
                compound.filter.id = Some(id.0.clone());
                compound.dependencies.ids.push(id.0.clone());
            }

            Component::Class(class) => {
                compound.filter.classes.push(class.0.clone());
                compound.dependencies.classes.push(class.0.clone());
            }

            Component::NonTSPseudoClass(pseudo_class) => {
                if let PseudoClass::Has(selectors) = pseudo_class {
                    self.visit_has(selectors.slice().iter(), scope);
                } else if let Some(flag) = pseudo_class_flag(pseudo_class) {
                    compound.dependencies.flags |= flag;
                }
            }

            Component::Negation(list) | Component::Is(list) | Component::Where(list) => {
                let nested_scope = scope.unwrap_or(Scope::Subject);
                for selector in list.slice() {
                    let (dependencies, _) =
                        self.walk_selector(selector, Some(nested_scope), in_has, false);
                    compound.dependencies.extend(dependencies);
                }
            }

            Component::Nth(_) => self.positional = true,

            Component::NthOf(data) => {
                self.positional = true;
                self.sibling_dependent = true;
                // The state of any sibling can affect the index of the entity.
                for selector in data.selectors() {
                    let (dependencies, _) =
                        self.walk_selector(selector, Some(Scope::Siblings), in_has, false);
                    self.push(dependencies, None, Scope::Siblings, None);
                }
            }

            Component::Has(relative_selectors) => {
                self.visit_has(relative_selectors.iter().map(|relative| &relative.selector), scope)
            }

            Component::Empty => self.ancestor_dependent = true,

            _ => {}
        }
    }

    fn visit_has<'a>(
        &mut self,
        selectors: impl Iterator<Item = &'a Selector<Selectors>>,
        scope: Option<Scope>,
    ) {
        self.ancestor_dependent = true;
        // A `:has()` outside of the subject compound makes the descendants and siblings of the
        // ancestor depend on the changed entity.
        if scope.is_some() {
            self.sibling_dependent = true;
        }

        for selector in selectors {
            self.walk_selector(selector, None, true, false);
        }
    }

    /// Returns the entities which may need to be restyled when an entity changes.
    pub(crate) fn invalidation(
        &self,
        style: &Style,
        entity: Entity,
        change: Change,
    ) -> Invalidation {
        let mut invalidation = Invalidation::default();
        if !self.may_depend_on(change) {
            return invalidation;
        }

        for (index, dependency) in self.dependencies.iter().enumerate() {
            if !dependency.dependencies.depends_on(change) {
                continue;
            }

            if dependency.path.is_none() && invalidation.scope >= Some(dependency.scope) {
                continue;
            }

            if dependency
                .filter
                .as_ref()
                .is_none_or(|filter| filter.matches_except(style, entity, Some(change)))
            {
                if dependency.path.is_some() {
                    invalidation.paths.push(index);
                } else {
                    invalidation.scope = Some(dependency.scope);
                }
            }
        }

        // The paths are contained in the descendants of the entity.
        if invalidation.scope >= Some(Scope::Descendants) {
            invalidation.paths.clear();
        }

        invalidation
    }

    /// Returns the entities at the end of the paths of an [`Invalidation`] below an entity.
    pub(crate) fn path_entities(
        &self,
        style: &Style,
        tree: &Tree<Entity>,
        entity: Entity,
        invalidation: &Invalidation,
    ) -> Vec<Entity> {
        let mut result = Vec::new();
        for index in invalidation.paths.iter() {
            let Some(path) = &self.dependencies[*index].path else { continue };
            let mut frontier = vec![entity];
            for filter in path.iter() {
                frontier = frontier
                    .iter()
                    .flat_map(|parent| LayoutChildIterator::new(tree, *parent))
                    .filter(|child| filter.matches(style, *child))
                    .collect();
            }
            result.extend(frontier);
        }

        result
    }

    #[cfg(test)]
    fn scope(&self, style: &Style, entity: Entity, change: Change) -> Option<Scope> {
        self.invalidation(style, entity, change).scope
    }

    #[cfg(test)]
    fn state_scope(
        &self,
        style: &Style,
        entity: Entity,
        changed: PseudoClassFlags,
    ) -> Option<Scope> {
        self.scope(style, entity, Change::State(changed))
    }
}

/// Returns the state flag corresponding to a pseudo-class, if any.
fn pseudo_class_flag(pseudo_class: &PseudoClass) -> Option<PseudoClassFlags> {
    Some(match pseudo_class {
        PseudoClass::Hover => PseudoClassFlags::HOVER,
        PseudoClass::Active => PseudoClassFlags::ACTIVE,
        PseudoClass::Over => PseudoClassFlags::OVER,
        PseudoClass::Focus => PseudoClassFlags::FOCUS,
        PseudoClass::FocusVisible => PseudoClassFlags::FOCUS_VISIBLE,
        PseudoClass::FocusWithin => PseudoClassFlags::FOCUS_WITHIN,
        PseudoClass::ReadOnly => PseudoClassFlags::READ_ONLY,
        PseudoClass::ReadWrite => PseudoClassFlags::READ_WRITE,
        PseudoClass::PlaceholderShown => PseudoClassFlags::PLACEHOLDER_SHOWN,
        PseudoClass::Default => PseudoClassFlags::DEFAULT,
        PseudoClass::Checked => PseudoClassFlags::CHECKED,
        PseudoClass::Indeterminate => PseudoClassFlags::INDETERMINATE,
        PseudoClass::Blank => PseudoClassFlags::BLANK,
        PseudoClass::Valid => PseudoClassFlags::VALID,
        PseudoClass::Invalid => PseudoClassFlags::INVALID,
        PseudoClass::InRange => PseudoClassFlags::IN_RANGE,
        PseudoClass::OutOfRange => PseudoClassFlags::OUT_OF_RANGE,
        PseudoClass::Required => PseudoClassFlags::REQUIRED,
        PseudoClass::Optional => PseudoClassFlags::OPTIONAL,
        PseudoClass::UserValid => PseudoClassFlags::USER_VALID,
        PseudoClass::UserInvalid => PseudoClassFlags::USER_INVALID,
        _ => return None,
    })
}

/// Inserts the entities of a scope, relative to the given entity, into the restyle set.
pub(crate) fn mark_scope(
    restyle: &mut HashSet<Entity>,
    tree: &Tree<Entity>,
    entity: Entity,
    scope: Scope,
) {
    match scope {
        Scope::Subject => {
            restyle.insert(entity);
        }
        Scope::Descendants => restyle.extend(LayoutTreeIterator::subtree(tree, entity)),
        Scope::Siblings => {
            restyle.insert(entity);
            let root = tree.get_layout_parent(entity).unwrap_or(entity);
            restyle.extend(LayoutTreeIterator::subtree(tree, root));
        }
        Scope::All => restyle.extend(LayoutTreeIterator::full(tree)),
    }
}

/// Inserts the layout ancestors of an entity (excluding the entity) into the restyle set.
pub(crate) fn mark_ancestors(restyle: &mut HashSet<Entity>, tree: &Tree<Entity>, entity: Entity) {
    restyle.extend(LayoutParentIterator::new(tree, entity).skip(1));
}

#[cfg(test)]
mod tests {
    use super::*;
    use vizia_id::GenerationalId;

    fn style_with(css: &str) -> Style {
        let mut style = Style::default();
        style.parse_theme(css);
        style
    }

    fn map(style: &Style) -> InvalidationMap {
        InvalidationMap::new(0, &style.rules, &style.variable_rules)
    }

    fn entity(index: u64) -> Entity {
        Entity::new(index, 0)
    }

    fn add(style: &mut Style, entity: Entity, element: &str, classes: &[&str]) {
        style.element.insert(entity, fxhash::hash32(element));
        style.classes.insert(entity, classes.iter().map(|class| class.to_string()).collect());
    }

    #[test]
    fn no_dependency_on_unused_state() {
        let style = style_with("button:hover { background-color: red; }");
        let map = map(&style);
        assert_eq!(map.state_scope(&style, entity(1), PseudoClassFlags::FOCUS), None);
    }

    #[test]
    fn subject_state_with_filter() {
        let mut style = style_with(".row:hover { background-color: red; }");
        add(&mut style, entity(1), "element", &["row"]);
        add(&mut style, entity(2), "element", &["other"]);
        let map = map(&style);
        assert_eq!(
            map.state_scope(&style, entity(1), PseudoClassFlags::HOVER),
            Some(Scope::Subject)
        );
        assert_eq!(map.state_scope(&style, entity(2), PseudoClassFlags::HOVER), None);
        assert!(!map.sibling_dependent);
        assert!(!map.ancestor_dependent);
    }

    #[test]
    fn ancestor_state_affects_descendants() {
        let mut style = style_with("scrollview.v-scroll:hover > scrollbar .thumb { opacity: 1; }");
        add(&mut style, entity(1), "scrollview", &["v-scroll"]);
        add(&mut style, entity(2), "scrollview", &[]);
        let map = map(&style);
        assert_eq!(
            map.state_scope(&style, entity(1), PseudoClassFlags::HOVER),
            Some(Scope::Descendants)
        );
        assert_eq!(map.state_scope(&style, entity(2), PseudoClassFlags::HOVER), None);
    }

    #[test]
    fn sibling_combinator_affects_siblings() {
        let mut style = style_with("button:hover + label { color: red; }");
        add(&mut style, entity(1), "button", &[]);
        let map = map(&style);
        assert_eq!(
            map.state_scope(&style, entity(1), PseudoClassFlags::HOVER),
            Some(Scope::Siblings)
        );
        assert!(map.sibling_dependent);
    }

    #[test]
    fn negated_state_ignores_nested_filter() {
        let mut style = style_with("button:not(.a:hover) { color: red; }");
        add(&mut style, entity(1), "button", &[]);
        let map = map(&style);
        assert_eq!(
            map.state_scope(&style, entity(1), PseudoClassFlags::HOVER),
            Some(Scope::Subject)
        );
    }

    #[test]
    fn has_state_affects_whole_tree() {
        let mut style = style_with("button-group:has(button:hover) { color: red; }");
        add(&mut style, entity(1), "label", &[]);
        let map = map(&style);
        assert_eq!(map.state_scope(&style, entity(1), PseudoClassFlags::HOVER), Some(Scope::All));
        assert!(map.ancestor_dependent);
        assert!(!map.sibling_dependent);
    }

    #[test]
    fn variable_rules_affect_descendants() {
        let mut style = style_with("button:hover { --accent: red; }");
        add(&mut style, entity(1), "button", &[]);
        let map = map(&style);
        assert_eq!(
            map.state_scope(&style, entity(1), PseudoClassFlags::HOVER),
            Some(Scope::Descendants)
        );
    }

    #[test]
    fn child_combinator_state_affects_matching_paths() {
        let mut style =
            style_with("scrollview:hover > scrollbar.vertical > .thumb { opacity: 1; }");
        let mut tree = Tree::new();
        let (scrollview, scrollbar, other, thumb, track, content) =
            (entity(1), entity(2), entity(3), entity(4), entity(5), entity(6));
        tree.add(scrollview, Entity::root()).unwrap();
        tree.add(scrollbar, scrollview).unwrap();
        tree.add(other, scrollview).unwrap();
        tree.add(thumb, scrollbar).unwrap();
        tree.add(track, scrollbar).unwrap();
        tree.add(content, other).unwrap();
        add(&mut style, scrollview, "scrollview", &[]);
        add(&mut style, scrollbar, "scrollbar", &["vertical"]);
        add(&mut style, other, "scrollbar", &["horizontal"]);
        add(&mut style, thumb, "element", &["thumb"]);
        add(&mut style, track, "element", &["track"]);
        add(&mut style, content, "element", &["thumb"]);

        let map = map(&style);
        let invalidation =
            map.invalidation(&style, scrollview, Change::State(PseudoClassFlags::HOVER));
        assert_eq!(invalidation.scope, None);
        assert_eq!(map.path_entities(&style, &tree, scrollview, &invalidation), vec![thumb]);
    }

    #[test]
    fn positional_selectors() {
        let style = style_with("button-group > :first-child { color: red; }");
        let map = map(&style);
        assert!(map.positional);
        assert!(!map.sibling_dependent);
    }

    #[test]
    fn unreferenced_class_has_no_dependents() {
        let mut style = style_with(".foo { color: red; }");
        add(&mut style, entity(1), "label", &[]);
        let map = map(&style);
        assert!(!map.may_depend_on(Change::Class("bar")));
        assert_eq!(map.scope(&style, entity(1), Change::Class("bar")), None);
    }

    #[test]
    fn subject_class_change() {
        let mut style = style_with("button.foo { color: red; }");
        add(&mut style, entity(1), "button", &[]);
        add(&mut style, entity(2), "button", &["foo"]);
        add(&mut style, entity(3), "label", &[]);
        add(&mut style, entity(4), "element", &["foo"]);
        let map = map(&style);
        assert_eq!(map.scope(&style, entity(1), Change::Class("foo")), Some(Scope::Subject));
        assert_eq!(map.scope(&style, entity(2), Change::Class("foo")), Some(Scope::Subject));
        assert_eq!(map.scope(&style, entity(3), Change::Class("foo")), None);
        assert_eq!(map.scope(&style, entity(4), Change::Class("foo")), None);
    }

    #[test]
    fn ancestor_class_affects_descendants() {
        let mut style = style_with(".foo label { color: red; }");
        add(&mut style, entity(1), "element", &[]);
        let map = map(&style);
        assert_eq!(map.scope(&style, entity(1), Change::Class("foo")), Some(Scope::Descendants));
    }

    #[test]
    fn sibling_class_affects_siblings() {
        let mut style = style_with(".foo + label { color: red; }");
        add(&mut style, entity(1), "element", &[]);
        let map = map(&style);
        assert_eq!(map.scope(&style, entity(1), Change::Class("foo")), Some(Scope::Siblings));
    }

    #[test]
    fn has_class_affects_whole_tree() {
        let mut style = style_with("element:has(.foo) { color: red; }");
        add(&mut style, entity(1), "label", &[]);
        let map = map(&style);
        assert_eq!(map.scope(&style, entity(1), Change::Class("foo")), Some(Scope::All));
    }

    #[test]
    fn negated_class_affects_subject() {
        let mut style = style_with("button:not(.foo) { color: red; }");
        add(&mut style, entity(1), "button", &["foo"]);
        let map = map(&style);
        assert_eq!(map.scope(&style, entity(1), Change::Class("foo")), Some(Scope::Subject));
    }

    #[test]
    fn id_change() {
        let mut style = style_with("button#save { color: red; }");
        add(&mut style, entity(1), "button", &[]);
        add(&mut style, entity(2), "label", &[]);
        let map = map(&style);
        assert_eq!(map.scope(&style, entity(1), Change::Id("save")), Some(Scope::Subject));
        assert_eq!(map.scope(&style, entity(2), Change::Id("save")), None);
        assert_eq!(map.scope(&style, entity(1), Change::Id("cancel")), None);
        assert!(!map.may_depend_on(Change::Id("cancel")));
    }
}
