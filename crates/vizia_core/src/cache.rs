//! The cache is a store for intermediate data produced while computing state, notably layout
//! results. The main type here is CachedData, usually accessed via `cx.cache`.

use crate::prelude::*;
use hashbrown::HashSet;
use skia_safe::{Matrix, Path, matrix::Member};
use vizia_storage::SparseSet;

/// The cached draw bounds of an entity and its descendants.
///
/// All bounds are in the coordinate space of the layout parent of the entity, i.e. they include the
/// local transform of the entity but not the transforms of its ancestors, and they are not limited
/// by the clip paths of ancestors. This keeps them valid when an ancestor is transformed (e.g.
/// scrolled) or clipped differently. Empty (zero-sized) boxes mean nothing is drawn.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub(crate) struct DrawBounds {
    /// The bounds drawn by the entity itself.
    pub(crate) own: BoundingBox,
    /// The bounds drawn by the entity and its descendants which are limited by the clip path of
    /// the parent of the entity.
    pub(crate) clipped: BoundingBox,
    /// The bounds drawn by descendants (or the entity itself) which escape the clip path of the
    /// parent of the entity.
    pub(crate) escaped: BoundingBox,
    /// Whether the entity itself escapes the clip path of its parent.
    pub(crate) escapes_clip: bool,
}

/// Stores data which can be cached between system runs.
///
/// When an event occurs or style data is changed systems run to determine the new state of the UI.
/// The output of these systems can be cached so that not all of the systems need to run again.
#[derive(Default)]
pub struct CachedData {
    pub(crate) bounds: SparseSet<BoundingBox>,
    /// Draw bounds of an entity and its draw descendants, used by the draw system to skip subtrees
    /// which do not intersect the dirty rect.
    ///
    /// Invariant: if an entity has an entry, so do all of its draw children. Use
    /// [`CachedData::invalidate_draw_bounds`] to clear an entry and all of its ancestors.
    pub(crate) draw_bounds: SparseSet<DrawBounds>,
    pub(crate) relative_bounds: SparseSet<BoundingBox>,
    pub(crate) geo_changed: SparseSet<GeoChanged>,
    /// The transform of each entity relative to its layout parent. Use
    /// [`CachedData::absolute_transform`] for the transform in window coordinates.
    pub(crate) local_transform: SparseSet<Matrix>,
    /// Clip paths in window coordinates.
    pub(crate) clip_path: SparseSet<Option<Path>>,
    /// Bounds of the clip paths, expanded to whole pixels.
    pub(crate) clip_bounds: SparseSet<BoundingBox>,
    /// Entities which clip their content, rather than only inheriting the clip of their parent.
    pub(crate) own_clips: HashSet<Entity>,
    pub(crate) path: SparseSet<Path>,
}

/// Returns true if a matrix only scales and translates. Unlike the Skia type mask, this does not
/// call into Skia.
pub(crate) fn is_scale_translate(matrix: &Matrix) -> bool {
    matrix.skew_x() == 0.0
        && matrix.skew_y() == 0.0
        && matrix[Member::Persp0] == 0.0
        && matrix[Member::Persp1] == 0.0
        && matrix[Member::Persp2] == 1.0
}

/// Returns true if a matrix is the identity. Unlike [Matrix::is_identity], this does not call
/// into Skia.
pub(crate) fn is_identity(matrix: &Matrix) -> bool {
    is_scale_translate(matrix)
        && matrix.scale_x() == 1.0
        && matrix.scale_y() == 1.0
        && matrix.translate_x() == 0.0
        && matrix.translate_y() == 0.0
}
impl CachedData {
    pub(crate) fn add(&mut self, entity: Entity) {
        self.bounds.insert(entity, Default::default());
        self.relative_bounds.insert(entity, Default::default());
        self.geo_changed.insert(entity, GeoChanged::empty());
        self.local_transform.insert(entity, Matrix::new_identity());
    }

    pub(crate) fn remove(&mut self, entity: Entity) {
        self.bounds.remove(entity);
        self.relative_bounds.remove(entity);
        self.draw_bounds.remove(entity);
        self.geo_changed.remove(entity);
        self.local_transform.remove(entity);
        self.clip_path.remove(entity);
        self.clip_bounds.remove(entity);
        self.own_clips.remove(&entity);
        self.path.remove(entity);
    }

    /// Returns the transform of an entity in window coordinates, combining its local transform
    /// with those of its ancestors up to its window.
    pub(crate) fn absolute_transform(&self, tree: &Tree<Entity>, entity: Entity) -> Matrix {
        let mut transform: Option<Matrix> = None;
        let mut current = Some(entity);
        while let Some(entity) = current {
            if let Some(local) = self.local_transform.get(entity)
                && !is_identity(local)
            {
                transform = Some(transform.map_or(*local, |transform| *local * transform));
            }

            if tree.is_window(entity) {
                break;
            }
            current = tree.get_layout_parent(entity);
        }

        transform.unwrap_or_default()
    }

    /// Invalidates the cached draw bounds of an entity and all of its ancestors.
    ///
    /// Must be called whenever the draw bounds of an entity change, or an entity is added to or
    /// removed from the tree.
    pub(crate) fn invalidate_draw_bounds(&mut self, tree: &Tree<Entity>, entity: Entity) {
        self.draw_bounds.remove(entity);
        let mut ancestor = tree.get_layout_parent(entity);
        while let Some(current) = ancestor {
            // A cached entry implies its draw children are cached, so an uncached ancestor means
            // every ancestor above it is uncached as well.
            if self.draw_bounds.remove(current).is_none() {
                break;
            }
            ancestor = tree.get_layout_parent(current);
        }
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
