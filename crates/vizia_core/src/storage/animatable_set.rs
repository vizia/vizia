use crate::animation::{Animation, AnimationState, Interpolator};
use crate::prelude::*;
use vizia_storage::{SparseSet, SparseSetGeneric, SparseSetIndex};
use vizia_style::{AnimationFillMode, AnimationIterationCount};

const INDEX_MASK: u32 = u32::MAX / 4;
const INLINE_MASK: u32 = 1 << 31;
const INHERITED_MASK: u32 = 1 << 30;

/// Represents an index that can either be used to retrieve inline or shared data
///
/// Since inline data will override shared data, this allows the same index to be used
/// with a flag to indicate which data the index refers to.
#[derive(Clone, Copy, PartialEq)]
struct DataIndex(u32);

impl DataIndex {
    /// Create a new data index with the first bit set to 1, indicating that
    /// the index refers to inline data.
    pub fn inline(index: usize) -> Self {
        assert!((index as u32) < INDEX_MASK);
        let value = (index as u32) | INLINE_MASK;
        Self(value)
    }

    pub fn inherited(self) -> Self {
        let value = self.0;
        Self(value | INHERITED_MASK)
    }

    /// Create a new data index with the first bit set to 0, indicating that
    /// the index refers to shared data.
    pub fn shared(index: usize) -> Self {
        assert!((index as u32) < INDEX_MASK);
        Self(index as u32)
    }

    /// Retrieve the inline or shared data index.
    pub fn index(&self) -> usize {
        (self.0 & INDEX_MASK) as usize
    }

    /// Returns true if the data index refers to inline data.
    pub fn is_inline(&self) -> bool {
        (self.0 & INLINE_MASK).rotate_left(1) != 0
    }

    /// Returns true if the data index refers to an inherited value
    pub fn is_inherited(&self) -> bool {
        (self.0 & INHERITED_MASK).rotate_left(2) != 0
    }

    /// Create a null data index.
    ///
    /// A null data index is used to signify that the index refers to no data.
    pub fn null() -> Self {
        Self(u32::MAX >> 1)
    }
}

impl std::fmt::Debug for DataIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_inline() {
            write!(f, "Inline: {}", self.index())
        } else {
            write!(f, "Shared: {}", self.index())
        }
    }
}

/// An Index is used by the AnimatableSet and contains a data index and an animation index.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct InlineIndex {
    data_index: DataIndex,
    anim_index: u32,
}

impl Default for InlineIndex {
    fn default() -> Self {
        InlineIndex { data_index: DataIndex::null(), anim_index: u32::MAX }
    }
}

impl SparseSetIndex for InlineIndex {
    fn new(index: usize) -> Self {
        InlineIndex { data_index: DataIndex::inline(index), anim_index: u32::MAX }
    }

    fn null() -> Self {
        Self::default()
    }

