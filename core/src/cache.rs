use std::collections::HashMap;
use std::fmt::Debug;

use crate::style::Display;
use crate::Abilities;
use crate::Entity;
use femtovg::ImageId;
use morphorm::GeometryChanged;

use crate::style::Transform2D;
use crate::style::Visibility;

use crate::storage::sparse_set::SparseSet;
use crate::storage::sparse_set::SparseSetError;

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

/// Respresents an axis-aligned bounding box of an entity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct Space {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct Size {
    pub width: f32,
    pub height: f32,
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, w: std::f32::MAX, h: std::f32::MAX }
    }
}

impl BoundingBox {
    /// Construct a [`BoundingBox`] from checked minimum and maximum values.
    #[inline(always)]
    pub fn from_min_max(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> BoundingBox {
        BoundingBox { x: min_x, y: min_y, w: max_x - min_x, h: max_y - min_y }
    }

    /// Left side of bounds equivalent to `x`
    #[inline(always)]
    pub fn left(&self) -> f32 {
        self.x
    }

    /// Top of bounds equivalent to `y`
    #[inline(always)]
    pub fn top(&self) -> f32 {
        self.y
    }

    /// Bounds width equivalent to `w`
    #[inline(always)]
    pub fn width(&self) -> f32 {
        self.w
    }

    /// Bounds height equivalent to `h`
    #[inline(always)]
    pub fn height(&self) -> f32 {
        self.h
    }

    /// Right side of bounds
    #[inline(always)]
    pub fn right(&self) -> f32 {
        self.left() + self.width()
    }

    /// Bottom of bounds
    #[inline(always)]
    pub fn bottom(&self) -> f32 {
        self.top() + self.height()
    }

    /// Horizontal and vertical center of bounds
    #[inline(always)]
    pub fn center(&self) -> (f32, f32) {
        ((self.width() / 2f32) + self.x, (self.height() / 2f32) + self.y)
    }

    /// Left center of bounds
    #[inline(always)]
    pub fn center_left(&self) -> (f32, f32) {
        (self.left(), (self.height() / 2f32) + self.top())
    }

    /// Right center of bounds
    #[inline(always)]
    pub fn center_right(&self) -> (f32, f32) {
        (self.right(), (self.height() / 2f32) + self.top())
    }

    /// Top center of bounds
    #[inline(always)]
    pub fn center_top(&self) -> (f32, f32) {
        ((self.width() / 2f32) + self.left(), self.top())
    }

    /// Bottom center of bounds
    #[inline(always)]
    pub fn center_bottom(&self) -> (f32, f32) {
        ((self.width() / 2f32) + self.left(), self.bottom())
    }

    /// Bottom left point of bounds
    #[inline(always)]
    pub fn bottom_left(&self) -> (f32, f32) {
        (self.left(), self.bottom())
    }

    /// Bottom right point of bounds
    #[inline(always)]
    pub fn bottom_right(&self) -> (f32, f32) {
        (self.right(), self.bottom())
    }

    /// Top left point of bounds
    #[inline(always)]
    pub fn top_left(&self) -> (f32, f32) {
        (self.left(), self.top())
    }

    /// Top right point of bounds
    #[inline(always)]
    pub fn top_right(&self) -> (f32, f32) {
        (self.right(), self.top())
    }

    /// Shrinks by some `amount` in both directions and returns a new
    /// [`BoundingBox`].
    #[inline(always)]
    #[must_use]
    pub fn shrink(&self, amount: f32) -> BoundingBox {
        BoundingBox::from_min_max(
            self.left() + amount,
            self.top() + amount,
            self.right() - amount,
            self.bottom() - amount,
        )
    }

    /// Shrinks by some `amount` horizontally and returns a new [`BoundingBox`].
    #[inline(always)]
    #[must_use]
    pub fn shrink_horizontal(&self, amount: f32) -> BoundingBox {
        BoundingBox::from_min_max(
            self.left() + amount,
            self.top(),
            self.right() - amount,
            self.bottom(),
        )
    }

    /// Shrinks by some `amount` vertically and returns a new [`BoundingBox`].
    #[inline(always)]
    #[must_use]
    pub fn shrink_vertical(&self, amount: f32) -> BoundingBox {
        BoundingBox::from_min_max(
            self.left(),
            self.top() + amount,
            self.right(),
            self.bottom() - amount,
        )
    }

    /// Expands by some `amount` in both directions and returns a new
    /// [`BoundingBox`].
    #[inline(always)]
    #[must_use]
    pub fn expand(&self, amount: f32) -> BoundingBox {
        BoundingBox::from_min_max(
            self.left() - amount,
            self.top() - amount,
            self.right() + amount,
            self.bottom() + amount,
        )
    }

    /// Expands by some `amount` horizontally and returns a new [`BoundingBox`].
    #[inline(always)]
    #[must_use]
    pub fn expand_horizontal(&self, amount: f32) -> BoundingBox {
        BoundingBox::from_min_max(
            self.left() - amount,
            self.top(),
            self.right() + amount,
            self.bottom(),
        )
    }

    /// Expands by some `amount` vertically and returns a new [`BoundingBox`].
    #[inline(always)]
    #[must_use]
    pub fn expand_vertical(&self, amount: f32) -> BoundingBox {
        BoundingBox::from_min_max(
            self.left(),
            self.top() - amount,
            self.right(),
            self.bottom() + amount,
        )
    }
}

/// Stores data which can be cached between system runs.
///
/// When an event occurs or style data is changed systems run to determine the new state of the UI.
/// The output of these systems can be cached so that not all of the systems need to run again.
#[derive(Default)]
pub struct CachedData {
    pub(crate) bounds: SparseSet<BoundingBox>,
    pub(crate) visibility: SparseSet<Visibility>,
    pub(crate) display: SparseSet<Display>,
    pub(crate) opacity: SparseSet<f32>,

