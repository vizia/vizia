//! The cache is a store for intermediate data produced while computing state, notably layout
//! results. The main type here is CachedData, usually accessed via `cx.cache`.

use crate::prelude::*;
use vizia_storage::SparseSet;
use vizia_storage::SparseSetError;

/// Computed properties used for layout and drawing.

#[derive(Clone, Copy, Debug)]
struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Default for Pos {
    fn default() -> Self {
        Pos { x: 0.0, y: 0.0 }
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Self {
        Pos { x: self.x + other.x, y: self.y + other.y }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct Size {
    pub width: f32,
    pub height: f32,
}

/// Stores data which can be cached between system runs.
///
/// When an event occurs or style data is changed systems run to determine the new state of the UI.
/// The output of these systems can be cached so that not all of the systems need to run again.
#[derive(Default)]
pub struct CachedData {
    pub(crate) bounds: SparseSet<BoundingBox>,
    pub(crate) abilities: SparseSet<Abilities>,
}

impl CachedData {
    pub fn add(&mut self, entity: Entity) -> Result<(), SparseSetError> {
        self.bounds.insert(entity, Default::default())?;
        self.abilities.insert(entity, Default::default())?;

        Ok(())
    }

    pub fn remove(&mut self, entity: Entity) {
        self.bounds.remove(entity);
        self.abilities.remove(entity);
    }

    /// Returns the bounding box of the entity, determined by the layout system.
    pub fn get_bounds(&self, entity: Entity) -> BoundingBox {
        BoundingBox {
            x: self.get_posx(entity),
            y: self.get_posy(entity),
            w: self.get_width(entity),
            h: self.get_height(entity),
        }
    }

    /// Returns the x position of the entity.
    pub fn get_posx(&self, entity: Entity) -> f32 {
        self.bounds.get(entity).cloned().unwrap_or_default().x
    }

    /// Returns the y position of the entity.
    pub fn get_posy(&self, entity: Entity) -> f32 {
        self.bounds.get(entity).cloned().unwrap_or_default().y
    }

    /// Returns the width of the entity.
    pub fn get_width(&self, entity: Entity) -> f32 {
        self.bounds.get(entity).cloned().unwrap_or_default().w
    }

    /// Returns the height of the entity.
    pub fn get_height(&self, entity: Entity) -> f32 {
        self.bounds.get(entity).cloned().unwrap_or_default().h
    }

    pub fn set_posx(&mut self, entity: Entity, val: f32) {
        if let Some(bounds) = self.bounds.get_mut(entity) {
            bounds.x = val;
        }
    }

    pub fn set_posy(&mut self, entity: Entity, val: f32) {
        if let Some(bounds) = self.bounds.get_mut(entity) {
            bounds.y = val;
        }
    }

    pub fn set_width(&mut self, entity: Entity, val: f32) {
        if let Some(bounds) = self.bounds.get_mut(entity) {
            bounds.w = val;
        }
    }

    pub fn set_height(&mut self, entity: Entity, val: f32) {
        if let Some(bounds) = self.bounds.get_mut(entity) {
            bounds.h = val;
        }
    }

    pub fn get_hoverability(&self, entity: Entity) -> bool {
        if let Some(abilities) = self.abilities.get(entity) {
            abilities.contains(Abilities::HOVERABLE)
        } else {
            false
        }
    }

    pub(crate) fn set_hoverability(&mut self, entity: Entity, val: bool) {
        if let Some(abilities) = self.abilities.get_mut(entity) {
            abilities.set(Abilities::HOVERABLE, val);
        }
    }
}
