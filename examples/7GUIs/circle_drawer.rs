#![allow(dead_code)]
use vizia::prelude::*;
use vizia::vg::{Paint, PaintStyle, Path, Point};

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

const STYLE: &str = r#"

    circle-drawer {
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

#[derive(Clone, Copy, PartialEq)]
struct Circle {
    x: f32,
    y: f32,
    r: f32,
}

#[derive(Clone, Default, PartialEq)]
struct CircleData {
    circles: Vec<Circle>,
    selected: Option<usize>,
}

impl CircleData {
    fn add_circle(&mut self, circle: Circle) {
        self.selected = Some(self.circles.len());
        self.circles.push(circle);
    }

    fn change_radius(&mut self, r: f32) {
        if let Some(idx) = self.selected {
            self.circles[idx].r = r;
        }
    }

    fn update_selected(&mut self, x: f32, y: f32) {
        self.selected = self
            .circles
            .iter()
            .enumerate()
            .rev()
            .find(|(_, c)| distance(c.x, c.y, x, y) < c.r)
            .map(|(i, _)| i);
    }

    fn get_selected_radius(&self) -> Option<f32> {
        self.selected.map(|idx| self.circles[idx].r)
    }
}

#[derive(Clone, PartialEq)]
enum UndoRedoAction {
    Circle(Circle),
    RadiusChange(usize, f32),
}

#[derive(Default, PartialEq, Clone)]
struct UndoRedo {
    undo_list: Vec<UndoRedoAction>,
    redo_list: Vec<UndoRedoAction>,
}

impl UndoRedo {
    fn add_action(&mut self, action: UndoRedoAction) {
        self.undo_list.push(action);
        self.redo_list.clear(); // empty the redo list
    }

    fn undo(&mut self, circles: &mut Vec<Circle>) {
        let last = self.undo_list.pop().unwrap();

        match last {
            UndoRedoAction::Circle(_) => {
                self.redo_list.push(last);
                circles.pop(); // remove the last circle
            }
            UndoRedoAction::RadiusChange(idx, r) => {
                self.redo_list.push(UndoRedoAction::RadiusChange(
                    idx,
                    circles[idx].r, // store the current radius in redo list
                ));
                circles[idx].r = r; // update the radius to the old one
            }
        }
    }

    fn redo(&mut self, circles: &mut Vec<Circle>) {
        let last = self.redo_list.pop().unwrap();

        match last {
            UndoRedoAction::Circle(c) => {
                self.undo_list.push(last);
                circles.push(c);
            }
            UndoRedoAction::RadiusChange(idx, r) => {
                self.undo_list.push(UndoRedoAction::RadiusChange(
                    idx,
                    circles[idx].r, // store the current radius in undo list
                ));
                circles[idx].r = r; // restore the radius
            }
        }
    }
}

struct CircleDrawerData {
    circles_data: Signal<CircleData>,
    /// Undo redo
    undo_redo: Signal<UndoRedo>,
    radius_before: f32,
    /// is right click menu open
    menu_open: Signal<bool>,
    menu_posx: Signal<Units>,
    menu_posy: Signal<Units>,
    /// is dialog box open
    dialog_open: Signal<bool>,
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

impl Model for CircleDrawerData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|event, _| match event {
            CircleDrawerEvent::AddCircle(x, y) => {
                let circle =
                    Circle { x: cx.physical_to_logical(x), y: cx.physical_to_logical(y), r: 26.0 };
                self.circles_data.update(|cd| cd.add_circle(circle));
                self.undo_redo.update(|ur| ur.add_action(UndoRedoAction::Circle(circle)));
            }
            CircleDrawerEvent::TrySelectCircle(x, y) => {
                if !(self.dialog_open.get() || self.menu_open.get()) {
                    self.circles_data.update(|cd| {
                        cd.update_selected(cx.physical_to_logical(x), cx.physical_to_logical(y))
                    });
                }
            }
            CircleDrawerEvent::ChangeRadius(r) => {
                self.circles_data.update(|cd| cd.change_radius(r))
            }
            CircleDrawerEvent::Undo => self.undo_redo.update(|ur| {
                self.circles_data.update(|cd| {
                    ur.undo(&mut cd.circles);
                    if cd.selected.is_some_and(|idx| idx >= cd.circles.len()) {
                        cd.selected = cd.circles.len().checked_sub(1);
                    }
                });
            }),
            CircleDrawerEvent::Redo => self.undo_redo.update(|ur| {
                self.circles_data.update(|cd| {
                    ur.redo(&mut cd.circles);
                    if cd.selected.is_some_and(|idx| idx >= cd.circles.len()) {
                        cd.selected = cd.circles.len().checked_sub(1);
                    }
                });
            }),
            CircleDrawerEvent::ToggleRightMenu => {
                if !self.menu_open.get() && self.circles_data.get().selected.is_some() {
                    let (x, y) = cx.mouse().right.pos_down;

                    self.menu_open.set(true);
                    self.menu_posx.set(Pixels(cx.physical_to_logical(x)));
                    self.menu_posy.set(Pixels(cx.physical_to_logical(y)));
                } else {
                    self.menu_open.set(false);
                }

                if !self.dialog_open.get() {
                    let x = cx.physical_to_logical(cx.mouse().cursor_x);
                    let y = cx.physical_to_logical(cx.mouse().cursor_y);

                    self.circles_data.update(|cd| cd.update_selected(x, y));
                }
            }
            CircleDrawerEvent::ToggleDialog => {
                self.dialog_open ^= true;

                let radius = self.circles_data.get().get_selected_radius().unwrap();

                if self.dialog_open.get() {
                    // if dialog just opened save the current radius as before radius
                    self.radius_before = radius;
                } else {
                    if self.radius_before != radius {
                        self.undo_redo.update(|ur| {
                            ur.add_action(UndoRedoAction::RadiusChange(
                                self.circles_data.get().selected.unwrap(),
                                self.radius_before,
                            ));
                        });
                    }

                    let x = cx.physical_to_logical(cx.mouse().cursor_x);
                    let y = cx.physical_to_logical(cx.mouse().cursor_y);

                    self.circles_data.update(|cd| cd.update_selected(x, y));
                }
            }
        });
    }
}