    pub(crate) abilities: SparseSet<Abilities>,

    pub(crate) z_index: SparseSet<i32>,

    pub(crate) child_sum: SparseSet<(f32, f32)>, // Sum of child (widths, heights)
    pub(crate) child_max: SparseSet<(f32, f32)>, // Max child (widths, heights)

    //pub(crate) prev_size: SparseSet<Pos>,
    clip_region: SparseSet<BoundingBox>,

    // Transform
    rotate: SparseSet<f32>,
    scale: SparseSet<(f32, f32)>,
    transform: SparseSet<Transform2D>,

    origin: SparseSet<(f32, f32)>,

    pub(crate) space: SparseSet<Space>,
    pub(crate) size: SparseSet<Size>,
    cross_stretch_sum: SparseSet<f32>,
    cross_free_space: SparseSet<f32>,

    horizontal_free_space: SparseSet<f32>,
    horizontal_stretch_sum: SparseSet<f32>,
    vertical_free_space: SparseSet<f32>,
    vertical_stretch_sum: SparseSet<f32>,

    grid_row_max: SparseSet<f32>,
    grid_col_max: SparseSet<f32>,

    // is_first_child, is_last_child
    stack_child: SparseSet<(bool, bool)>,

    pub(crate) geometry_changed: SparseSet<GeometryChanged>,

    pub(crate) shadow_image: HashMap<Entity, (ImageId, ImageId)>,
}

impl CachedData {
    pub fn add(&mut self, entity: Entity) -> Result<(), SparseSetError> {
        self.bounds.insert(entity, Default::default())?;
        self.visibility.insert(entity, Default::default())?;
        self.display.insert(entity, Default::default())?;
        self.child_sum.insert(entity, (0.0, 0.0))?;
        self.child_max.insert(entity, (0.0, 0.0))?;

        self.opacity.insert(entity, 1.0)?;

        self.rotate.insert(entity, 0.0)?;
        self.scale.insert(entity, (1.0, 1.0))?;
        self.transform.insert(entity, Transform2D::identity())?;
        self.origin.insert(entity, (0.0, 0.0))?;

        self.z_index.insert(entity, 0)?;

        self.clip_region.insert(entity, Default::default())?;
        self.space.insert(entity, Default::default())?;
        self.size.insert(entity, Default::default())?;
        self.cross_stretch_sum.insert(entity, Default::default())?;
        self.cross_free_space.insert(entity, Default::default())?;

        self.horizontal_free_space.insert(entity, Default::default())?;
        self.horizontal_stretch_sum.insert(entity, Default::default())?;
        self.vertical_free_space.insert(entity, Default::default())?;
        self.vertical_stretch_sum.insert(entity, Default::default())?;
        self.stack_child.insert(entity, (false, false))?;

        self.grid_row_max.insert(entity, 0.0)?;
        self.grid_col_max.insert(entity, 0.0)?;

        self.geometry_changed.insert(entity, Default::default())?;

        self.abilities.insert(entity, Default::default())?;

        Ok(())
    }

