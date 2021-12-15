
use morphorm::{Cache, GeometryChanged};

use crate::{CachedData, Display, Entity, Visibility};

impl Cache for CachedData {

    type Item = Entity;

    fn visible(&self, node: Self::Item) -> bool {
        //self.visibility.get(node).cloned().map_or(true, |vis| vis == Visibility::Visible)
        self.display.get(node).cloned().map_or(true, |display| display == Display::Flex)
    }

    fn set_visible(&mut self, node: Self::Item, value: bool) {
        if let Some(visibility) = self.visibility.get_mut(node) {
            *visibility = if value {Visibility::Visible} else {Visibility::Invisible} 
        }
    }

    fn geometry_changed(&self, node: Self::Item) -> GeometryChanged {
        self.geometry_changed.get(node).cloned().unwrap_or_default()
    }

    fn set_geo_changed(&mut self, node: Self::Item, flag: GeometryChanged, value: bool) {
        if let Some(geometry_changed) = self.geometry_changed.get_mut(node) {
            geometry_changed.set(flag, value);
        }
    }

    fn new_width(&self, node: Self::Item) -> f32 {
        self.size.get(node).cloned().unwrap_or_default().width
    }

    fn new_height(&self, node: Self::Item) -> f32 {
        self.size.get(node).cloned().unwrap_or_default().height
    }

    fn set_new_width(&mut self, node: Self::Item, value: f32) {
        if let Some(size) = self.size.get_mut(node) {
            size.width = value;
        }
    }

    fn set_new_height(&mut self, node: Self::Item, value: f32) {
        if let Some(size) = self.size.get_mut(node) {
            size.height = value;
        }
    }

    // Width
    fn width(&self, node: Self::Item) -> f32 {
        self.get_width(node)
    }

    fn set_width(&mut self, node: Self::Item, value: f32) {
        self.set_width(node, value);
    }

    // Height
    fn height(&self, node: Self::Item) -> f32 {
        self.get_height(node)
    }

    fn set_height(&mut self, node: Self::Item, value: f32) {
        self.set_height(node, value);
    }

    // Posx
    fn posx(&self, node: Self::Item) -> f32 {
        self.get_posx(node)
    }

    fn set_posx(&mut self, node: Self::Item, value: f32) {
        self.set_posx(node, value)
    }

    // Posy
    fn posy(&self, node: Self::Item) -> f32 {
        self.get_posy(node)
    }

    fn set_posy(&mut self, node: Self::Item, value: f32) {
        self.set_posy(node, value)
    }

    // Left
    fn left(&self, node: Self::Item) -> f32 {
        //self.get_space_left(node)
        self.space.get(node).cloned().unwrap_or_default().left
    }

    fn set_left(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(node) {
            space.left = value;
        }
    }

    // Right
    fn right(&self, node: Self::Item) -> f32 {
        //self.get_space_right(node)
        self.space.get(node).cloned().unwrap_or_default().right
    }

    fn set_right(&mut self, node: Self::Item, value: f32) {
        self.set_space_right(node, value);
        if let Some(space) = self.space.get_mut(node) {
            space.right = value;
        }
    }

    // Top

    fn top(&self, node: Self::Item) -> f32 {
        //self.get_space_top(node)
        self.space.get(node).cloned().unwrap_or_default().top
    }

    fn set_top(&mut self, node: Self::Item, value: f32) {
        //self.set_space_top(node, value)
        if let Some(space) = self.space.get_mut(node) {
            space.top = value;
        }
    }

    // Bottom

    fn bottom(&self, node: Self::Item) -> f32 {
        //self.get_space_bottom(node)
        self.space.get(node).cloned().unwrap_or_default().bottom
    }

    fn set_bottom(&mut self, node: Self::Item, value: f32) {
        //self.set_space_bottom(node, value)
        if let Some(space) = self.space.get_mut(node) {
            space.bottom = value;
        }
    }

    // Child Width Max

    fn child_width_max(&self, node: Self::Item) -> f32 {
        self.get_child_width_max(node)
    }

    fn set_child_width_max(&mut self, node: Self::Item, value: f32) {
        self.set_child_width_max(node, value)
    }

    // Child Width Sum
    fn child_width_sum(&self, node: Self::Item) -> f32 {
        self.get_child_width_sum(node)
    }

    fn set_child_width_sum(&mut self, node: Self::Item, value: f32) {
        self.set_child_width_sum(node, value)
    }

    // Child Height Max
    fn child_height_max(&self, node: Self::Item) -> f32 {
        self.get_child_height_max(node)
    }

    fn set_child_height_max(&mut self, node: Self::Item, value: f32) {
        self.set_child_height_max(node, value);
    }

    // Child Height Sum
    fn child_height_sum(&self, node: Self::Item) -> f32 {
        self.get_child_height_sum(node)
    }

    fn set_child_height_sum(&mut self, node: Self::Item, value: f32) {
        self.set_child_height_sum(node, value)
    }

    //

    fn stack_first_child(&self, node: Self::Item) -> bool {
        self.get_stack_child(node).0
    }

    fn stack_last_child(&self, node: Self::Item) -> bool {
        self.get_stack_child(node).1
    }

    fn set_stack_first_child(&mut self, node: Self::Item, value: bool) {
        self.set_stack_first_child(node, value)
    }

    fn set_stack_last_child(&mut self, node: Self::Item, value: bool) {
        self.set_stack_last_child(node, value)
    }

    fn horizontal_free_space(&self, node: Self::Item) -> f32 {
        self.get_horizontal_free_space(node)
    }

    fn set_horizontal_free_space(&mut self, node: Self::Item, value: f32) {
        self.set_horizontal_free_space(node, value);
    }

    fn vertical_free_space(&self, node: Self::Item) -> f32 {
        self.get_vertical_free_space(node)
    }

    fn set_vertical_free_space(&mut self, node: Self::Item, value: f32) {
        self.set_vertical_free_space(node, value);
    }

    fn horizontal_stretch_sum(&self, node: Self::Item) -> f32 {
        self.get_horizontal_stretch_sum(node)
    }

    fn set_horizontal_stretch_sum(&mut self, node: Self::Item, value: f32) {
        self.set_horizontal_stretch_sum(node, value);
    }

    fn vertical_stretch_sum(&self, node: Self::Item) -> f32 {
        self.get_vertical_stretch_sum(node)
    }

    fn set_vertical_stretch_sum(&mut self, node: Self::Item, value: f32) {
        self.set_vertical_stretch_sum(node, value);
    }

    fn grid_row_max(&self, node: Self::Item) -> f32 {
        self.get_grid_row_max(node)
    }

    fn set_grid_row_max(&mut self, node: Self::Item, value: f32) {
        self.set_grid_row_max(node, value);
    }

    fn grid_col_max(&self, node: Self::Item) -> f32 {
        self.get_grid_col_max(node)
    }

    fn set_grid_col_max(&mut self, node: Self::Item, value: f32) {
        self.set_grid_col_max(node, value);
    }


}