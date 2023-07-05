use morphorm::{Node, Units};
use vizia_storage::MorphormChildIter;

use crate::prelude::*;
use crate::resource::{ImageOrId, ResourceManager};
use crate::style::{ImageOrGradient, Style};
use crate::text::TextContext;

pub struct SubLayout<'a> {
    pub text_context: &'a mut TextContext,
    pub resource_manager: &'a ResourceManager,
}

impl Node for Entity {
    type Store = Style;
    type Tree = Tree<Entity>;
    type CacheKey = Entity;
    type ChildIter<'t> = MorphormChildIter<'t, Entity>;
    type SubLayout<'a> = SubLayout<'a>;

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

    fn content_size(
        &self,
        store: &Self::Store,
        sublayout: &mut Self::SubLayout<'_>,
        width: Option<f32>,
        height: Option<f32>,
    ) -> Option<(f32, f32)> {
        if sublayout.text_context.has_buffer(*self) {
            // If the width is known use that, else use 0 for wrapping text or 999999 for non-wrapping text.
            let max_width = if let Some(width) = width {
                let child_left =
                    store.child_left.get(*self).cloned().unwrap_or_default().to_px(width, 0.0)
                        * store.scale_factor();
                let child_right =
                    store.child_right.get(*self).cloned().unwrap_or_default().to_px(width, 0.0)
                        * store.scale_factor();
                (width.ceil() - child_left - child_right) as i32
            } else if store.text_wrap.get(*self).copied().unwrap_or(true) {
                0
            } else {
                999999
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

            sublayout.text_context.sync_styles(*self, store);
            let (text_width, mut text_height) =
                sublayout.text_context.with_buffer(*self, |fs, buffer| {
                    buffer.set_size(fs, max_width as f32, f32::MAX);
                    let w = buffer
                        .layout_runs()
                        .filter_map(|r| (!r.line_w.is_nan()).then_some(r.line_w))
                        .max_by(|f1, f2| f1.partial_cmp(f2).unwrap())
                        .unwrap_or_default();
                    let lines = buffer.layout_runs().filter(|run| run.line_w != 0.0).count();
                    let h = lines as f32 * buffer.metrics().line_height;
                    (w, h)
                });

            if height.is_none() {
                text_height = sublayout.text_context.with_buffer(*self, |fs, buffer| {
                    buffer.set_size(fs, text_width, f32::MAX);
                    let h = buffer.layout_runs().len() as f32 * buffer.metrics().line_height;
                    h
                });
            }

            let height =
                if let Some(height) = height { height } else { text_height + child_space_y };
            let width = if let Some(width) = width { width } else { text_width + child_space_x };

            // Cache the text_width/ text_height in the text context so we can use it to compute transforms later
            sublayout.text_context.set_bounds(
                *self,
                BoundingBox { w: text_width, h: text_height, ..Default::default() },
            );

            Some((width, height))
        } else if let Some(images) = store.background_image.get(*self) {
            let mut max_width = 0.0f32;
            let mut max_height = 0.0f32;
            for image in images.iter() {
                match image {
                    ImageOrGradient::Image(image_name) => {
                        if let Some(ImageOrId::Id(_, dim)) = sublayout
                            .resource_manager
                            .images
                            .get(image_name)
                            .map(|stored_img| &stored_img.image)
                        {
                            max_width = max_width.max(dim.0 as f32);
                            max_height = max_height.max(dim.1 as f32);
                        }
                    }
                    _ => {}
                }
            }

            let width = if let Some(width) = width { width } else { max_width };
            let height = if let Some(height) = height { height } else { max_height };

            Some((width, height))
        } else {
            None
        }
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

    fn border_left(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.border_width.get(*self).map(|border_width| match border_width {
            LengthOrPercentage::Length(val) => {
                Units::Pixels(store.logical_to_physical(val.to_px().unwrap_or_default()))
            }
            LengthOrPercentage::Percentage(val) => Units::Percentage(*val),
        })
    }

    fn border_right(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.border_width.get(*self).map(|border_width| match border_width {
            LengthOrPercentage::Length(val) => {
                Units::Pixels(store.logical_to_physical(val.to_px().unwrap_or_default()))
            }
            LengthOrPercentage::Percentage(val) => Units::Percentage(*val),
        })
    }

    fn border_top(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.border_width.get(*self).map(|border_width| match border_width {
            LengthOrPercentage::Length(val) => {
                Units::Pixels(store.logical_to_physical(val.to_px().unwrap_or_default()))
            }
            LengthOrPercentage::Percentage(val) => Units::Percentage(*val),
        })
    }

    fn border_bottom(&self, store: &Self::Store) -> Option<morphorm::Units> {
        store.border_width.get(*self).map(|border_width| match border_width {
            LengthOrPercentage::Length(val) => {
                Units::Pixels(store.logical_to_physical(val.to_px().unwrap_or_default()))
            }
            LengthOrPercentage::Percentage(val) => Units::Percentage(*val),
        })
    }
}