    pub fn remove(&mut self, entity: Entity) {
        self.bounds.remove(entity);
        self.visibility.remove(entity);
        self.child_sum.remove(entity);
        self.child_max.remove(entity);

        self.opacity.remove(entity);

        self.rotate.remove(entity);
        self.scale.remove(entity);
        self.transform.remove(entity);
        self.origin.remove(entity);

        self.z_index.remove(entity);

        self.clip_region.remove(entity);
        self.space.remove(entity);
        self.size.remove(entity);
        self.cross_stretch_sum.remove(entity);
        self.cross_free_space.remove(entity);

        self.horizontal_free_space.remove(entity);
        self.horizontal_stretch_sum.remove(entity);
        self.vertical_free_space.remove(entity);
        self.vertical_stretch_sum.remove(entity);
        self.stack_child.remove(entity);

        self.grid_row_max.remove(entity);
        self.grid_col_max.remove(entity);

        self.geometry_changed.remove(entity);

        self.abilities.remove(entity);
    }

    // For getters and setters it's safe to use unwrap because every entity must have a position and size.
    // Event if the position and size are 0.0, or the entity is invisible.

    // pub fn get_clip_widget(&self, entity: Entity) -> Entity {
    //     self.clip_widget
    //         .get(entity.index_unchecked())
    //         .cloned()
    //         .unwrap()
    // }

    pub fn get_grid_row_max(&self, entity: Entity) -> f32 {
        self.grid_row_max.get(entity).cloned().unwrap_or_default()
    }

    pub fn set_grid_row_max(&mut self, entity: Entity, value: f32) {
        if let Some(grid_row_max) = self.grid_row_max.get_mut(entity) {
            *grid_row_max = value;
        }
    }

    pub fn get_grid_col_max(&self, entity: Entity) -> f32 {
        self.grid_col_max.get(entity).cloned().unwrap_or_default()
    }

    pub fn set_grid_col_max(&mut self, entity: Entity, value: f32) {
        if let Some(grid_col_max) = self.grid_col_max.get_mut(entity) {
            *grid_col_max = value;
        }
    }

