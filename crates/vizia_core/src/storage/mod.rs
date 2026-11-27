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

                let mut store = <$store>::default();
                store.insert_rule(Rule::new(0, 0), 30.0);
                store.insert_rule(Rule::new(1, 0), 40.0);
                store.link(parent, &[(Rule::new(0, 0), 0)] $(, $variables)?);
                assert!(store.inherit_shared(child, parent));
                assert!(store.link(child, &[(Rule::new(0, 0), 0)] $(, $variables)?));
                assert!(!store.link(child, &[(Rule::new(0, 0), 0)] $(, $variables)?));
                store.link(parent, &[(Rule::new(1, 0), 0)] $(, $variables)?);
                assert!(!store.inherit_shared(child, parent));
                assert_eq!(store.get(child), Some(&30.0));
                store.link(parent, &[] $(, $variables)?);
                assert!(!store.inherit_shared(child, parent));
                assert_eq!(store.get(child), Some(&30.0));
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

    /// Plays an animation holding `value` on `entity`.
    fn play_held(
        store: &mut AnimatableVarSet<f32>,
        entity: Entity,
        animation: Animation,
        value: f32,
        start: Instant,
        duration: Duration,
    ) {
        use crate::animation::{AnimationState, Keyframe, TimingFunction};
        let keyframe = |time| Keyframe { time, value, timing_function: TimingFunction::linear() };
        store.insert_animation(
            animation,
            AnimationState::new(animation)
                .with_keyframe(keyframe(0.0))
                .with_keyframe(keyframe(1.0)),
        );
        store.play_animation(entity, animation, start, duration, Duration::ZERO);
    }

    /// Plays a 60 second animation at 10.0 on `parent`.
    fn play(store: &mut AnimatableVarSet<f32>, parent: Entity) {
        play_held(
            store,
            parent,
            Animation::new(0, 0),
            10.0,
            Instant::now(),
            Duration::from_secs(60),
        );
    }

    /// Ticks past the end of the animation, then once more to remove it.
    fn end_animations(store: &mut AnimatableVarSet<f32>) {
        let end = Instant::now() + Duration::from_secs(61);
        store.tick(end);
        store.tick(end);
    }

    #[test]
    fn shared_inheritance_points_descendants_at_an_ancestor_animation() {
        let parent = Entity::new(1, 0);
        let child = Entity::new(2, 0);
        let grandchild = Entity::new(3, 0);
        let explicit = Entity::new(4, 0);
        let mut store = AnimatableVarSet::<f32>::default();
        let variables = hashbrown::HashMap::new();
        store.insert_rule(Rule::new(0, 0), 30.0);
        store.insert_rule(Rule::new(1, 0), 40.0);
        store.link(parent, &[(Rule::new(0, 0), 0)], &variables);
        store.link(explicit, &[(Rule::new(1, 0), 0)], &variables);
        play(&mut store, parent);

        assert!(store.inherit_shared(child, parent));
        assert!(store.inherit_shared(grandchild, child));
        assert!(!store.inherit_shared(child, parent));
        assert!(!store.inherit_shared(explicit, parent));
        assert_eq!(store.get(child), Some(&10.0));
        assert_eq!(store.get_resolved(grandchild, &variables), Some(10.0));
        assert_eq!(store.get(explicit), Some(&40.0));

        end_animations(&mut store);
        assert_eq!(store.get(child), Some(&30.0));
        assert_eq!(store.get_resolved(grandchild, &variables), Some(30.0));
    }

    #[test]
    fn inline_inheritance_points_descendants_at_an_ancestor_animation() {
        let parent = Entity::new(1, 0);
        let child = Entity::new(2, 0);
        let grandchild = Entity::new(3, 0);
        let explicit = Entity::new(4, 0);
        let mut store = AnimatableVarSet::<f32>::default();
        store.insert(parent, 20.0);
        store.insert(explicit, 40.0);
        play(&mut store, parent);

        assert!(store.inherit_inline(child, parent));
        assert!(store.inherit_inline(grandchild, child));
        assert!(!store.inherit_inline(child, parent));
        assert!(!store.inherit_inline(explicit, parent));
        assert_eq!(store.get(child), Some(&10.0));
        assert_eq!(store.get(grandchild), Some(&10.0));
        assert_eq!(store.get(explicit), Some(&40.0));

        // A value set on the child itself replaces the one it inherited.
        store.insert(grandchild, 50.0);
        store.inherit_inline(grandchild, child);
        assert_eq!(store.get(grandchild), Some(&50.0));

        end_animations(&mut store);
        assert_eq!(store.get(child), Some(&20.0));
    }

    #[test]
    fn inline_inheritance_points_descendants_at_the_animation_of_a_parent_without_inline_data() {
        let parent = Entity::new(1, 0);
        let child = Entity::new(2, 0);
        let mut store = AnimatableVarSet::<f32>::default();
        play(&mut store, parent);
        assert!(store.inherit_inline(child, parent));
        assert_eq!(store.get(child), Some(&10.0));

        end_animations(&mut store);
        assert_eq!(store.get(child), None);
    }

    #[test]
    fn a_descendant_keeps_an_outer_animation_after_an_inner_one_ends() {
        let (grandparent, parent, child) =
            (Entity::new(1, 0), Entity::new(2, 0), Entity::new(3, 0));
        let mut store = AnimatableVarSet::<f32>::default();
        store.insert(grandparent, 20.0);
        let start = Instant::now();
        play_held(
            &mut store,
            grandparent,
            Animation::new(0, 0),
            10.0,
            start,
            Duration::from_secs(60),
        );
        store.inherit_inline(parent, grandparent);
        play_held(&mut store, parent, Animation::new(1, 0), 5.0, start, Duration::from_secs(1));
        store.inherit_inline(child, parent);
        assert_eq!(store.get(child), Some(&5.0));

        let later = start + Duration::from_secs(2);
        store.tick(later);
        let reported = store.tick(later);
        assert_eq!(store.get(parent), Some(&10.0));
        assert_eq!(store.get(child), Some(&10.0));
        // The tick reports the entity whose animation ended, as well as the one still animating.
        assert_eq!(reported, [parent, grandparent]);

        // Then the outer one ends too.
        let end = start + Duration::from_secs(61);
        store.tick(end);
        store.tick(end);
        assert_eq!(store.get(child), Some(&20.0));
        assert!(!store.inherit_inline(parent, grandparent));
        assert!(!store.inherit_inline(child, parent));
    }

    #[test]
    fn nested_animations_ending_together_leave_no_inherited_animation_behind() {
        // The child has the lowest index, so it is released before the parent it inherits through.
        let (child, parent, grandparent) =
            (Entity::new(1, 0), Entity::new(2, 0), Entity::new(3, 0));
        let mut store = AnimatableVarSet::<f32>::default();
        store.insert(grandparent, 20.0);
        let start = Instant::now();
        play_held(
            &mut store,
            grandparent,
            Animation::new(0, 0),
            10.0,
            start,
            Duration::from_secs(1),
        );
        store.inherit_inline(parent, grandparent);
        play_held(&mut store, parent, Animation::new(1, 0), 5.0, start, Duration::from_secs(1));
        store.inherit_inline(child, parent);

        let later = start + Duration::from_secs(2);
        store.tick(later);
        store.tick(later);
        assert_eq!(store.get(child), Some(&20.0));
        assert!(!store.inherit_inline(parent, grandparent));
        assert!(!store.inherit_inline(child, parent));
    }

    #[test]
    fn restarting_an_animation_keeps_the_descendants_inheriting_it() {
        let (parent, child) = (Entity::new(1, 0), Entity::new(2, 0));
        let mut store = AnimatableVarSet::<f32>::default();
        store.insert(parent, 20.0);
        let start = Instant::now();
        let animation = Animation::new(0, 0);
        play_held(&mut store, parent, animation, 10.0, start, Duration::from_secs(1));
        store.inherit_inline(child, parent);
        // Playing it again keeps the first state, which ends first.
        store.play_animation(parent, animation, start, Duration::from_secs(60), Duration::ZERO);

        let later = start + Duration::from_secs(2);
        store.tick(later);
        store.tick(later);
        assert_eq!(store.get(parent), Some(&10.0));
        assert_eq!(store.get(child), Some(&10.0));
    }

    #[test]
    fn removing_an_inline_value_keeps_the_animation_of_the_swapped_entity() {
        let (removed, parent, child) = (Entity::new(1, 0), Entity::new(2, 0), Entity::new(3, 0));
        let mut store = AnimatableVarSet::<f32>::default();
        store.insert(removed, 30.0);
        store.insert(parent, 20.0);
        play(&mut store, parent);
        store.inherit_inline(child, parent);
        assert_eq!(store.get(child), Some(&10.0));
        store.remove(removed);
        store.inherit_inline(child, parent);
        store.tick(Instant::now());
        assert_eq!(store.get(parent), Some(&10.0));
        assert_eq!(store.get(child), Some(&10.0));
    }

    #[test]
    fn an_inline_value_on_an_animating_parent_keeps_its_descendants_animating() {
        let (parent, child) = (Entity::new(1, 0), Entity::new(2, 0));
        let mut store = AnimatableVarSet::<f32>::default();
        play(&mut store, parent);
        store.inherit_inline(child, parent);
        store.insert(parent, 20.0);
        store.inherit_inline(child, parent);
        store.tick(Instant::now());
        assert_eq!(store.get(parent), Some(&10.0));
        assert_eq!(store.get(child), Some(&10.0));
    }

    #[test]
    fn removing_an_animating_entity_leaves_finished_animations_to_the_next_tick() {
        let (removed, ended, child) = (Entity::new(1, 0), Entity::new(2, 0), Entity::new(3, 0));
        let mut store = AnimatableVarSet::<f32>::default();
        let start = Instant::now();
        store.insert(ended, 20.0);
        play_held(&mut store, ended, Animation::new(0, 0), 10.0, start, Duration::from_secs(1));
        store.inherit_inline(child, ended);
        play_held(&mut store, removed, Animation::new(1, 0), 5.0, start, Duration::from_secs(60));
        let later = start + Duration::from_secs(2);
        store.tick(later);

        // Removing an entity ends its own animation, and the next tick releases the animations
        // that had already finished, reporting them but not the removed entity.
        store.remove(removed);
        assert_eq!(store.get(child), Some(&10.0));
        assert_eq!(store.tick(later), [ended]);
        assert_eq!(store.get(child), Some(&20.0));
    }

    #[test]
    fn stopping_an_animation_releases_the_descendants_inheriting_it() {
        let (parent, child) = (Entity::new(1, 0), Entity::new(2, 0));
        let mut store = AnimatableVarSet::<f32>::default();
        store.insert(parent, 20.0);
        play(&mut store, parent);
        store.inherit_inline(child, parent);
        assert_eq!(store.get(child), Some(&10.0));

        store.stop_animation(parent, Animation::new(0, 0));
        assert_eq!(store.get(child), Some(&20.0));
        assert!(!store.inherit_inline(child, parent));
    }

    #[test]
    fn stopping_another_animation_keeps_the_one_playing() {
        let (parent, child) = (Entity::new(1, 0), Entity::new(2, 0));
        let mut store = AnimatableVarSet::<f32>::default();
        store.insert(parent, 20.0);
        play(&mut store, parent);
        store.inherit_inline(child, parent);

        store.stop_animation(parent, Animation::new(1, 0));
        assert_eq!(store.get(parent), Some(&10.0));
        assert_eq!(store.get(child), Some(&10.0));
    }

    #[test]
    fn removing_an_entity_with_a_restarted_animation_drops_it_from_both_states() {
        let (removed, other) = (Entity::new(1, 0), Entity::new(2, 0));
        let mut store = AnimatableVarSet::<f32>::default();
        let start = Instant::now();
        let animation = Animation::new(0, 0);
        play_held(&mut store, removed, animation, 10.0, start, Duration::from_secs(1));
        // Playing it again keeps the first state, which still lists the entity.
        store.play_animation(removed, animation, start, Duration::from_secs(60), Duration::ZERO);
        play_held(&mut store, other, Animation::new(1, 0), 5.0, start, Duration::from_secs(60));

        store.remove(removed);
        let later = start + Duration::from_secs(2);
        store.tick(later);
        assert_eq!(store.tick(later), [other]);
    }

    #[test]
    fn a_rule_on_an_inheriting_descendant_replaces_an_ancestor_animation() {
        let (parent, child) = (Entity::new(1, 0), Entity::new(2, 0));
        let mut store = AnimatableVarSet::<f32>::default();
        let variables = hashbrown::HashMap::new();
        store.insert_rule(Rule::new(0, 0), 30.0);
        store.insert_rule(Rule::new(1, 0), 40.0);
        store.link(parent, &[(Rule::new(0, 0), 0)], &variables);
        play(&mut store, parent);
        assert!(store.inherit_shared(child, parent));
        assert_eq!(store.get(child), Some(&10.0));
        store.link(child, &[(Rule::new(1, 0), 0)], &variables);
        assert!(store.inherit_shared(child, parent));
        assert_eq!(store.get(child), Some(&40.0));
    }

    #[test]
    fn resolved_variable_link_promotes_an_inherited_rule_to_explicit() {
        let mut store = AnimatableVarSet::<f32>::default();
        let parent = Entity::new(1, 0);
        let child = Entity::new(2, 0);
        let variables = hashbrown::HashMap::new();
        store.insert_rule(Rule::new(0, 0), 30.0);
        store.insert_rule(Rule::new(1, 0), 40.0);
        store.link_with_resolved(parent, &[(Rule::new(0, 0), 0)], &variables);
        assert!(store.inherit_shared(child, parent));
        assert!(store.link_with_resolved(child, &[(Rule::new(0, 0), 0)], &variables));
        assert!(!store.link_with_resolved(child, &[(Rule::new(0, 0), 0)], &variables));
        store.link_with_resolved(parent, &[(Rule::new(1, 0), 0)], &variables);
        assert!(!store.inherit_shared(child, parent));
        assert_eq!(store.get(child), Some(&30.0));
    }
}
