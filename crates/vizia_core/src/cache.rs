//! The cache is a store for intermediate data produced while computing state, notably layout
//! results. The main type here is CachedData, usually accessed via `cx.cache`.

use crate::prelude::*;
use femtovg::ImageId;
use vizia_storage::SparseSet;

/// Stores data which can be cached between system runs.
///
/// When an event occurs or style data is changed systems run to determine the new state of the UI.
/// The output of these systems can be cached so that not all of the systems need to run again.
#[derive(Default)]
pub struct CachedData {
    pub(crate) bounds: SparseSet<BoundingBox>,
    pub(crate) shadow_images: SparseSet<Vec<Option<(ImageId, ImageId)>>>,
    pub(crate) filter_image: SparseSet<Option<(ImageId, ImageId)>>,
    pub(crate) screenshot_image: SparseSet<Option<ImageId>>,
}

impl CachedData {
    pub(crate) fn add(&mut self, entity: Entity) {
        self.bounds.insert(entity, Default::default());
    }

    pub(crate) fn remove(&mut self, entity: Entity) {
        self.bounds.remove(entity);
        self.filter_image.remove(entity);
        self.screenshot_image.remove(entity);
        self.shadow_images.remove(entity);
    }

    /// Returns the bounding box of the entity, determined by the layout system.
    pub fn get_bounds(&self, entity: Entity) -> BoundingBox {
        self.bounds.get(entity).cloned().unwrap()
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
