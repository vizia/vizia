use std::cell::RefCell;
use std::fmt::format;
use std::marker::PhantomData;

use femtovg::{Align, Baseline, FontId, Paint, Path};
use morphorm::{LayoutType, PositionType, Units};
use vizia_storage::LayoutChildIterator;

use crate::prelude::*;
use crate::style::StyleRule;
use crate::systems::compute_matched_rules;
use crate::vg;

#[derive(Lens)]
pub(crate) struct InspectorData {
    canvas: Entity,
    overlay: Entity,
    selected: Entity,

    sub_tree: Vec<(Entity, u32)>,
    styles: Vec<StyleRule>,
}

impl InspectorData {
    fn build_tree(&mut self, cx: &EventContext, parent: Entity, level: u32) {
        if parent != self.canvas
            && parent != self.overlay
            && cx.cache.get_display(parent) != Display::None
            && cx.style.display.get(parent).cloned().unwrap_or_default() != Display::None
        {
            self.sub_tree.push((parent, level));
        }
        let child_iter = LayoutChildIterator::new(&cx.tree, parent);
        for child in child_iter {
            self.build_tree(cx, child, level + 1);
        }
    }
}

#[derive(Debug)]
pub enum InspectorEvent {
    SetRoot(Entity),
    SetOverlay(Entity),
    SetSelected(Entity),
}

impl Model for InspectorData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|inspector_event, _| match inspector_event {
            InspectorEvent::SetRoot(root) => {
                self.canvas = *root;
                self.build_tree(cx, self.canvas, 0);
            }

            InspectorEvent::SetOverlay(overlay) => {
                self.overlay = *overlay;
            }

            InspectorEvent::SetSelected(selected) => {
                if selected.is_descendant_of(&cx.tree, self.canvas) && *selected != self.overlay {
                    self.selected = *selected;
                }
                let mut result = vec![];
                compute_matched_rules(&cx.style, &cx.views, &cx.tree, self.selected, &mut result);
                let append = self.styles.len() == result.len();
                self.styles = result.into_iter().map(|item| item.clone()).collect();
                if append {
                    self.styles.push(StyleRule::default());
                }
            }
        });
    }
}

pub struct Inspector {}

impl Inspector {
    pub fn new<F>(cx: &mut Context, builder: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context),
    {
        Self {}.build(cx, move |cx| {
            if cx.data::<InspectorData>().is_none() {
                InspectorData {
                    canvas: Entity::null(),
                    selected: Entity::null(),
                    overlay: Entity::null(),
                    sub_tree: Vec::new(),
                    styles: Vec::new(),
                }
                .build(cx);
            }

            HStack::new(cx, move |cx| {
                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Elements");
                        Label::new(cx, "Fonts");
                    })
                    .height(Pixels(30.0));
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        List::new(cx, InspectorData::sub_tree, |cx, _, item| {
                            Binding::new(cx, InspectorData::selected, move |cx, selected| {
                                let (entity, level) = item.get(cx);
                                let selected_color = if selected.get(cx) == entity {
                                    Color::rgb(120, 120, 120)
                                } else {
                                    Color::rgb(150, 150, 150)
                                };

                                let name = cx
                                    .views
                                    .get(&entity)
                                    .map(|view| view.element())
                                    .flatten()
                                    .unwrap_or("unknown");
                                let name = format!("{} ({})", name, entity);
                                Label::new(cx, &name)
                                    .width(Stretch(1.0))
                                    .height(Pixels(30.0))
                                    .background_color(selected_color)
                                    .child_space(Stretch(1.0))
                                    .child_left(Pixels(20.0 * level as f32))
                                    .on_press(move |cx| {
                                        cx.emit(InspectorEvent::SetSelected(entity))
                                    });
                            });
                        })
                        .width(Stretch(1.0))
                        .height(Auto)
                        .background_color(Color::rgb(150, 150, 150));
                    })
                    .top(Pixels(0.0))
                    .bottom(Pixels(0.0))
                    .child_top(Pixels(0.0))
                    .child_bottom(Pixels(0.0))
                    .width(Stretch(1.0))
                    .height(Stretch(1.0));
                })
                .width(Pixels(300.0))
                .height(Stretch(1.0));

                VStack::new(cx, move |cx| {
                    (builder)(cx);
                    Overlay::new(cx)
                        .z_order(100)
                        .overflow(Overflow::Visible)
                        .position_type(PositionType::SelfDirected);
                    cx.emit(InspectorEvent::SetRoot(cx.current));
                })
                .background_color(Color::rgb(200, 200, 200))
                .space(Pixels(50.0))
                .size(Stretch(1.0));

                VStack::new(cx, |cx| {
                    List::new(cx, InspectorData::styles, |cx, index, item| {
                        if !item.get(cx).properties.is_empty() {
                            Label::new(cx, &format!("{}", item.get(cx))).child_space(Pixels(10.0));
                        }
                    });
                })
                .width(Pixels(300.0))
                .background_color(Color::rgb(150, 150, 150));
            });
        })
    }
}