    fn index(&self) -> usize {
        self.data_index.index()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SharedIndex {
    data_index: u32,
    animation: AnimationId,
}

impl Default for SharedIndex {
    fn default() -> Self {
        SharedIndex { data_index: u32::MAX, animation: AnimationId::null() }
    }
}

impl SparseSetIndex for SharedIndex {
    fn new(index: usize) -> Self {
        SharedIndex { data_index: index as u32, animation: AnimationId::null() }
    }

    fn null() -> Self {
        Self::default()
    }

    fn index(&self) -> usize {
        self.data_index as usize
    }
}

/// Animatable set is used for storing inline and shared data for entities as well as definitions for
/// animations, which can be played for entities, and transitions, which play when an entity matches a new shared style
/// rule which defines a trnasition.
///
/// Animations are moved from animations to active_animations when played. This allows the active
/// animations to be quickly iterated to update the value.
#[derive(Default, Debug)]
pub(crate) struct AnimatableSet<T: Interpolator> {
    /// Shared data determined by style rules
    pub(crate) shared_data: SparseSetGeneric<SharedIndex, T>,
    /// Inline data defined on specific entities
    pub(crate) inline_data: SparseSetGeneric<InlineIndex, T>,
    /// Animation descriptions
    animations: SparseSet<AnimationState<T>>,
    /// Animations which are currently playing
    active_animations: Vec<AnimationState<T>>,
}

impl<T> AnimatableSet<T>
where
    T: 'static + Default + Clone + Interpolator + PartialEq + std::fmt::Debug,
{
    /// Insert an inline value for an entity.
    pub fn insert(&mut self, entity: Entity, value: T) {
        self.inline_data.insert(entity, value);
    }

    /// Remove an entity and any inline data.
    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let entity_index = entity.index();

        if entity_index < self.inline_data.sparse.len() {
            let active_anim_index = self.inline_data.sparse[entity_index].anim_index as usize;

            if active_anim_index < self.active_animations.len() {
                let anim_state = &mut self.active_animations[active_anim_index];
                anim_state.t = 1.0;

                self.remove_innactive_animations();
            }

            let data_index = self.inline_data.sparse[entity_index].data_index;
            if data_index.is_inline() && !data_index.is_inherited() {
                self.inline_data.remove(entity)
            } else {
                self.inline_data.sparse[entity_index] = InlineIndex::null();
                None
            }
        } else {
            None
        }
    }

    /// Inherit inline data from a parent entity.
    pub fn inherit_inline(&mut self, entity: Entity, parent: Entity) -> bool {
        let entity_index = entity.index();
        let parent_index = parent.index();

        if parent_index < self.inline_data.sparse.len() {
            let parent_sparse_index = self.inline_data.sparse[parent_index];

            if parent_sparse_index.data_index.is_inline()
                && parent_sparse_index.data_index.index() < self.inline_data.dense.len()
            {
                if entity_index >= self.inline_data.sparse.len() {
                    self.inline_data.sparse.resize(entity_index + 1, InlineIndex::null());
                }

                let entity_sparse_index = self.inline_data.sparse[entity_index];

                if self.inline_data.sparse[entity_index].data_index.index()
                    != parent_sparse_index.data_index.index()
                {
                    if entity_sparse_index.data_index.index() < self.inline_data.dense.len() {
                        if entity_sparse_index.data_index.is_inherited()
                            && entity_sparse_index.data_index.is_inline()
                        {
                            self.inline_data.sparse[entity_index] = InlineIndex {
                                data_index: DataIndex::inline(
                                    parent_sparse_index.data_index.index(),
                                )
                                .inherited(),
                                anim_index: u32::MAX,
                            };
                            return true;
                        }
                    } else {
                        self.inline_data.sparse[entity_index] = InlineIndex {
                            data_index: DataIndex::inline(parent_sparse_index.data_index.index())
                                .inherited(),
                            anim_index: u32::MAX,
                        };
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Inherit shared data from a parent entity.
    pub fn inherit_shared(&mut self, entity: Entity, parent: Entity) -> bool {
        let entity_index = entity.index();
        let parent_index = parent.index();

        if parent_index < self.inline_data.sparse.len() {
            let parent_sparse_index = self.inline_data.sparse[parent_index];

            if !parent_sparse_index.data_index.is_inline()
                && parent_sparse_index.data_index.index() < self.shared_data.dense.len()
            {
                if entity_index >= self.inline_data.sparse.len() {
                    self.inline_data.sparse.resize(entity_index + 1, InlineIndex::null());
                }

                let entity_sparse_index = self.inline_data.sparse[entity_index];

                if !entity_sparse_index.data_index.is_inline()
                    && self.inline_data.sparse[entity_index].data_index.index()
                        != parent_sparse_index.data_index.index()
                {
                    if entity_sparse_index.data_index.index() < self.shared_data.dense.len() {
                        if entity_sparse_index.data_index.is_inherited() {
                            self.inline_data.sparse[entity_index] = InlineIndex {
                                data_index: DataIndex::shared(
                                    parent_sparse_index.data_index.index(),
                                )
                                .inherited(),
                                anim_index: u32::MAX,
                            };
                            return true;
                        }
                    } else {
                        if !entity_sparse_index.data_index.is_inline() {
                            self.inline_data.sparse[entity_index] = InlineIndex {
                                data_index: DataIndex::shared(
                                    parent_sparse_index.data_index.index(),
                                )
                                .inherited(),
                                anim_index: u32::MAX,
                            };
                        }
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Inserts an animation
    ///
    /// Animations exist separately to inline (entity) data and shared (rule) data.
    /// Playing an aimation for a particular entity will clone the animation state to the
    /// active animations and then link the entity to it.
    pub(crate) fn insert_animation(
        &mut self,
        animation: AnimationId,
        animation_description: AnimationState<T>,
    ) {
        self.animations.insert(animation, animation_description);
    }

    pub(crate) fn insert_rule(&mut self, rule: Rule, value: T) {
        self.shared_data.insert(rule, value);
    }

    // pub(crate) fn remove_rule(&mut self, rule: Rule) -> Option<T> {
    //     self.shared_data.remove(rule)
    // }

    /// Inserts a transition for a given rule
    ///
    /// Transitions are animations which are defined for a particular rule. When an entity is linked to
    /// a rule any transition associated with that rule will play for that entity.
    ///
    pub(crate) fn insert_transition(&mut self, rule: Rule, animation: AnimationId) {
        // Check if the rule exists
        if self.shared_data.contains(rule) && self.animations.contains(animation) {
            self.shared_data.sparse[rule.index()].animation = animation;
        }
    }

    /// Play an animation for a given entity.
    pub(crate) fn play_animation(
        &mut self,
        entity: Entity,
        animation_id: AnimationId,
        animation: Animation,
        start_time: Instant,
    ) {
        // Early return if animation doesn't exist
        if !self.animations.contains(animation_id) {
            return;
        }

        let entity_index = entity.index();

        // Ensure we have space for this entity
        if entity_index >= self.inline_data.sparse.len() {
            self.inline_data.sparse.resize(entity_index + 1, InlineIndex::null());
        }

        // Check if entity is already animating
        let active_anim_index = self.inline_data.sparse[entity_index].anim_index as usize;
        if active_anim_index < self.active_animations.len() {
            let anim_state = &mut self.active_animations[active_anim_index];

            // If same animation is already playing, just update its parameters
            if anim_state.id == animation_id {
                anim_state.active = true;
                anim_state.t = 0.0;
                anim_state.start_time = start_time;
                anim_state.iteration_count = animation.iteration_count;
                anim_state.fill_mode = animation.fill_mode;
                anim_state.direction = animation.direction;
                anim_state.output = None;
                return; // We're done, no need to create new animation
            } else {
                // Remove entity from previous animation
                anim_state.output = None;
                anim_state.entities.remove(&entity);
            }
        }

        // Create new animation state from template
        let mut anim_state = self.animations.get(animation_id).cloned().unwrap();

        // Set animation properties
        anim_state.id = animation_id;
        anim_state.duration = animation.duration;
        anim_state.delay = animation.delay;
        anim_state.dt = animation.delay.as_secs_f32() / animation.duration.as_secs_f32();
        anim_state.iteration_count = animation.iteration_count;
        anim_state.current_iteration = 0;
        anim_state.fill_mode = animation.fill_mode;
        anim_state.direction = animation.direction;
        anim_state.output = None;

        // Link entity to this animation
        anim_state.play(entity);

        // Store animation and update entity's reference to it
        self.inline_data.sparse[entity_index].anim_index = self.active_animations.len() as u32;
        self.active_animations.push(anim_state);
    }

    /// Stop an animation for a given entity.
    pub(crate) fn stop_animation(&mut self, entity: Entity, animation_id: AnimationId) {
        let entity_index = entity.index();

        if entity_index < self.inline_data.sparse.len() {
            let active_anim_index = self.inline_data.sparse[entity_index].anim_index as usize;
            if active_anim_index < self.active_animations.len() {
                let anim_state = &mut self.active_animations[active_anim_index];
                if anim_state.id == animation_id {
                    anim_state.entities.remove(&entity);
                }
            }
            self.inline_data.sparse[entity_index].anim_index = u32::MAX;
        }
    }

    /// Tick the animation for the given time and return a list of entities which have been animated.
    pub fn tick(&mut self, time: Instant) -> Vec<Entity> {
        // Collect of affected entities.
        let entities =
            self.active_animations.iter().flat_map(|state| state.entities.clone()).collect();

        self.remove_innactive_animations();

        if !self.has_animations() {
            return entities;
        }

        for state in self.active_animations.iter_mut() {
            // Skip completed non-infinite animations.
            if state.t >= 1.0 && !state.iteration_count.is_infinite() {
                continue;
            }

            // Fast path for single-keyframe animations.
            if state.keyframes.len() <= 1 {
                if let Some(keyframe) = state.keyframes.first() {
                    state.output = Some(keyframe.value.clone());
                }
                continue;
            }

            // Calculate timing values.
            let elapsed_time = time.duration_since(state.start_time);
            let elapsed_secs = elapsed_time.as_secs_f32();
            let duration_secs = state.duration.as_secs_f32();
            let mut progress = elapsed_secs / duration_secs;

            // Handle delay phase.
            if progress < state.dt {
                // Apply backwards fill mode during delay.
                if matches!(state.fill_mode, AnimationFillMode::Backwards | AnimationFillMode::Both)
                {
                    state.output = match state.direction {
                        AnimationDirection::Normal | AnimationDirection::Alternate => {
                            state.keyframes.first().map(|k| k.value.clone())
                        }
                        AnimationDirection::Reverse | AnimationDirection::AlternateReverse => {
                            state.keyframes.last().map(|k| k.value.clone())
                        }
                    };
                }
                continue;
            }

            // Adjust for delay.
            let adjusted_secs = elapsed_secs - state.delay.as_secs_f32();
            progress = (adjusted_secs / duration_secs).clamp(0.0, f32::MAX);

            // Calculate iteration info.
            let raw_iteration = progress.floor();
            let current_iteration = raw_iteration as usize;
            let iteration_progress = progress - raw_iteration;

            // Check if animation should complete.
            let is_final_iteration = match state.iteration_count {
                AnimationIterationCount::Count(count) => current_iteration >= count as usize,
                AnimationIterationCount::Infinite => false,
            };

            if is_final_iteration {
                // Mark animation as complete.
                state.t = 1.0;
                state.current_iteration = state.iteration_count.to_count() - 1;

                // Apply fill mode with direction awareness.
                if matches!(state.fill_mode, AnimationFillMode::Forwards | AnimationFillMode::Both)
                {
                    let is_even_iteration = state.current_iteration % 2 == 0;

                    // Determine final keyframe based on direction and iteration count.
                    let final_value = match state.direction {
                        AnimationDirection::Normal => {
                            state.keyframes.last().map(|k| k.value.clone())
                        }

                        AnimationDirection::Reverse => {
                            state.keyframes.first().map(|k| k.value.clone())
                        }

                        AnimationDirection::Alternate => {
                            if is_even_iteration {
                                state.keyframes.last().map(|k| k.value.clone())
                            } else {
                                state.keyframes.first().map(|k| k.value.clone())
                            }
                        }

                        AnimationDirection::AlternateReverse => {
                            if is_even_iteration {
                                state.keyframes.first().map(|k| k.value.clone())
                            } else {
                                state.keyframes.last().map(|k| k.value.clone())
                            }
                        }
                    };

                    state.output = final_value;
                }

                continue;
            }

            // Calculate direction-adjusted progress for this iteration.
            let direction_adjusted_progress = match state.direction {
                AnimationDirection::Normal => iteration_progress,
                AnimationDirection::Reverse => 1.0 - iteration_progress,
                AnimationDirection::Alternate => {
                    if current_iteration % 2 == 0 {
                        iteration_progress
                    } else {
                        1.0 - iteration_progress
                    }
                }
                AnimationDirection::AlternateReverse => {
                    if current_iteration % 2 == 0 {
                        1.0 - iteration_progress
                    } else {
                        iteration_progress
                    }
                }
            };

            state.t = direction_adjusted_progress;
            state.current_iteration = current_iteration as u32;

            // Find keyframes to interpolate between.
            let keyframe_index = if state.keyframes.len() > 8 {
                // Binary search for larger keyframe collections.
                match state.keyframes.binary_search_by(|k| {
                    k.time
                        .partial_cmp(&direction_adjusted_progress)
                        .unwrap_or(std::cmp::Ordering::Equal)
                }) {
                    Ok(exact) => exact.saturating_sub(1), // Exact match, use previous
                    Err(insertion) => insertion.saturating_sub(1).min(state.keyframes.len() - 2), // Use previous frame
                }
            } else {
                // Linear search for small keyframe collections
                state
                    .keyframes
                    .windows(2)
                    .position(|w| w[1].time >= direction_adjusted_progress)
                    .unwrap_or(state.keyframes.len() - 2)
            };

            // Calculate interpolation.
            let (from, to) =
                (&state.keyframes[keyframe_index], &state.keyframes[keyframe_index + 1]);
            let keyframe_progress = if to.time != from.time {
                (direction_adjusted_progress - from.time) / (to.time - from.time)
            } else {
                0.0
            };

            // Apply easing and interpolate.
            let easing = from.timing_function.unwrap_or(state.easing_function);
            let eased_progress = easing.value(keyframe_progress);

            // Set output value through interpolation.
            state.output = Some(T::interpolate(&from.value, &to.value, eased_progress));

            // Handle wrap-around for repeating animations.
            if progress - raw_iteration >= 1.0 {
                match state.iteration_count {
                    AnimationIterationCount::Count(count) => {
                        if current_iteration < count as usize - 1 {
                            // Next iteration.
                            state.start_time = time;
                        }
                    }
                    AnimationIterationCount::Infinite => {
                        // Reset for next iteration.
                        state.start_time = time;
                    }
                }
            }
        }

        entities
    }

    // Returns true if the given entity is linked to an active animation
    // pub fn is_animating(&self, entity: Entity) -> bool {
    //     let entity_index = entity.index();
    //     if entity_index < self.inline_data.sparse.len() {
    //         let anim_index = self.inline_data.sparse[entity_index].anim_index as usize;
    //         if anim_index < self.active_animations.len() {
    //             return true;
    //         }
    //     }

    //     false
    // }

    /// Remove any inactive animations from the active animations list.
    pub fn remove_innactive_animations(&mut self) {
        // Create a list of finished animations
        let inactive: Vec<AnimationState<T>> = self
            .active_animations
            .iter()
            .filter(|e| e.t >= 1.0 && !e.should_persist())
            .cloned()
            .collect();

        // Remove inactive animation states from active animations list
        // Retains persistent animations
        self.active_animations.retain(|e| e.t < 1.0 || e.should_persist());

        for state in inactive.into_iter() {
            for entity in state.entities.iter() {
                self.inline_data.sparse[entity.index()].anim_index = u32::MAX;
            }
        }

        for (index, state) in self.active_animations.iter().enumerate() {
            for entity in state.entities.iter() {
                self.inline_data.sparse[entity.index()].anim_index = index as u32;
            }
        }
    }

    /// Returns true if there are any active animations.
    pub fn has_animations(&self) -> bool {
        for state in self.active_animations.iter() {
            if state.t < 1.0 {
                return true;
            }
        }

        false
    }

    /// Returns true if the given entity is linked to an active animation.
    pub fn has_active_animation(&self, entity: Entity, animation_id: AnimationId) -> bool {
        let entity_index = entity.index();
        if entity_index < self.inline_data.sparse.len() {
            let anim_index = self.inline_data.sparse[entity_index].anim_index as usize;
            if anim_index < self.active_animations.len()
                && self.active_animations[anim_index].id == animation_id
            {
                return true;
            }
        }

        false
    }

    // Returns a reference to any inline data on the entity if it exists.
    // pub fn get_inline(&self, entity: Entity) -> Option<&T> {
    //     let entity_index = entity.index();
    //     if entity_index < self.inline_data.sparse.len() {
    //         let data_index = self.inline_data.sparse[entity_index].data_index;
    //         if data_index.is_inline() {
    //             return self.inline_data.get(entity);
    //         }
    //     }

    //     None
    // }

    /// Returns a mutable reference to any inline data on the entity if it exists.
    pub fn get_inline_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let entity_index = entity.index();
        if entity_index < self.inline_data.sparse.len() {
            let data_index = self.inline_data.sparse[entity_index].data_index;
            if data_index.is_inline() {
                return self.inline_data.get_mut(entity);
            }
        }

        None
    }

    // /// Returns a reference to any shared data for a given rule if it exists.
    // pub(crate) fn get_shared(&self, rule: Rule) -> Option<&T> {
    //     self.shared_data.get(rule)
    // }

    // /// Returns a mutable reference to any shared data for a given rule if it exists.
    // pub(crate) fn get_shared_mut(&mut self, rule: Rule) -> Option<&mut T> {
    //     self.shared_data.get_mut(rule)
    // }

    pub(crate) fn get_animation_mut(
        &mut self,
        animation_id: AnimationId,
    ) -> Option<&mut AnimationState<T>> {
        self.animations.get_mut(animation_id)
    }

    /// Returns a reference to the active animation linked to the given entity if it exists,
    /// else returns None.
    pub(crate) fn get_active_animation(&self, entity: Entity) -> Option<&AnimationState<T>> {
        let entity_index = entity.index();
        if entity_index < self.inline_data.sparse.len() {
            let anim_index = self.inline_data.sparse[entity_index].anim_index as usize;
            if anim_index < self.active_animations.len() {
                return Some(&self.active_animations[anim_index]);
            }
        }

        None
    }

    /// Returns a reference to the active animations.
    pub(crate) fn get_active_animations(&mut self) -> Option<&Vec<AnimationState<T>>> {
        Some(&self.active_animations)
    }

    /// Get the animated, inline, or shared data value from the storage.
    pub fn get(&self, entity: Entity) -> Option<&T> {
        let entity_index = entity.index();
        if entity_index < self.inline_data.sparse.len() {
            // Animations override inline and shared styling
            let animation_index = self.inline_data.sparse[entity_index].anim_index as usize;

            if animation_index < self.active_animations.len()
                && self.active_animations[animation_index].output.is_some()
            {
                return self.active_animations[animation_index].get_output();
            }

            let data_index = self.inline_data.sparse[entity_index].data_index;
            if data_index.is_inline() {
                if data_index.index() < self.inline_data.dense.len() {
                    return Some(&self.inline_data.dense[data_index.index()].value);
                }
            } else if data_index.index() < self.shared_data.dense.len() {
                return Some(&self.shared_data.dense[data_index.index()].value);
            }
        }

        None
    }

    /// Link an entity to some shared data.
    pub(crate) fn link(&mut self, entity: Entity, rules: &[(Rule, u32)]) -> bool {
        let entity_index = entity.index();

        // Check if the entity already has some data
        if entity_index < self.inline_data.sparse.len() {
            let data_index = self.inline_data.sparse[entity_index].data_index;
            // If the data is inline then skip linking as inline data overrides shared data
            if data_index.is_inline() && !data_index.is_inherited() {
                return false;
            }
        }

        // Loop through matched rules and link to the first valid rule
        for (rule, _) in rules {
            if let Some(shared_data_index) = self.shared_data.dense_idx(*rule) {
                // If the entity doesn't have any previous shared data then create space for it
                if entity_index >= self.inline_data.sparse.len() {
                    self.inline_data.sparse.resize(entity_index + 1, InlineIndex::null());
                }

                // Get the animation state index of any animations (transitions) defined for the rule
                let rule_animation = shared_data_index.animation;

                //if let Some(transition_state) = self.animations.get_mut(rule_animation) {
                let entity_anim_index = self.inline_data.sparse[entity_index].anim_index as usize;
                if entity_anim_index < self.active_animations.len() {
                    // Already animating
                    let current_value = self.get(entity).cloned().unwrap_or_default();
                    let current_anim_state = &mut self.active_animations[entity_anim_index];
                    let rule_data_index = shared_data_index.data_index as usize;

                    if current_anim_state.is_transition() {
                        // Skip if the transition hasn't changed
                        if current_anim_state.to_rule != rule_data_index {
                            if rule_data_index == current_anim_state.from_rule {
                                // Transitioning back to previous rule
                                current_anim_state.from_rule = current_anim_state.to_rule;
                                current_anim_state.to_rule = rule_data_index;
                                current_anim_state.keyframes.first_mut().unwrap().value =
                                    self.shared_data.dense[current_anim_state.from_rule]
                                        .value
                                        .clone();

                                current_anim_state.keyframes.last_mut().unwrap().value =
                                    self.shared_data.dense[current_anim_state.to_rule]
                                        .value
                                        .clone();

                                current_anim_state.dt = current_anim_state.t - 1.0;
                                current_anim_state.start_time = Instant::now();
                            } else {
                                // Transitioning to new rule
                                current_anim_state.to_rule = rule_data_index;
                                current_anim_state.keyframes.first_mut().unwrap().value =
                                    current_value;
                                current_anim_state.keyframes.last_mut().unwrap().value =
                                    self.shared_data.dense[current_anim_state.to_rule]
                                        .value
                                        .clone();
                                current_anim_state.t = 0.0;
                                current_anim_state.start_time = Instant::now();
                            }
                        }
                    }
                } else if let Some(transition_state) = self.animations.get_mut(rule_animation) {
                    // Safe to unwrap because already checked that the rule exists
                    let end = self.shared_data.get(*rule).unwrap();

                    let entity_data_index = self.inline_data.sparse[entity_index].data_index;

                    if !entity_data_index.is_inline()
                        && entity_data_index.index() < self.shared_data.dense.len()
                    {
                        let start_data =
                            self.shared_data.dense[entity_data_index.index()].value.clone();
                        transition_state.keyframes.first_mut().unwrap().value = start_data;
                    } else {
                        transition_state.keyframes.first_mut().unwrap().value = end.clone();
                    }

                    transition_state.keyframes.last_mut().unwrap().value = end.clone();
                    transition_state.from_rule =
                        self.inline_data.sparse[entity_index].data_index.index();
                    transition_state.to_rule = shared_data_index.index();

                    let duration = transition_state.duration;
                    let delay = transition_state.delay;

                    if transition_state.from_rule != DataIndex::null().index()
                        && transition_state.from_rule != transition_state.to_rule
                    {
                        self.play_animation(
                            entity,
                            rule_animation,
                            Animation { duration, delay, ..Default::default() },
                            Instant::now() + delay,
                        );
                    }
                    //}
                }
                //}

                let data_index = self.inline_data.sparse[entity_index].data_index;

                // Already linked
                if !data_index.is_inline() && data_index.index() == shared_data_index.index() {
                    return false;
                }

                self.inline_data.sparse[entity_index].data_index =
                    DataIndex::shared(shared_data_index.index());

                return true;
            }
        }

        // No matching rules so set if the data is shared set the index to null if not already null
        if entity_index < self.inline_data.sparse.len() {
            let data_index = self.inline_data.sparse[entity_index].data_index;
            if !data_index.is_inline()
                && !data_index.is_inherited()
                && self.inline_data.sparse[entity_index].data_index != DataIndex::null()
            {
                self.inline_data.sparse[entity_index].data_index = DataIndex::null();
                return true;
            }
        }

        false
    }

    /// Clear all rules and animations from the storage.
    pub fn clear_rules(&mut self) {
        // Remove transitions
        for index in self.shared_data.sparse.iter() {
            let animation = index.animation;
            self.animations.remove(animation);
        }

        self.shared_data.clear();

        for index in self.inline_data.sparse.iter_mut() {
            if !index.data_index.is_inline() {
                index.data_index = DataIndex::null();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // DataIndex tests

    /// Test for creating an inline data index and retrieving the index.
    #[test]
    fn inline() {
        let data_index = DataIndex::inline(5);
        assert_eq!(data_index.0, INLINE_MASK + 5);
        assert_eq!(data_index.index(), 5);
    }

    /// Test that an invalid (too large) inline index causes a panic.
    #[test]
    #[should_panic]
    fn invalid_inline() {
        DataIndex::inline(usize::MAX);
    }

    /// Test for creating a shared data index and retrieving the index.
    #[test]
    fn shared() {
        let data_index = DataIndex::shared(5);
        assert_eq!(data_index.0, 5);
        assert_eq!(data_index.index(), 5);
    }

    /// Test that an invalid (too large) shared index causes a panic.
    #[test]
    #[should_panic]
    fn invalid_shared() {
        DataIndex::shared(usize::MAX);
    }

    /// Test of the is_inline() method.
    #[test]
    fn is_inline() {
        let data_index1 = DataIndex::inline(5);
        assert!(data_index1.is_inline());
        let data_index2 = DataIndex::shared(5);
        assert!(!data_index2.is_inline());
    }

    /// Test that a null data index is the correct value #7FFFFFFF (i.e. all bits = 1 except the first bit).
    #[test]
    fn null() {
        let data_index = DataIndex::null();
        assert_eq!(data_index.0, 2147483647);
    }

    // AnimatableStorage tests

    /// Test for constructing a new empty animatable storage.
    #[test]
    fn new() {
        let animatable_storage = AnimatableSet::<f32>::default();
        assert!(animatable_storage.inline_data.is_empty());
        assert!(animatable_storage.shared_data.is_empty());
        assert!(animatable_storage.animations.is_empty());
        assert!(animatable_storage.active_animations.is_empty());
    }

    /// Test inserting inline data into the storage.
    #[test]
    fn insert_inline() {
        let mut animatable_storage = AnimatableSet::default();
        animatable_storage.insert(Entity::root(), 5.0);
        //assert_eq!(animatable_storage.entity_indices.first().unwrap().data_index, DataIndex::inline(0));
    }
}
