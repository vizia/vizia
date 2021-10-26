use morphorm::Node;

use crate::{Entity, Style};

// TODO - generate these with a macro
impl<'a> Node<'a> for Entity {
    type Data = Style;

    fn layout_type(&self, store: &'_ Self::Data) -> Option<morphorm::LayoutType> {
        store.layout_type.get(*self).cloned()
    }

    fn position_type(&self, store: &'_ Self::Data) -> Option<morphorm::PositionType> {
        store.position_type.get(*self).cloned()
    }

    fn left(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.left.get(*self).cloned()
    }

    fn min_left(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.min_left.get(*self).cloned()
    }

    fn max_left(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.max_left.get(*self).cloned()
    }

    fn right(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.right.get(*self).cloned()
    }

    fn min_right(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.min_right.get(*self).cloned()
    }

    fn max_right(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.max_right.get(*self).cloned()
    }

    fn top(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.top.get(*self).cloned()
    }

    fn min_top(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.min_top.get(*self).cloned()
    }

    fn max_top(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.max_top.get(*self).cloned()
    }

    fn bottom(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.bottom.get(*self).cloned()
    }

    fn min_bottom(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.min_bottom.get(*self).cloned()
    }

    fn max_bottom(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.max_bottom.get(*self).cloned()
    }

    fn width(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.width.get(*self).cloned()
    }

    fn min_width(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.min_width.get(*self).cloned()
    }

    fn max_width(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.max_width.get(*self).cloned()
    }

    fn height(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.height.get(*self).cloned()
    }

    fn min_height(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.min_height.get(*self).cloned()
    }

    fn max_height(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.max_height.get(*self).cloned()
    }

    fn child_left(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.child_left.get(*self).cloned()
    }

    fn child_right(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.child_right.get(*self).cloned()
    }

    fn child_top(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.child_top.get(*self).cloned()
    }

    fn child_bottom(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.child_bottom.get(*self).cloned()
    }

    fn grid_cols(&self, store: &'_ Self::Data) -> Option<Vec<morphorm::Units>> {
        store.grid_cols.get(*self).cloned()
    }

    fn grid_rows(&self, store: &'_ Self::Data) -> Option<Vec<morphorm::Units>> {
        store.grid_rows.get(*self).cloned()
    }

    fn row_between(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.row_between.get(*self).cloned()
    }

    fn col_between(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.col_between.get(*self).cloned()
    }

    fn border_left(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.border_width.get(*self).cloned()
    }

    fn border_right(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.border_width.get(*self).cloned()
    }

    fn border_top(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.border_width.get(*self).cloned()
    }

    fn border_bottom(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        store.border_width.get(*self).cloned()
    }

    fn row_index(&self, store: &'_ Self::Data) -> Option<usize> {
        store.row_index.get(*self).cloned()
    }

    fn row_span(&self, store: &'_ Self::Data) -> Option<usize> {
        store.row_span.get(*self).cloned()
    }

    fn col_index(&self, store: &'_ Self::Data) -> Option<usize> {
        store.col_index.get(*self).cloned()
    }

    fn col_span(&self, store: &'_ Self::Data) -> Option<usize> {
        store.col_span.get(*self).cloned()
    }
}