impl View for Inspector {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                cx.emit(InspectorEvent::SetSelected(cx.hovered()));
            }

            _ => {}
        });
    }
}

pub struct Overlay {
    // TODO: Replace with cosmic-text rendering
    font: RefCell<Option<FontId>>,
}

impl Overlay {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self { font: RefCell::new(None) }
            .build(cx, |cx| {
                cx.emit(InspectorEvent::SetOverlay(cx.current));
            })
            .hoverable(false)
    }
}

impl View for Overlay {
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        // println!("current: {:?}", cx.cache.get_bounds(cx.current));
        // let overlay_bounds = cx.cache.get_bounds(cx.current);
        // let mut path = Path::new();
        // path.rect(overlay_bounds.x + 1.0, overlay_bounds.y + 1.0, overlay_bounds.w - 2.0, overlay_bounds.h - 2.0);
        // let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 255));
        // paint.set_line_width(2.0);
        // canvas.stroke_path(&mut path, paint);

        let mut inspector_bounds = cx.bounds();
        inspector_bounds.x -= 50.0;

        let font_id = if self.font.borrow().is_none() {
            let loaded_font = canvas
                .add_font_mem(include_bytes!("../../resources/fonts/Roboto-Regular.ttf"))
                .expect("Faled");
            *self.font.borrow_mut() = Some(loaded_font.clone());
            loaded_font
        } else {
            (*self.font.borrow()).unwrap()
        };

        if let Some(inspector_data) = cx.data::<InspectorData>() {
            if inspector_data.selected != Entity::null() {
                let parent = inspector_data.selected.parent(&cx.tree).unwrap();

                // Draw outline
                let bounds = cx.cache.get_bounds(inspector_data.selected);
                let mut path = Path::new();
                path.rect(bounds.x + 1.0, bounds.y + 1.0, bounds.w - 2.0, bounds.h - 2.0);
                let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 255));
                paint.set_line_width(2.0);
                canvas.stroke_path(&mut path, &paint);

