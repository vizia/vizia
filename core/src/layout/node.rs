use femtovg::TextContext;
use morphorm::{Node, Units};

use crate::{Entity, ResourceManager, Style, text_layout, text_paint};

impl<'w> Node<'w> for Entity {
    type Data = (Style, TextContext, ResourceManager);

    fn layout_type(&self, store: &Self::Data) -> Option<morphorm::LayoutType> {
        store.0.layout_type.get(*self).cloned()
    }

    fn position_type(&self, store: &Self::Data) -> Option<morphorm::PositionType> {
        store.0.position_type.get(*self).cloned()
    }

    fn left(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.left.get(*self).cloned()
    }

    fn min_left(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.min_left.get(*self).cloned()
    }

    fn max_left(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.max_left.get(*self).cloned()
    }

    fn right(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.right.get(*self).cloned()
    }

    fn min_right(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.min_right.get(*self).cloned()
    }

    fn max_right(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.max_right.get(*self).cloned()
    }

    fn top(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.top.get(*self).cloned()
    }

    fn min_top(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.min_top.get(*self).cloned()
    }

    fn max_top(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.max_top.get(*self).cloned()
    }

    fn bottom(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.bottom.get(*self).cloned()
    }

    fn min_bottom(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.min_bottom.get(*self).cloned()
    }

    fn max_bottom(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.max_bottom.get(*self).cloned()
    }

    fn width(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.width.get(*self).cloned()
    }

    fn min_width(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.min_width.get(*self).cloned()
    }

    fn max_width(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.max_width.get(*self).cloned()
    }

    fn content_width(&self, store: &Self::Data) -> Option<f32> {
        store.0.content_width.get(*self).cloned()
    }

    fn content_height(&self, store: &Self::Data) -> Option<f32> {
        store.0.content_height.get(*self).cloned()
    }

    fn content_width_secondary(&self, store: &Self::Data, _height: f32) -> Option<f32> {
        store.0.content_width.get(*self).cloned()
    }

    fn content_height_secondary(&self, store: &Self::Data, width: f32) -> Option<f32> {
        if let Some(text) = store.0.text.get(*self) {
            let paint = text_paint(&store.0, &store.2, *self);

            let font_metrics =
                store.1.measure_font(paint).expect("Failed to read font metrics");
            let mut child_space_x = 0.0;
            if let Some(Units::Pixels(val)) = store.0.child_left.get(*self) {
                child_space_x += *val;
            }
            if let Some(Units::Pixels(val)) = store.0.child_right.get(*self) {
                child_space_x += *val;
            }
            let child_width = (width - child_space_x).max(0.0);

            if let Ok(lines) = text_layout(child_width, text, paint, &store.1) {
                Some(font_metrics.height() * lines.len() as f32)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn height(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.height.get(*self).cloned()
    }

    fn min_height(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.min_height.get(*self).cloned()
    }

    fn max_height(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.max_height.get(*self).cloned()
    }

    fn child_left(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.child_left.get(*self).cloned()
    }

    fn child_right(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.child_right.get(*self).cloned()
    }

    fn child_top(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.child_top.get(*self).cloned()
    }

    fn child_bottom(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.child_bottom.get(*self).cloned()
    }

    fn grid_cols(&self, store: &Self::Data) -> Option<Vec<morphorm::Units>> {
        store.0.grid_cols.get(*self).cloned()
    }

    fn grid_rows(&self, store: &Self::Data) -> Option<Vec<morphorm::Units>> {
        store.0.grid_rows.get(*self).cloned()
    }

    fn row_between(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.row_between.get(*self).cloned()
    }

    fn col_between(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.col_between.get(*self).cloned()
    }

    fn border_left(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.border_width.get(*self).cloned()
    }

    fn border_right(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.border_width.get(*self).cloned()
    }

    fn border_top(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.border_width.get(*self).cloned()
    }

    fn border_bottom(&self, store: &Self::Data) -> Option<morphorm::Units> {
        store.0.border_width.get(*self).cloned()
    }

    fn row_index(&self, store: &Self::Data) -> Option<usize> {
        store.0.row_index.get(*self).cloned()
    }

    fn row_span(&self, store: &Self::Data) -> Option<usize> {
        store.0.row_span.get(*self).cloned()
    }

    fn col_index(&self, store: &Self::Data) -> Option<usize> {
        store.0.col_index.get(*self).cloned()
    }

    fn col_span(&self, store: &Self::Data) -> Option<usize> {
        store.0.col_span.get(*self).cloned()
    }
}
