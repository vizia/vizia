use femtovg::{Paint, Path};
use usvg::{NodeKind, PathSegment};

use crate::prelude::*;

/// A svg display used to display vector graphics to the screen.
///
/// # Examples
///
/// ## Basic Svg Display
///
/// A svg display can be used to simply load and display vector graphics on the screen.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// #
/// SvgDisplay::new(cx, SvgTree::from_str(include_str!("path/to/my/graphic.svg"), &SvgOptions::default()));
/// ```
pub struct SvgDisplay<L: Lens> {
    tree: L,
}

impl<L> SvgDisplay<L>
where
    L: Lens<Target = SvgTree>,
{
    /// Creates a new svg display.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// SvgDisplay::new(cx, SvgTree::from_str(include_str!("path/to/my/graphic.svg"), &SvgOptions::default()));
    /// ```
    pub fn new<'a>(cx: &'a mut Context, svg_tree: L) -> Handle<'a, Self> {
        Self { tree: svg_tree }.build(cx, |_| {})
    }
}

impl<L> View for SvgDisplay<L>
where
    L: Lens<Target = SvgTree>,
{
    fn element(&self) -> Option<&'static str> {
        Some("svg_display")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        for (path, fill, stroke) in &mut self.tree.get(cx).result {
            if let Some(fill) = fill {
                fill.set_anti_alias(true);
                canvas.fill_path(path, fill);
            }

            if let Some(stroke) = stroke {
                stroke.set_anti_alias(true);
                canvas.stroke_path(path, stroke);
            }
        }
    }
}

#[derive(Clone)]
pub struct SvgTree {
    result: Vec<(Path, Option<Paint>, Option<Paint>)>,
}

pub type SvgOptions = usvg::Options;

impl SvgTree {
    pub fn from_str(text: &str, options: &SvgOptions) -> SvgTree {
        let tree = usvg::Tree::from_str(text, options).unwrap();
        let mut paths = vec![];

        for node in tree.root.descendants() {
            if let NodeKind::Path(svg_path) = &*node.borrow() {
                let mut path = Path::new();

                for command in svg_path.data.segments() {
                    match command {
                        PathSegment::MoveTo { x, y } => path.move_to(x as f32, y as f32),
                        PathSegment::LineTo { x, y } => path.line_to(x as f32, y as f32),
                        PathSegment::CurveTo { x1, y1, x2, y2, x, y } => path.bezier_to(
                            x1 as f32, y1 as f32, x2 as f32, y2 as f32, x as f32, y as f32,
                        ),
                        PathSegment::ClosePath => path.close(),
                    }
                }

                #[inline]
                fn to_femto_color(paint: &usvg::Paint) -> Option<femtovg::Color> {
                    match paint {
                        usvg::Paint::Color(usvg::Color { red, green, blue }) => {
                            Some(femtovg::Color::rgb(*red, *green, *blue))
                        }
                        _ => None,
                    }
                }

                let fill = svg_path
                    .fill
                    .as_ref()
                    .and_then(|fill| to_femto_color(&fill.paint))
                    .map(Paint::color);

                let stroke = svg_path.stroke.as_ref().and_then(|stroke| {
                    to_femto_color(&stroke.paint).map(|paint| {
                        let mut stroke_paint = Paint::color(paint);
                        stroke_paint.set_line_width(stroke.width.get() as f32);
                        stroke_paint
                    })
                });

                paths.push((path, fill, stroke));
            }
        }

        Self { result: paths }
    }
}