                let child_top = cx.style.child_top.get(parent).cloned().unwrap_or_default();
                let child_bottom = cx.style.child_bottom.get(parent).cloned().unwrap_or_default();
                let child_left = cx.style.child_left.get(parent).cloned().unwrap_or_default();
                let child_right = cx.style.child_left.get(parent).cloned().unwrap_or_default();
                let parent_layout_type =
                    cx.style.layout_type.get(parent).cloned().unwrap_or_default();
                for (_, child) in parent.child_iter(&cx.tree).enumerate() {
                    if child == cx.current {
                        continue;
                    }

                    let height = cx.style.height.get(child).cloned().unwrap_or_default();
                    let width = cx.style.width.get(child).cloned().unwrap_or_default();

                    let mut top = cx.style.top.get(child).cloned().unwrap_or_default();
                    if top == Units::Auto {
                        top = child_top;
                    };

                    let mut bottom = cx.style.bottom.get(child).cloned().unwrap_or_default();
                    if bottom == Units::Auto {
                        bottom = child_bottom;
                    };

                    let mut left = cx.style.left.get(child).cloned().unwrap_or_default();
                    if left == Units::Auto {
                        left = child_left;
                    };

                    let mut right = cx.style.right.get(child).cloned().unwrap_or_default();
                    if right == Units::Auto {
                        right = child_right;
                    };

                    let space = cx.cache.space.get(child).unwrap();

                    let bounds = cx.cache.get_bounds(child);

                    if child == inspector_data.selected {
                        draw_vertical_marker(
                            inspector_bounds.x,
                            bounds.y,
                            bounds.h,
                            font_id,
                            true,
                            canvas,
                            &print_units(height),
                        );

                        if space.top != 0.0 {
                            draw_vertical_marker(
                                inspector_bounds.x,
                                bounds.y - space.top,
                                space.top,
                                font_id,
                                true,
                                canvas,
                                &print_units(top),
                            );
                        }

                        if space.bottom != 0.0 {
                            draw_vertical_marker(
                                inspector_bounds.x,
                                bounds.y + bounds.h,
                                space.bottom,
                                font_id,
                                true,
                                canvas,
                                &print_units(bottom),
                            );
                        }

                        draw_horizontal_marker(
                            bounds.x,
                            bounds.w,
                            font_id,
                            true,
                            canvas,
                            &print_units(width),
                        );

                        if space.left != 0.0 {
                            draw_horizontal_marker(
                                bounds.x - space.left,
                                space.left,
                                font_id,
                                true,
                                canvas,
                                &print_units(left),
                            );
                        }

                        if space.right != 0.0 {
                            draw_horizontal_marker(
                                bounds.x + bounds.w,
                                space.right,
                                font_id,
                                true,
                                canvas,
                                &print_units(right),
                            );
                        }
                    } else {
                        if parent_layout_type == LayoutType::Column {
                            draw_vertical_marker(
                                inspector_bounds.x,
                                bounds.y,
                                bounds.h,
                                font_id,
                                false,
                                canvas,
                                &print_units(height),
                            );
                            if space.top != 0.0 {
                                draw_vertical_marker(
                                    inspector_bounds.x,
                                    bounds.y - space.top,
                                    space.top,
                                    font_id,
                                    false,
                                    canvas,
                                    &print_units(top),
                                );
                            }

                            if space.bottom != 0.0 {
                                draw_vertical_marker(
                                    inspector_bounds.x,
                                    bounds.y + bounds.h,
                                    space.bottom,
                                    font_id,
                                    false,
                                    canvas,
                                    &print_units(bottom),
                                );
                            }
                        } else if parent_layout_type == LayoutType::Row {
                            draw_horizontal_marker(
                                bounds.x,
                                bounds.w,
                                font_id,
                                false,
                                canvas,
                                &print_units(width),
                            );

                            if space.left != 0.0 {
                                draw_horizontal_marker(
                                    bounds.x - space.left,
                                    space.left,
                                    font_id,
                                    false,
                                    canvas,
                                    &print_units(left),
                                );
                            }

                            if space.right != 0.0 {
                                draw_horizontal_marker(
                                    bounds.x + bounds.w,
                                    space.right,
                                    font_id,
                                    false,
                                    canvas,
                                    &print_units(right),
                                );
                            }
                        }
                    }

                    // // Draw line
                    // let mut path = Path::new();
                    // path.move_to(bounds.x - 30.0, bounds.y);
                    // path.line_to(bounds.x - 20.0, bounds.y);
                    // path.move_to(bounds.x - 25.0, bounds.y);
                    // path.line_to(bounds.x - 25.0, bounds.y + bounds.h);
                    // path.move_to(bounds.x - 30.0, bounds.y + bounds.h);
                    // path.line_to(bounds.x - 20.0, bounds.y + bounds.h);
                    // let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 255));
                    // paint.set_line_width(2.0);
                    // canvas.stroke_path(&mut path, paint);

                    // // Draw box
                    // let mut path = Path::new();
                    // path.rounded_rect(bounds.x - 40.0, bounds.y + bounds.h/2.0 - 10.0, 30.0, 20.0, 1.0);
                    // let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 255));
                    // canvas.fill_path(&mut path, paint);

                    // // Draw text
                    // let mut text_paint = Paint::color(femtovg::Color::rgb(50, 50, 50));
                    // text_paint.set_text_align(Align::Center);
                    // text_paint.set_text_baseline(Baseline::Middle);
                    // text_paint.set_font(&[*font_id]);
                    // text_paint.set_font_size(12.0);
                    // canvas.fill_text(bounds.x - 24.5, bounds.y + bounds.h/2.0 + 0.5, &bounds.h.to_string(), text_paint);

                    // let mut text_paint = Paint::color(femtovg::Color::rgb(255, 255, 255));
                    // text_paint.set_text_align(Align::Center);
                    // text_paint.set_text_baseline(Baseline::Middle);
                    // text_paint.set_font(&[*font_id]);
                    // text_paint.set_font_size(12.0);
                    // canvas.fill_text(bounds.x - 25.0, bounds.y + bounds.h/2.0, &bounds.h.to_string(), text_paint);
                }
            }
        }
    }
}

