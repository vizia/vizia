//! The cache is a store for intermediate data produced while computing state, notably layout
//! results. The main type here is CachedData, usually accessed via `cx.cache`.

use crate::prelude::*;
use skia_safe::{Matrix, Path};
use vizia_storage::SparseSet;

/// Stores data which can be cached between system runs.
///
/// When an event occurs or style data is changed systems run to determine the new state of the UI.
/// The output of these systems can be cached so that not all of the systems need to run again.
#[derive(Default)]
pub struct CachedData {
    pub(crate) bounds: SparseSet<BoundingBox>,
    pub(crate) draw_bounds: SparseSet<BoundingBox>,
    pub(crate) relative_bounds: SparseSet<BoundingBox>,
    pub(crate) geo_changed: SparseSet<GeoChanged>,
    pub(crate) transform: SparseSet<Matrix>,
    pub(crate) clip_path: SparseSet<skia_safe::Path>,
    pub(crate) path: SparseSet<Path>,
}

impl CachedData {
    pub(crate) fn add(&mut self, entity: Entity) {
        self.bounds.insert(entity, Default::default());
        self.relative_bounds.insert(entity, Default::default());
        self.geo_changed.insert(entity, GeoChanged::empty());
        self.transform.insert(entity, Matrix::new_identity());
    }

    pub(crate) fn remove(&mut self, entity: Entity) {
        self.bounds.remove(entity);
        self.relative_bounds.remove(entity);
        self.draw_bounds.remove(entity);
        self.geo_changed.remove(entity);
        self.transform.remove(entity);
        self.clip_path.remove(entity);
        self.path.remove(entity);
    }

    /// Returns the bounding box of the entity, determined by the layout system.
    pub fn get_bounds(&self, entity: Entity) -> BoundingBox {
        self.bounds.get(entity).cloned().unwrap()
    }

    /// Returns the x position of the entity.
    pub fn get_posx(&self, entity: Entity) -> f32 {
        self.bounds.get(entity).map_or(0.0, |b| b.x)
    }

    /// Returns the y position of the entity.
    pub fn get_posy(&self, entity: Entity) -> f32 {
        self.bounds.get(entity).map_or(0.0, |b| b.y)
    }

    /// Returns the width of the entity.
    pub fn get_width(&self, entity: Entity) -> f32 {
        self.bounds.get(entity).map_or(0.0, |b| b.w)
    }

    /// Returns the height of the entity.
    pub fn get_height(&self, entity: Entity) -> f32 {
        self.bounds.get(entity).map_or(0.0, |b| b.h)
    }

    pub fn set_bounds(&mut self, entity: Entity, bounds: BoundingBox) {
        if let Some(b) = self.bounds.get_mut(entity) {
            *b = bounds;
        }
    }

    /// Sets the x position of the entity.
    pub fn set_posx(&mut self, entity: Entity, val: f32) {
        if let Some(bounds) = self.bounds.get_mut(entity) {
            bounds.x = val;
        }
    }

    /// Sets the y position of the entity.
    pub fn set_posy(&mut self, entity: Entity, val: f32) {
        if let Some(bounds) = self.bounds.get_mut(entity) {
            bounds.y = val;
        }
    }

    /// Sets the width of the entity.
    pub fn set_width(&mut self, entity: Entity, val: f32) {
        if let Some(bounds) = self.bounds.get_mut(entity) {
            bounds.w = val;
        }
    }

    /// Sets the height of the entity.
    pub fn set_height(&mut self, entity: Entity, val: f32) {
        if let Some(bounds) = self.bounds.get_mut(entity) {
            bounds.h = val;
        }
    }
}
