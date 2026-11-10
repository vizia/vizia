pub(crate) mod animatable_set;
pub(crate) mod animatable_var_set;
pub(crate) mod style_set;

use std::sync::atomic::{AtomicU64, Ordering};

/// Incremented whenever an entity gains or loses inline style data, which changes how inline
/// values are inherited and linked. Value updates of existing inline data do not affect it.
static INLINE_STRUCTURE_GENERATION: AtomicU64 = AtomicU64::new(0);

pub(crate) fn inline_structure_generation() -> u64 {
    INLINE_STRUCTURE_GENERATION.load(Ordering::Relaxed)
}

pub(crate) fn bump_inline_structure_generation() {
    INLINE_STRUCTURE_GENERATION.fetch_add(1, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::{
        animatable_set::AnimatableSet, animatable_var_set::AnimatableVarSet, style_set::StyleSet,
    };
    use crate::prelude::*;

    macro_rules! inheritance_test {
        ($name:ident, $store:ty $(, $variables:expr)?) => {
            #[test]
            fn $name() {
                for inline_count in [1, 4] {
                    for rule_index in 0..3 {
                        let mut store = <$store>::default();
                        let parent = Entity::new(1, 0);
                        let child = Entity::new(2, 0);
                        for index in 0..3 {
                            store.insert_rule(Rule::new(index, 0), 30.0 + index as f32);
                        }
                        for index in 0..inline_count {
                            store.insert(Entity::new(index + 10, 0), 10.0);
                        }
                        store.insert(parent, 20.0);
                        let rules = [(Rule::new(rule_index, 0), 0)];
                        store.link(child, &rules $(, $variables)?);
                        assert!(!store.inherit_inline(child, parent));
                        assert_eq!(store.get(child), Some(&(30.0 + rule_index as f32)));

                        store.link(child, &[] $(, $variables)?);
                        assert!(store.inherit_inline(child, parent));
                        assert_eq!(store.get(child), Some(&20.0));
                        assert!(!store.inherit_inline(child, parent));
                    }
                }

                let mut store = <$store>::default();
                let parent = Entity::new(1, 0);
                let child = Entity::new(2, 0);
                store.insert_rule(Rule::new(0, 0), 30.0);
                store.link(parent, &[(Rule::new(0, 0), 0)] $(, $variables)?);
                assert!(store.inherit_shared(child, parent));
                store.insert(parent, 20.0);
                assert!(store.inherit_inline(child, parent));
                assert_eq!(store.get(child), Some(&20.0));
                store.insert(child, 40.0);
                assert!(!store.inherit_inline(child, parent));
                assert_eq!(store.get(child), Some(&40.0));
            }
        };
    }

    inheritance_test!(style_inline_inheritance_preserves_explicit_values, StyleSet<f32>);
    inheritance_test!(animated_inline_inheritance_preserves_explicit_values, AnimatableSet<f32>);
    inheritance_test!(
        variable_inline_inheritance_preserves_explicit_values,
        AnimatableVarSet<f32>,
        &hashbrown::HashMap::new()
    );

    #[test]
    fn shared_inheritance_clears_a_removed_parent_declaration() {
        let mut store = StyleSet::<f32>::default();
        let parent = Entity::new(1, 0);
        let child = Entity::new(2, 0);
        let explicit_child = Entity::new(3, 0);
        store.insert_rule(Rule::new(0, 0), 30.0);
        store.link(parent, &[(Rule::new(0, 0), 0)]);
        store.link(explicit_child, &[(Rule::new(0, 0), 0)]);
        assert!(store.inherit_shared(child, parent));
        assert!(!store.inherit_shared(child, parent));
        assert_eq!(store.get(child), Some(&30.0));

        store.link(parent, &[]);
        assert!(store.inherit_shared(child, parent));
        assert_eq!(store.get(child), None);
        assert!(!store.inherit_shared(child, parent));
        assert!(!store.inherit_shared(explicit_child, parent));
        assert_eq!(store.get(explicit_child), Some(&30.0));
    }
}
