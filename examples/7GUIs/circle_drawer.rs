#![allow(dead_code)]
use vizia::prelude::*;
use vizia::vg::{Paint, PaintStyle, Path, Point};

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

const STYLE: &str = r#"

    .circle-drawer {
        padding: 12px;
        gap: 12px;
    }

    circle-drawer-canvas {
        border-color: black;
        border-width: 2px;
    }

    popup {
        min-width: 0px;
        border-width: 0px;
    }
"#;

fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    f32::sqrt(f32::powf(x1 - x2, 2.0) + f32::powf(y1 - y2, 2.0))
}

#[derive(Clone, Copy)]
struct Circle {
    x: f32,
    y: f32,
    r: f32,
}

#[derive(Clone)]
enum UndoRedoAction {
    Circle(Circle),
    RadiusChange(usize, f32),
}

struct CircleDrawerCanvas {
    circles: Signal<Vec<Circle>>,
    selected: Signal<Option<usize>>,
    // Cached for draw() since DrawContext doesn't impl DataContext
    circles_cache: Vec<Circle>,
    selected_cache: Option<usize>,
}

impl CircleDrawerCanvas {
    fn new(
        cx: &mut Context,
        circles: Signal<Vec<Circle>>,
        selected: Signal<Option<usize>>,
    ) -> Handle<'_, Self> {
        let overflow_hidden = cx.state(Overflow::Hidden);
        Self {
            circles,
            selected,
            circles_cache: circles.get(cx).clone(),
            selected_cache: *selected.get(cx),
        }
        .build(cx, |_| {})
        .bind(circles, |handle, c| {
            let cached = c.get(&handle).clone();
            handle.modify(|canvas| canvas.circles_cache = cached).needs_redraw();
        })
        .bind(selected, |handle, s| {
            let cached = *s.get(&handle);
            handle.modify(|canvas| canvas.selected_cache = cached).needs_redraw();
        })
        .overflow(overflow_hidden)
    }
}

impl View for CircleDrawerCanvas {
    fn element(&self) -> Option<&'static str> {
        Some("circle-drawer-canvas")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            WindowEvent::MouseDown(button) => match button {
                MouseButton::Left => {
                    cx.emit(CircleDrawerEvent::AddCircle(cx.mouse().cursor_x, cx.mouse().cursor_y));
                }
                MouseButton::Right => cx.emit(CircleDrawerEvent::ToggleRightMenu),
                _ => (),
            },
            WindowEvent::MouseMove(x, y) => cx.emit(CircleDrawerEvent::TrySelectCircle(*x, *y)),
            _ => (),
        })
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        cx.draw_border(canvas);
        let mut paint = Paint::default();

        paint.set_color(Color::black());
        paint.set_style(PaintStyle::Stroke);
        paint.set_stroke_width(2.0);
        paint.set_anti_alias(true);

        for (idx, Circle { x, y, r }) in self.circles_cache.iter().copied().enumerate() {
            let path = Path::circle(
                Point::new(cx.logical_to_physical(x), cx.logical_to_physical(y)),
                r,
                None,
            );

            if self.selected_cache.is_some_and(|i| i == idx) {
                let mut paint = Paint::default();
                paint.set_color(Color::gray());
                paint.set_style(PaintStyle::Fill);
                canvas.draw_path(&path, &paint);
            }

            canvas.draw_path(&path, &paint);
        }
    }
}

enum CircleDrawerEvent {
    AddCircle(f32, f32),
    TrySelectCircle(f32, f32),
    ChangeRadius(f32),
    Undo,
    Redo,
    ToggleRightMenu,
    ToggleDialog,
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
    CircleDrawerApp::run()
}

#[cfg(not(feature = "baseview"))]
struct CircleDrawerApp {
    circles: Signal<Vec<Circle>>,
    selected: Signal<Option<usize>>,
    undo_list: Signal<Vec<UndoRedoAction>>,
    redo_list: Signal<Vec<UndoRedoAction>>,
    radius_before: Signal<f32>,
    menu_open: Signal<bool>,
    menu_posx: Signal<Units>,
    menu_posy: Signal<Units>,
    dialog_open: Signal<bool>,
}

#[cfg(not(feature = "baseview"))]
impl CircleDrawerApp {
    fn update_selected(&self, cx: &mut EventContext, x: f32, y: f32) {
        let circles_list = self.circles.get(cx);
        let new_selected = circles_list
            .iter()
            .enumerate()
            .rev()
            .find(|(_, c)| distance(c.x, c.y, x, y) < c.r)
            .map(|(i, _)| i);
        self.selected.set(cx, new_selected);
    }
}