    pub fn get_stack_child(&self, entity: Entity) -> (bool, bool) {
        self.stack_child.get(entity).cloned().unwrap_or((false, false))
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
    // pub(crate) fn get_cross_stretch_sum(&self, entity: Entity) -> f32 {
    //     self.cross_stretch_sum
    //         .get(entity)
    //         .cloned()
    //         .unwrap()
    // }

    // pub(crate) fn set_cross_stretch_sum(&mut self, entity: Entity, val: f32) {
    //     if let Some(cross_stretch_sum) = self.cross_stretch_sum.get_mut(entity) {
    //         *cross_stretch_sum = val;
    //     }
    // }

    // pub(crate) fn get_cross_free_space(&self, entity: Entity) -> f32 {
    //     self.cross_free_space
    //         .get(entity)
    //         .cloned()
    //         .unwrap()
    // }

    // pub(crate) fn set_cross_free_space(&mut self, entity: Entity, val: f32) {
    //     if let Some(cross_free_space) = self.cross_free_space.get_mut(entity) {
    //         *cross_free_space = val;
    //     }
    // }

    // pub(crate) fn get_space_left(&self, entity: Entity) -> f32 {
    //     self.space
    //         .get(entity)
    //         .cloned()
    //         .unwrap()
    //         .left
    // }

    // pub(crate) fn get_space_right(&self, entity: Entity) -> f32 {
    //     self.space
    //         .get(entity)
    //         .cloned()
    //         .unwrap()
    //         .right
    // }

    // pub(crate) fn get_space_top(&self, entity: Entity) -> f32 {
    //     self.space
    //         .get(entity)
    //         .cloned()
    //         .unwrap()
    //         .top
    // }

    // pub(crate) fn get_space_bottom(&self, entity: Entity) -> f32 {
    //     self.space
    //         .get(entity)
    //         .cloned()
    //         .unwrap()
    //         .bottom
    // }

    // pub fn get_space(&self, entity: Entity) -> Space {
    //     self.space.get(entity).cloned().unwrap()
    // }

    /// Returns the clip region of the entity.
    ///
    /// This is the bounding box for which rendering of the widget will be clipped/cropped when outside of the bounds.
    pub fn get_clip_region(&self, entity: Entity) -> BoundingBox {
        self.clip_region.get(entity).cloned().unwrap()
    }

    /// Returns the Z index of the entity.
    ///
    /// Entities can specify a z-index with `entity.set_z_index(cx, value)`.
    /// The z_order_system then determines the z-index of child entities based on their parent and any specified z-index.
    pub fn get_z_index(&self, entity: Entity) -> i32 {
        self.z_index.get(entity).cloned().unwrap()
    }

    pub fn get_child_width_sum(&self, entity: Entity) -> f32 {
        self.child_sum.get(entity).cloned().unwrap().0
    }

    pub fn get_child_height_sum(&self, entity: Entity) -> f32 {
        self.child_sum.get(entity).cloned().unwrap().1
    }

    pub fn get_child_width_max(&self, entity: Entity) -> f32 {
        self.child_max.get(entity).cloned().unwrap().0
    }

    pub fn get_child_height_max(&self, entity: Entity) -> f32 {
        self.child_max.get(entity).cloned().unwrap().1
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

    // pub(crate) fn get_prev_width(&self, entity: Entity) -> f32 {
    //     self.prev_size
    //         .get(entity.index_unchecked())
    //         .cloned()
    //         .unwrap_or_default()
    //         .x
    // }

    // pub(crate) fn get_prev_height(&self, entity: Entity) -> f32 {
    //     self.prev_size
    //         .get(entity.index_unchecked())
    //         .cloned()
    //         .unwrap_or_default()
    //         .y
    // }

    /// Returns the opacity of the entity.
    pub fn get_opacity(&self, entity: Entity) -> f32 {
        self.opacity.get(entity).cloned().unwrap()
    }

    pub fn get_horizontal_free_space(&self, entity: Entity) -> f32 {
        self.horizontal_free_space.get(entity).cloned().unwrap()
    }

    pub fn get_horizontal_stretch_sum(&self, entity: Entity) -> f32 {
        self.horizontal_stretch_sum.get(entity).cloned().unwrap()
    }

    pub fn get_vertical_free_space(&self, entity: Entity) -> f32 {
        self.vertical_free_space.get(entity).cloned().unwrap()
    }

    pub fn get_vertical_stretch_sum(&self, entity: Entity) -> f32 {
        self.vertical_stretch_sum.get(entity).cloned().unwrap()
    }

    pub fn get_rotate(&self, entity: Entity) -> f32 {
        self.transform.get(entity).cloned().unwrap()[0].acos()
    }

    pub fn get_translate(&self, entity: Entity) -> (f32, f32) {
        let transform = self.transform.get(entity).cloned().unwrap();

        (transform[4], transform[5])
    }

    pub fn get_scale(&self, entity: Entity) -> f32 {
        let scale = self.scale.get(entity).cloned().unwrap();

        scale.0
    }

    pub fn get_origin(&self, entity: Entity) -> (f32, f32) {
        self.origin.get(entity).cloned().unwrap()
    }

    /// Returns the transform on the entity.
    pub fn get_transform(&self, entity: Entity) -> Transform2D {
        self.transform.get(entity).cloned().unwrap()
    }

    pub fn get_transform_mut(&mut self, entity: Entity) -> &mut Transform2D {
        self.transform.get_mut(entity).unwrap()
    }

    // SETTERS

    // pub fn set_clip_widget(&mut self, entity: Entity, val: Entity) {
    //     if let Some(clip_widget) = self.clip_widget.get_mut(entity.index_unchecked()) {
    //         *clip_widget = val;
    //     }
    // }

    pub fn set_stack_first_child(&mut self, entity: Entity, value: bool) {
        if let Some(stack_child) = self.stack_child.get_mut(entity) {
            stack_child.0 = value;
        }
    }

    pub fn set_stack_last_child(&mut self, entity: Entity, value: bool) {
        if let Some(stack_child) = self.stack_child.get_mut(entity) {
            stack_child.1 = value;
        }
    }

    pub fn set_horizontal_free_space(&mut self, entity: Entity, value: f32) {
        if let Some(horizontal_free_space) = self.horizontal_free_space.get_mut(entity) {
            *horizontal_free_space = value;
        }
    }

    pub fn set_horizontal_stretch_sum(&mut self, entity: Entity, value: f32) {
        if let Some(horizontal_stretch_sum) = self.horizontal_stretch_sum.get_mut(entity) {
            *horizontal_stretch_sum = value;
        }
    }

    pub fn set_vertical_free_space(&mut self, entity: Entity, value: f32) {
        if let Some(vertical_free_space) = self.vertical_free_space.get_mut(entity) {
            *vertical_free_space = value;
        }
    }

    pub fn set_vertical_stretch_sum(&mut self, entity: Entity, value: f32) {
        if let Some(vertical_stretch_sum) = self.vertical_stretch_sum.get_mut(entity) {
            *vertical_stretch_sum = value;
        }
    }

    // pub fn set_space(&mut self, entity: Entity, val: Space) {
    //     if let Some(space) = self.space.get_mut(entity.index_unchecked()) {
    //         *space = val;
    //     }
    // }

    // pub(crate) fn set_space_left(&mut self, entity: Entity, val: f32) {
    //     if let Some(space) = self.space.get_mut(entity) {
    //         space.left = val;
    //     }
    // }

    pub fn set_space_right(&mut self, entity: Entity, val: f32) {
        if let Some(space) = self.space.get_mut(entity) {
            space.right = val;
        }
    }

    // pub(crate) fn set_space_top(&mut self, entity: Entity, val: f32) {
    //     if let Some(space) = self.space.get_mut(entity) {
    //         space.top = val;
    //     }
    // }

    // pub(crate) fn set_space_bottom(&mut self, entity: Entity, val: f32) {
    //     if let Some(space) = self.space.get_mut(entity) {
    //         space.bottom = val;
    //     }
    // }

    pub fn set_clip_region(&mut self, entity: Entity, val: BoundingBox) {
        if let Some(clip_region) = self.clip_region.get_mut(entity) {
            *clip_region = val;
        }
    }

    pub fn set_z_index(&mut self, entity: Entity, val: i32) {
        if let Some(z_index) = self.z_index.get_mut(entity) {
            *z_index = val;
        }
    }

    pub fn set_child_width_sum(&mut self, entity: Entity, val: f32) {
        if let Some(child_sum) = self.child_sum.get_mut(entity) {
            child_sum.0 = val;
        }
    }

    pub fn set_child_height_sum(&mut self, entity: Entity, val: f32) {
        if let Some(child_sum) = self.child_sum.get_mut(entity) {
            child_sum.1 = val;
        }
    }

    pub fn set_child_width_max(&mut self, entity: Entity, val: f32) {
        if let Some(child_max) = self.child_max.get_mut(entity) {
            child_max.0 = val;
        }
    }

    pub fn set_child_height_max(&mut self, entity: Entity, val: f32) {
        if let Some(child_max) = self.child_max.get_mut(entity) {
            child_max.1 = val;
        }
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

    // pub(crate) fn set_prev_width(&mut self, entity: Entity, val: f32) {
    //     if let Some(size) = self.prev_size.get_mut(entity.index_unchecked()) {
    //         size.x = val;
    //     }
    // }

    // pub(crate) fn set_prev_height(&mut self, entity: Entity, val: f32) {
    //     if let Some(size) = self.prev_size.get_mut(entity.index_unchecked()) {
    //         size.y = val;
    //     }
    // }

    pub fn get_visibility(&self, entity: Entity) -> Visibility {
        self.visibility.get(entity).cloned().unwrap()
    }

    pub fn get_display(&self, entity: Entity) -> Display {
        self.display.get(entity).cloned().unwrap()
    }

    pub(crate) fn set_visibility(&mut self, entity: Entity, val: Visibility) {
        if let Some(visibility) = self.visibility.get_mut(entity) {
            *visibility = val;
        }
    }

    pub(crate) fn set_display(&mut self, entity: Entity, val: Display) {
        if let Some(display) = self.display.get_mut(entity) {
            *display = val;
        }
    }

    // pub(crate) fn set_hoverable(&mut self, entity: Entity, val: bool) {
    //     if let Some(abilities) = self.abilities.get_mut(entity) {
    //         abilities.set(Abilities::HOVERABLE, val);
    //     }
    // }

    // pub(crate) fn set_focusable(&mut self, entity: Entity, val: bool) {
    //     if let Some(abilities) = self.abilities.get_mut(entity) {
    //         abilities.set(Abilities::FOCUSABLE, val);
    //     }
    // }

    // pub(crate) fn set_checkable(&mut self, entity: Entity, val: bool) {
    //     if let Some(abilities) = self.abilities.get_mut(entity) {
    //         abilities.set(Abilities::CHECKABLE, val);
    //     }
    // }

    // pub(crate) fn set_selectable(&mut self, entity: Entity, val: bool) {
    //     if let Some(abilities) = self.abilities.get_mut(entity) {
    //         abilities.set(Abilities::SELECTABLE, val);
    //     }
    // }

    pub fn set_opacity(&mut self, entity: Entity, val: f32) {
        if let Some(opacity) = self.opacity.get_mut(entity) {
            *opacity = val;
        }
    }

    pub(crate) fn set_rotate(&mut self, entity: Entity, val: f32) {
        if let Some(transform) = self.transform.get_mut(entity) {
            let mut t = Transform2D::identity();
            t.rotate(val);
            transform.premultiply(&t);
        }
    }

    pub(crate) fn set_translate(&mut self, entity: Entity, val: (f32, f32)) {
        if let Some(transform) = self.transform.get_mut(entity) {
            let mut t = Transform2D::identity();
            t.translate(val.0, val.1);
            transform.premultiply(&t);
        }
    }

    pub(crate) fn set_scale(&mut self, entity: Entity, val: (f32, f32)) {
        if let Some(transform) = self.transform.get_mut(entity) {
            let mut t = Transform2D::identity();
            t.scale(val.0, val.1);
            transform.premultiply(&t);
        }
    }

    // pub(crate) fn set_origin(&mut self, entity: Entity, val: (f32, f32)) {
    //     if let Some(origin) = self.origin.get_mut(entity) {
    //         *origin = val;
    //     }
    // }

    pub(crate) fn set_transform(&mut self, entity: Entity, val: Transform2D) {
        if let Some(transform) = self.transform.get_mut(entity) {
            *transform = val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn rect() -> BoundingBox {
        BoundingBox { x: 100f32, y: 100f32, w: 100f32, h: 100f32 }
    }

    #[test]
    fn get_center() {
        let rect = rect();
        assert_eq!(rect.center(), (150f32, 150f32));
    }

    #[test]
    fn get_center_top() {
        let rect = rect();
        assert_eq!(rect.center_top(), (150f32, 100f32));
    }

    #[test]
    fn get_center_bottom() {
        let rect = rect();
        assert_eq!(rect.center_bottom(), (150f32, 200f32));
    }

    #[test]
    fn get_center_left() {
        let rect = rect();
        assert_eq!(rect.center_left(), (100f32, 150f32));
    }

    #[test]
    fn get_center_right() {
        let rect = rect();
        assert_eq!(rect.center_right(), (200f32, 150f32));
    }

    #[test]
    fn get_left() {
        let rect = rect();
        assert_eq!(rect.left(), 100f32);
    }

    #[test]
    fn get_right() {
        let rect = rect();
        assert_eq!(rect.right(), 200f32);
    }

    #[test]
    fn get_top() {
        let rect = rect();
        assert_eq!(rect.top(), 100f32);
    }

    #[test]
    fn get_bottom() {
        let rect = rect();
        assert_eq!(rect.bottom(), 200f32);
    }

    #[test]
    fn get_shrunken() {
        let rect = rect();
        let a = rect.shrink(25f32);
        let b = BoundingBox { x: 125f32, y: 125f32, w: 50f32, h: 50f32 };
        assert_eq!(a, b);
    }

    #[test]
    fn get_shrunken_horizontal() {
        let rect = rect();
        let a = rect.shrink_horizontal(25f32);
        let b = BoundingBox { x: 125f32, y: 100f32, w: 50f32, h: 100f32 };
        assert_eq!(a, b);
    }

    #[test]
    fn get_shrunken_vertical() {
        let rect = rect();
        let a = rect.shrink_vertical(25f32);
        let b = BoundingBox { x: 100f32, y: 125f32, w: 100f32, h: 50f32 };
        assert_eq!(a, b);
    }

    #[test]
    fn get_expanded() {
        let rect = rect();
        let a = rect.expand(25f32);
        let b = BoundingBox { x: 75f32, y: 75f32, w: 150f32, h: 150f32 };
        assert_eq!(a, b);
    }

    #[test]
    fn get_expanded_horizontal() {
        let rect = rect();
        let a = rect.expand_horizontal(25f32);
        let b = BoundingBox { x: 75f32, y: 100f32, w: 150f32, h: 100f32 };
        assert_eq!(a, b);
    }

    #[test]
    fn get_expanded_vertical() {
        let rect = rect();
        let a = rect.expand_vertical(25f32);
        let b = BoundingBox { x: 100f32, y: 75f32, w: 100f32, h: 150f32 };
        assert_eq!(a, b);
    }
}