fn draw_vertical_marker(
    x: f32,
    y: f32,
    h: f32,
    font_id: FontId,
    filled: bool,
    canvas: &mut Canvas,
    text: &str,
) {
    // Draw line
    let mut path = Path::new();
    path.move_to(x + 20.0, y);
    path.line_to(x + 30.0, y);
    path.move_to(x + 25.0, y);
    path.line_to(x + 25.0, y + h);
    path.move_to(x + 20.0, y + h);
    path.line_to(x + 30.0, y + h);
    let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 255));
    paint.set_line_width(2.0);
    canvas.stroke_path(&mut path, &paint);

    // Draw box
    let mut path = Path::new();
    path.rounded_rect(x + 10.0, y + h / 2.0 - 10.0, 30.0, 20.0, 1.0);
    if filled {
        canvas.fill_path(&mut path, &vg::Paint::color(femtovg::Color::rgb(255, 0, 255)));
    } else {
        canvas.fill_path(&mut path, &vg::Paint::color(femtovg::Color::rgb(255, 255, 255)));
    }
    let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 255));
    paint.set_line_width(2.0);
    canvas.stroke_path(&mut path, &paint);

    // Draw text
    let mut text_paint = Paint::color(femtovg::Color::rgb(0, 0, 0));
    text_paint.set_text_align(Align::Center);
    text_paint.set_text_baseline(Baseline::Middle);
    text_paint.set_font(&[font_id]);
    text_paint.set_font_size(12.0);
    canvas.fill_text(x + 25.5, y + h / 2.0 + 0.5, text, &text_paint).unwrap();
}

fn draw_horizontal_marker(
    x: f32,
    w: f32,
    font_id: FontId,
    filled: bool,
    canvas: &mut Canvas,
    text: &str,
) {
    // Draw line
    let mut path = Path::new();
    path.move_to(x, 20.0);
    path.line_to(x, 30.0);
    path.move_to(x, 25.0);
    path.line_to(x + w, 25.0);
    path.move_to(x + w, 20.0);
    path.line_to(x + w, 30.0);
    let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 255));
    paint.set_line_width(2.0);
    canvas.stroke_path(&mut path, &paint);

    // Draw box
    let mut path = Path::new();
    path.rounded_rect(x + w / 2.0 - 15.0, 15.0, 30.0, 20.0, 1.0);
    if filled {
        canvas.fill_path(&mut path, &vg::Paint::color(femtovg::Color::rgb(255, 0, 255)));
    } else {
        canvas.fill_path(&mut path, &vg::Paint::color(femtovg::Color::rgb(255, 255, 255)));
    }
    let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 255));
    paint.set_line_width(2.0);
    canvas.stroke_path(&mut path, &paint);

    // Draw text
    let mut text_paint = Paint::color(femtovg::Color::rgb(0, 0, 0));
    text_paint.set_text_align(Align::Center);
    text_paint.set_text_baseline(Baseline::Middle);
    text_paint.set_font(&[font_id]);
    text_paint.set_font_size(12.0);
    canvas.fill_text(x + w / 2.0, 25.0, text, &text_paint).unwrap();
}

pub fn print_units(units: Units) -> String {
    match units {
        Pixels(val) => format!("{}", val),
        Percentage(val) => format!("{}%", val),
        Stretch(val) => format!("{}s", val),
        Auto => "auto".to_string(),
    }
}