#[cfg(not(feature = "baseview"))]
impl App for CircleDrawerApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            circles: cx.state(Vec::<Circle>::new()),
            selected: cx.state(None::<usize>),
            undo_list: cx.state(Vec::<UndoRedoAction>::new()),
            redo_list: cx.state(Vec::<UndoRedoAction>::new()),
            radius_before: cx.state(0.0f32),
            menu_open: cx.state(false),
            menu_posx: cx.state(Pixels(0.0)),
            menu_posy: cx.state(Pixels(0.0)),
            dialog_open: cx.state(false),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let circles = self.circles;
        let selected = self.selected;
        let undo_list = self.undo_list;
        let redo_list = self.redo_list;
        let menu_open = self.menu_open;
        let menu_posx = self.menu_posx;
        let menu_posy = self.menu_posy;
        let dialog_open = self.dialog_open;

        let auto = cx.state(Auto);
        let gap_12 = cx.state(Pixels(12.0));
        let padding_12 = cx.state(Pixels(12.0));
        let align_center = cx.state(Alignment::Center);
        let align_top_center = cx.state(Alignment::TopCenter);
        let slider_width = cx.state(Percentage(80.0));
        let dialog_title = cx.state("Adjust diameter..");
        let dialog_size = cx.state((300, 50));
        let dialog_pos = cx.state((500, 100));

        let undo_disabled = cx.derived({
            let undo_list = undo_list;
            move |store| undo_list.get(store).is_empty()
        });
        let redo_disabled = cx.derived({
            let redo_list = redo_list;
            move |store| redo_list.get(store).is_empty()
        });

        VStack::new(cx, move |cx| {
            Binding::new(cx, menu_open, move |cx| {
                if *menu_open.get(cx) {
                    Popup::new(cx, |cx| {
                        Button::new(cx, |cx| Label::new(cx, "Adjust diameter..")).on_press(|cx| {
                            cx.emit(CircleDrawerEvent::ToggleDialog);
                            cx.emit(CircleDrawerEvent::ToggleRightMenu);
                        });
                    })
                    .left(menu_posx)
                    .top(menu_posy)
                    .size(auto)
                    .on_blur(|cx| cx.emit(CircleDrawerEvent::ToggleRightMenu))
                    .lock_focus_to_within();
                }
            });

            #[cfg(not(feature = "baseview"))]
            Binding::new(cx, dialog_open, move |cx| {
                if *dialog_open.get(cx) {
                    if let Some(sel) = *selected.get(cx) {
                        let circle_list = circles.get(cx);
                        let (cx_pos, cy_pos) =
                            circle_list.get(sel).map(|c| (c.x, c.y)).unwrap_or((0.0, 0.0));
                        let current_radius = circle_list.get(sel).map(|c| c.r).unwrap_or(26.0);

                        let slider_value = cx.state(current_radius);

                        Window::popup(cx, true, move |cx| {
                            let label_text = cx.state(format!(
                                "Adjust diameter of circle at ({:.0}, {:.0}).",
                                cx_pos, cy_pos
                            ));
                            VStack::new(cx, move |cx| {
                                Label::new(cx, label_text);

                                Slider::new(cx, slider_value)
                                    .range(4.0..150.0)
                                    .on_change(|cx, value| {
                                        cx.emit(CircleDrawerEvent::ChangeRadius(value))
                                    })
                                    .width(slider_width);
                            })
                            .alignment(align_top_center)
                            .gap(gap_12)
                            .padding(padding_12);
                        })
                        .title(dialog_title)
                        .inner_size(dialog_size)
                        .position(dialog_pos)
                        .on_close(|cx| cx.emit(CircleDrawerEvent::ToggleDialog));
                    }
                }
            });

            HStack::new(cx, move |cx| {
                Button::new(cx, |cx| Label::new(cx, "Undo"))
                    .disabled(undo_disabled)
                    .on_press(|cx| cx.emit(CircleDrawerEvent::Undo));

                Button::new(cx, |cx| Label::new(cx, "Redo"))
                    .disabled(redo_disabled)
                    .on_press(|cx| cx.emit(CircleDrawerEvent::Redo));
            })
            .alignment(align_center)
            .gap(gap_12)
            .height(auto);

            CircleDrawerCanvas::new(cx, circles, selected);
        })
        .class("circle-drawer");

        self
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|event, _| match event {
            CircleDrawerEvent::AddCircle(x, y) => {
                let circle =
                    Circle { x: cx.physical_to_logical(x), y: cx.physical_to_logical(y), r: 26.0 };
                self.circles.update(cx, |circles| {
                    circles.push(circle);
                });
                self.selected.set(cx, Some(self.circles.get(cx).len() - 1));
                self.undo_list.update(cx, |list| {
                    list.push(UndoRedoAction::Circle(circle));
                });
                self.redo_list.set(cx, Vec::new());
            }
            CircleDrawerEvent::TrySelectCircle(x, y) => {
                let dialog_open = *self.dialog_open.get(cx);
                let menu_open = *self.menu_open.get(cx);
                if !(dialog_open || menu_open) {
                    self.update_selected(cx, cx.physical_to_logical(x), cx.physical_to_logical(y));
                }
            }
            CircleDrawerEvent::ChangeRadius(r) => {
                if let Some(idx) = *self.selected.get(cx) {
                    self.circles.update(cx, |circles| {
                        circles[idx].r = r;
                    });
                }
            }
            CircleDrawerEvent::Undo => {
                if let Some(action) = self.undo_list.get(cx).last().cloned() {
                    self.undo_list.update(cx, |list| {
                        list.pop();
                    });
                    match action {
                        UndoRedoAction::Circle(_) => {
                            self.redo_list.update(cx, |list| list.push(action));
                            self.circles.update(cx, |circles| {
                                circles.pop();
                            });
                        }
                        UndoRedoAction::RadiusChange(idx, r) => {
                            let current_r = self.circles.get(cx)[idx].r;
                            self.redo_list.update(cx, |list| {
                                list.push(UndoRedoAction::RadiusChange(idx, current_r));
                            });
                            self.circles.update(cx, |circles| {
                                circles[idx].r = r;
                            });
                        }
                    }
                }
            }
            CircleDrawerEvent::Redo => {
                if let Some(action) = self.redo_list.get(cx).last().cloned() {
                    self.redo_list.update(cx, |list| {
                        list.pop();
                    });
                    match action {
                        UndoRedoAction::Circle(c) => {
                            self.undo_list.update(cx, |list| list.push(action));
                            self.circles.update(cx, |circles| {
                                circles.push(c);
                            });
                        }
                        UndoRedoAction::RadiusChange(idx, r) => {
                            let current_r = self.circles.get(cx)[idx].r;
                            self.undo_list.update(cx, |list| {
                                list.push(UndoRedoAction::RadiusChange(idx, current_r));
                            });
                            self.circles.update(cx, |circles| {
                                circles[idx].r = r;
                            });
                        }
                    }
                }
            }
            CircleDrawerEvent::ToggleRightMenu => {
                let menu_open_val = *self.menu_open.get(cx);
                let has_selected = self.selected.get(cx).is_some();

                if !menu_open_val && has_selected {
                    let (x, y) = cx.mouse().right.pos_down;
                    self.menu_open.set(cx, true);
                    self.menu_posx.set(cx, Pixels(cx.physical_to_logical(x)));
                    self.menu_posy.set(cx, Pixels(cx.physical_to_logical(y)));
                } else {
                    self.menu_open.set(cx, false);
                }

                if !*self.dialog_open.get(cx) {
                    let x = cx.physical_to_logical(cx.mouse().cursor_x);
                    let y = cx.physical_to_logical(cx.mouse().cursor_y);
                    self.update_selected(cx, x, y);
                }
            }
            CircleDrawerEvent::ToggleDialog => {
                let dialog_open_val = *self.dialog_open.get(cx);
                self.dialog_open.set(cx, !dialog_open_val);

                if let Some(idx) = *self.selected.get(cx) {
                    let radius = self.circles.get(cx)[idx].r;

                    if !dialog_open_val {
                        // Dialog just opened - save current radius
                        self.radius_before.set(cx, radius);
                    } else {
                        // Dialog closing
                        let radius_before_val = *self.radius_before.get(cx);
                        if radius_before_val != radius {
                            self.undo_list.update(cx, |list| {
                                list.push(UndoRedoAction::RadiusChange(idx, radius_before_val));
                            });
                        }

                        let x = cx.physical_to_logical(cx.mouse().cursor_x);
                        let y = cx.physical_to_logical(cx.mouse().cursor_y);
                        self.update_selected(cx, x, y);
                    }
                }
            }
        });
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| {
            app.title("Circle Drawer")
        })
    }
}