struct CircleDrawerCanvas {
    circles_data: Signal<CircleData>,
}

impl CircleDrawerCanvas {
    fn new(cx: &mut Context, lens: Signal<CircleData>) -> Handle<'_, Self> {
        Self { circles_data: lens }
            .build(cx, |_| {})
            .bind(lens, |mut handle, _| handle.needs_redraw())
            .overflow(Overflow::Hidden)
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
                    let x = cx.mouse().cursor_x;
                    let y = cx.mouse().cursor_y;

                    cx.emit(CircleDrawerEvent::AddCircle(x, y))
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

        let circle_data = self.circles_data.get();
        for (idx, Circle { x, y, r }) in circle_data.circles.iter().copied().enumerate() {
            let path = Path::circle(
                Point::new(cx.logical_to_physical(x), cx.logical_to_physical(y)),
                r,
                None,
            );

            if circle_data.selected.is_some_and(|i| i == idx) {
                let mut paint = Paint::default();
                paint.set_color(Color::gray());
                paint.set_style(PaintStyle::Fill);
                canvas.draw_path(&path, &paint);
            }

            canvas.draw_path(&path, &paint);
        }
    }
}

struct CircleDrawer;

impl CircleDrawer {
    fn new(cx: &mut Context) -> Handle<'_, Self> {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let circles_data = Signal::new(CircleData::default());
        let undo_redo = Signal::new(UndoRedo::default());
        let menu_open = Signal::new(false);
        let menu_posx = Signal::new(Units::Pixels(0.0));
        let menu_posy = Signal::new(Units::Pixels(0.0));
        let dialog_open = Signal::new(false);

        CircleDrawerData {
            circles_data,
            undo_redo,
            radius_before: 0.0,
            menu_open,
            menu_posx,
            menu_posy,
            dialog_open,
        }
        .build(cx);

        Self.build(cx, |cx| {
            Binding::new(cx, menu_open, move |cx| {
                let is_open = menu_open.get();
                if is_open {
                    Popup::new(cx, |cx| {
                        Button::new(cx, |cx| Label::new(cx, "Adjust diameter..")).on_press(|cx| {
                            cx.emit(CircleDrawerEvent::ToggleDialog);
                            cx.emit(CircleDrawerEvent::ToggleRightMenu);
                        });
                    })
                    .left(menu_posx)
                    .top(menu_posy)
                    .size(Auto)
                    .on_blur(|cx| cx.emit(CircleDrawerEvent::ToggleRightMenu))
                    .lock_focus_to_within();
                }
            });

            #[cfg(not(feature = "baseview"))]
            Binding::new(cx, dialog_open, move |cx| {
                let is_open = dialog_open.get();
                if is_open {
                    Window::popup(cx, true, move |cx| {
                        let selected = circles_data.map(|cd| cd.selected).get().unwrap();

                        VStack::new(cx, move |cx| {
                            Label::new(
                                cx,
                                &format!(
                                    "Adjust diameter of circle at {:?}.",
                                    circles_data
                                        .map(move |cd| {
                                            let c = cd.circles[selected];
                                            (c.x, c.y)
                                        })
                                        .get()
                                ),
                            );

                            Slider::new(cx, circles_data.map(move |cd| cd.circles[selected].r))
                                .range(4.0..150.0)
                                .on_change(|cx, value| {
                                    cx.emit(CircleDrawerEvent::ChangeRadius(value))
                                })
                                .width(Percentage(80.0));
                        })
                        .alignment(Alignment::TopCenter)
                        .gap(Pixels(12.0))
                        .padding(Pixels(12.0));
                    })
                    .title("Adjust diameter..")
                    .inner_size((300, 50))
                    .position((500, 100))
                    .on_close(|cx| cx.emit(CircleDrawerEvent::ToggleDialog));
                }
            });

            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "Undo"))
                    .disabled(undo_redo.map(|v| v.undo_list.is_empty()))
                    .on_press(|cx| cx.emit(CircleDrawerEvent::Undo));

                Button::new(cx, |cx| Label::new(cx, "Redo"))
                    .disabled(undo_redo.map(|v| v.redo_list.is_empty()))
                    .on_press(|cx| cx.emit(CircleDrawerEvent::Redo));
            })
            .alignment(Alignment::Center)
            .gap(Pixels(12.0))
            .height(Auto);

            CircleDrawerCanvas::new(cx, circles_data);
        })
    }
}

impl View for CircleDrawer {
    fn element(&self) -> Option<&'static str> {
        Some("circle-drawer")
    }
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
    Application::new(|cx: &mut Context| {
        CircleDrawer::new(cx);
    })
    .run()
}
