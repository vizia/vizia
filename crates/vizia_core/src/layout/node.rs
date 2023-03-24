use morphorm::{Node, Units};
use vizia_storage::{LayoutChildIterator, MorphormChildIter};

use crate::prelude::*;
use crate::style::Style;
use crate::text::TextContext;

impl Node for Entity {
    type Store = Style;
    type Tree = Tree<Entity>;
    type CacheKey = Entity;
    type ChildIter<'t> = MorphormChildIter<'t, Entity>;
    type SubLayout = TextContext;

    fn children<'t>(&'t self, tree: &'t Self::Tree) -> Self::ChildIter<'t> {
        MorphormChildIter::new(tree, *self)
    }

    fn key(&self) -> Self::CacheKey {
        *self
    }

    fn visible(&self, store: &Self::Store) -> bool {
        store.display.get(*self).copied().map(|display| display == Display::Flex).unwrap_or(true)
    }

    fn layout_type(&self, store: &Self::Store) -> Option<morphorm::LayoutType> {
        store.layout_type.get(*self).cloned()
    }

    fn position_type(&self, store: &Self::Store) -> Option<morphorm::PositionType> {
        store.position_type.get(*self).cloned()
    }

    fn left(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.left.get(*self).cloned().map(|l| match l {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn min_left(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.min_left.get(*self).cloned().map(|l| match l {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn max_left(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.max_left.get(*self).cloned().map(|l| match l {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn right(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.right.get(*self).cloned().map(|r| match r {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn min_right(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.min_right.get(*self).cloned().map(|r| match r {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn max_right(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.max_right.get(*self).cloned().map(|r| match r {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn top(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.top.get(*self).cloned().map(|t| match t {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn min_top(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.min_top.get(*self).cloned().map(|t| match t {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn max_top(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.max_top.get(*self).cloned().map(|t| match t {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn bottom(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.bottom.get(*self).cloned().map(|b| match b {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn min_bottom(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.min_bottom.get(*self).cloned().map(|b| match b {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn max_bottom(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.max_bottom.get(*self).cloned().map(|b| match b {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn width(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.width.get(*self).cloned().map(|w| match w {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn min_width(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.min_width.get(*self).cloned().map(|w| match w {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn max_width(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.max_width.get(*self).cloned().map(|w| match w {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    // fn content_width(&self, store: &Self::Store) -> Option<f32> {
    //     store.content_width.get(*self).cloned().map(|x| (x * store.dpi_factor as f32).ceil())
    // }

    // fn content_height(&self, store: &Self::Store) -> Option<f32> {
    //     store.content_height.get(*self).cloned().map(|x| (x * store.dpi_factor as f32).ceil())
    // }

    // fn content_width_secondary(
    //     &self,
    //     store: &'_ Self::Store,
    //     _sublayout: &'_ mut Self::Sublayout,
    //     _height: f32,
    // ) -> Option<f32> {
    //     store.content_width.get(*self).cloned().map(|x| (x * store.dpi_factor as f32).ceil())
    // }

    // fn content_height_secondary(
    //     &self,
    //     store: &Self::Store,
    //     sublayout: &'_ mut Self::Sublayout,
    //     width: f32,
    // ) -> Option<f32> {
    //     let width = width.ceil();
    //     if !store.text_wrap.get(*self).copied().unwrap_or(true) {
    //         return None;
    //     }

    //     if sublayout.has_buffer(*self) {
    //         Some(sublayout.with_buffer(*self, |buf| {
    //             buf.set_size(width as i32, i32::MAX);
    //             buf.layout_runs().count() as f32 * buf.metrics().line_height as f32
    //         }))
    //     } else {
    //         None
    //     }
    // }

    fn content_size(
        &self,
        store: &Self::Store,
        text_context: &mut Self::SubLayout,
        width: Option<f32>,
        height: Option<f32>,
    ) -> Option<(f32, f32)> {
        println!("{:?} {:?}", width, height);
        if !text_context.has_buffer(*self) {
            return None;
        }

        // If the width is known use that, else use 0 for wrapping text or 999999 for non-wrapping text.
        let width = if let Some(width) = width {
            width.ceil() as i32
        } else {
            if store.text_wrap.get(*self).copied().unwrap_or(true) {
                0
            } else {
                999999
            }
        };

        let child_left = store.child_left.get(*self).cloned().unwrap_or_default();
        let child_right = store.child_right.get(*self).cloned().unwrap_or_default();
        let child_top = store.child_top.get(*self).cloned().unwrap_or_default();
        let child_bottom = store.child_bottom.get(*self).cloned().unwrap_or_default();

        let mut child_space_x = 0.0;
        let mut child_space_y = 0.0;

        // shrink the bounding box based on pixel values
        if let Pixels(val) = child_left {
            let val = val * store.scale_factor();
            child_space_x += val;
        }
        if let Pixels(val) = child_right {
            let val = val * store.scale_factor();
            child_space_x += val;
        }
        if let Pixels(val) = child_top {
            let val = val * store.scale_factor();
            child_space_y += val;
        }
        if let Pixels(val) = child_bottom {
            let val = val * store.scale_factor();
            child_space_y += val;
        }

        text_context.sync_styles(*self, store);
        let (mut text_width, mut text_height) = text_context.with_buffer(*self, |buffer| {
            buffer.set_size(width, i32::MAX);
            let w = buffer
                .layout_runs()
                .filter_map(|r| (!r.line_w.is_nan()).then_some(r.line_w))
                .max_by(|f1, f2| f1.partial_cmp(f2).unwrap())
                .unwrap_or_default();
            let h = buffer.layout_runs().len() as f32 * buffer.metrics().line_height as f32;
            (w, h)
        });

        text_width += child_space_x;
        text_height += child_space_y;

        let height = if let Some(height) = height { height } else { text_height };
        Some((text_width.max(width as f32), height))
    }

    fn height(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.height.get(*self).cloned().map(|h| match h {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn min_height(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.min_height.get(*self).cloned().map(|h| match h {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn max_height(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.max_height.get(*self).cloned().map(|h| match h {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn child_left(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.child_left.get(*self).cloned().map(|l| match l {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn child_right(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.child_right.get(*self).cloned().map(|r| match r {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn child_top(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.child_top.get(*self).cloned().map(|t| match t {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn child_bottom(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.child_bottom.get(*self).cloned().map(|b| match b {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn row_between(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.row_between.get(*self).cloned().map(|v| match v {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    fn col_between(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.col_between.get(*self).cloned().map(|v| match v {
            Units::Pixels(val) => Units::Pixels(store.logical_to_physical(val)),
            t => t,
        })
    }

    // fn border_left(&self, store: &Self::Store) -> Option<morphorm::Units> {
    //     store.border_width.get(*self).map(|border_width| match border_width {
    //         LengthOrPercentage::Length(val) => {
    //             Units::Pixels(store.logical_to_physical(val.to_px().unwrap_or_default()))
    //         }
    //         LengthOrPercentage::Percentage(val) => Units::Percentage(*val),
    //     })
    // }

    // fn border_right(&self, store: &Self::Store) -> Option<morphorm::Units> {
    //     store.border_width.get(*self).map(|border_width| match border_width {
    //         LengthOrPercentage::Length(val) => {
    //             Units::Pixels(store.logical_to_physical(val.to_px().unwrap_or_default()))
    //         }
    //         LengthOrPercentage::Percentage(val) => Units::Percentage(*val),
    //     })
    // }

    // fn border_top(&self, store: &Self::Store) -> Option<morphorm::Units> {
    //     store.border_width.get(*self).map(|border_width| match border_width {
    //         LengthOrPercentage::Length(val) => {
    //             Units::Pixels(store.logical_to_physical(val.to_px().unwrap_or_default()))
    //         }
    //         LengthOrPercentage::Percentage(val) => Units::Percentage(*val),
    //     })
    // }

    // fn border_bottom(&self, store: &Self::Store) -> Option<morphorm::Units> {
    //     store.border_width.get(*self).map(|border_width| match border_width {
    //         LengthOrPercentage::Length(val) => {
    //             Units::Pixels(store.logical_to_physical(val.to_px().unwrap_or_default()))
    //         }
    //         LengthOrPercentage::Percentage(val) => Units::Percentage(*val),
    //     })
    // }
}
