use vizia::prelude::*;
use vizia_core::vg::{Paint, Path};

const STYLE: &str = r#"
    :root {
        child-space: 10px;
    }

    .header {
        left: 1s;
        right: 1s;
        col-between: 5px;
    }

    circle-drawer {
        row-between: 10px;
    }

    circle-drawer-canvas {
        border-color: black;
        border-width: 2px;
    }

    .dialog-box {
        child-space: 1s;
        width: 460px;
        height: 100px;
        top: 1s;
        left: 1s;
        right: 1s;
        bottom: 40px;
        background-color: rgba(255, 255, 255, 0.7);
        border-color: black;
        border-width: 1px;
        border-radius: 10%;
    }

    .dialog-box vstack {
        child-space: 1s;
        row-between: 15px;
    }

    .dialog-box vstack slider {
        width: 400px;
    }
"#;

fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    f32::sqrt(f32::powf(x1 - x2, 2.0) + f32::powf(y1 - y2, 2.0))
}

#[derive(Clone, Copy, Data)]
struct Circle {
    x: f32,
    y: f32,
    r: f32,
}

#[derive(Clone, Default, Data, Lens)]
struct CircleData {
    /// circles
    circles: Vec<Circle>,
    /// index of current selected circle
    selected: Option<usize>,
}

enum UndoRedoAction {
    Circle(Circle),
    RadiusChange(usize, f32),
}

#[derive(Lens)]
struct CircleDrawerData {
    circles_data: CircleData,
    /// Undo redo
    undo_list: Vec<UndoRedoAction>,
    redo_list: Vec<UndoRedoAction>,
    radius_before: f32,
    /// is right click menu open
    menu_open: bool,
    menu_posx: Units,
    menu_posy: Units,
    /// is dialog box open
    dialog_open: bool,
}

impl Default for CircleDrawerData {
    fn default() -> Self {
        Self {
            circles_data: CircleData::default(),
            undo_list: Vec::new(),
            redo_list: Vec::new(),
            radius_before: 0.0,
            menu_open: false,
            menu_posx: Default::default(),
            menu_posy: Default::default(),
            dialog_open: false,
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

impl CircleDrawerData {
    fn update_selected(&mut self, x: f32, y: f32) {
        self.circles_data.selected =
            self.circles_data.circles.iter().position(|c| distance(c.x, c.y, x, y) < c.r);
    }
}

impl Model for CircleDrawerData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        if let Some(event) = event.take() {
            match event {
                CircleDrawerEvent::AddCircle(x, y) => {
                    let circle = Circle { x, y, r: 26.0 };
                    self.circles_data.selected = Some(self.circles_data.circles.len());
                    self.circles_data.circles.push(circle);
                    self.undo_list.push(UndoRedoAction::Circle(circle));
                    self.redo_list.clear(); // we need to clear redo list in each new event
                }
                CircleDrawerEvent::TrySelectCircle(x, y) => {
                    if !(self.dialog_open || self.menu_open) {
                        self.update_selected(x, y)
                    }
                }
                CircleDrawerEvent::ChangeRadius(r) => {
                    if let Some(idx) = self.circles_data.selected {
                        self.circles_data.circles[idx].r = r;
                    }
                }
                CircleDrawerEvent::Undo => {
                    let last = self.undo_list.remove(self.undo_list.len() - 1);

                    match last {
                        UndoRedoAction::Circle(_) => {
                            self.redo_list.push(last);
                            self.circles_data.circles.pop(); // remove the last circle
                        }
                        UndoRedoAction::RadiusChange(idx, r) => {
                            self.redo_list.push(UndoRedoAction::RadiusChange(
                                idx,
                                self.circles_data.circles[idx].r, // store the current radius in redo list
                            ));
                            self.circles_data.circles[idx].r = r // update the radius to the old one
                        }
                    }
                }
                CircleDrawerEvent::Redo => {
                    let last = self.redo_list.remove(self.redo_list.len() - 1);

                    match last {
                        UndoRedoAction::Circle(c) => {
                            self.undo_list.push(last);
                            self.circles_data.circles.push(c);
                        }
                        UndoRedoAction::RadiusChange(idx, r) => {
                            self.undo_list.push(UndoRedoAction::RadiusChange(
                                idx,
                                self.circles_data.circles[idx].r, // store the current radius in undo list
                            ));
                            self.circles_data.circles[idx].r = r // update the radius
                        }
                    }
                }
                CircleDrawerEvent::ToggleRightMenu => {
                    let x = cx.mouse().cursorx;
                    let y = cx.mouse().cursory;

                    if !self.menu_open && self.circles_data.selected.is_some() {
                        self.menu_open = true;
                        self.menu_posx = Pixels(x);
                        self.menu_posy = Pixels(y);
                    } else {
                        self.menu_open = false;
                    }

                    if !self.dialog_open {
                        self.update_selected(x, y);
                    }
                }
                CircleDrawerEvent::ToggleDialog => {
                    self.dialog_open ^= true;

                    let radius = self.circles_data.circles[self.circles_data.selected.unwrap()].r;

                    if self.dialog_open {
                        // if dialog just opened save the current radius as before radius
                        self.radius_before = radius;
                    } else {
                        if self.radius_before != radius {
                            self.undo_list.push(UndoRedoAction::RadiusChange(
                                self.circles_data.selected.unwrap(),
                                self.radius_before,
                            ));
                            self.redo_list.clear(); // we need to clear redo list in each new event
                        }

                        let x = cx.mouse().cursorx;
                        let y = cx.mouse().cursory;

                        self.update_selected(x, y);
                    }
                }
            }
        }
    }
}

struct CircleDrawerCanvas;

impl CircleDrawerCanvas {
    fn new(cx: &mut Context, lens: impl Lens<Target = CircleData>) -> Handle<Self> {
        Self.build(cx, |cx| {
            Binding::new(cx, lens, |cx, _| cx.needs_redraw());
        })
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
                    let x = cx.mouse().cursorx;
                    let y = cx.mouse().cursory;

                    cx.emit(CircleDrawerEvent::AddCircle(x, y))
                }
                MouseButton::Right => cx.emit(CircleDrawerEvent::ToggleRightMenu),
                _ => (),
            },
            WindowEvent::MouseMove(x, y) => cx.emit(CircleDrawerEvent::TrySelectCircle(*x, *y)),
            _ => (),
        })
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let mut path = cx.build_path();
        cx.draw_border(canvas, &mut path);

        let paint = Paint::color(Color::black().into()).with_line_width(2.0);

        for (idx, Circle { x, y, r }) in
            CircleDrawerData::circles_data.get(cx).circles.into_iter().enumerate()
        {
            let mut path = Path::new();
            path.circle(x, y, r);

            if CircleDrawerData::circles_data.get(cx).selected.is_some_and(|i| i == idx) {
                let paint = Paint::color(Color::gray().into());
                canvas.fill_path(&path, &paint);
            }

            canvas.stroke_path(&path, &paint);
        }
    }
}

struct CircleDrawer;

impl CircleDrawer {
    fn new(cx: &mut Context) -> Handle<Self> {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        CircleDrawerData::default().build(cx);

        Self.build(cx, |cx| {
            Popup::new(cx, CircleDrawerData::menu_open, true, |cx| {
                Button::new(
                    cx,
                    |cx| {
                        cx.emit(CircleDrawerEvent::ToggleDialog);
                        cx.emit(CircleDrawerEvent::ToggleRightMenu);
                    },
                    |cx| Label::new(cx, "Adjust diameter.."),
                );
            })
            .size(Auto)
            .left(CircleDrawerData::menu_posx)
            .top(CircleDrawerData::menu_posy)
            .on_blur(|cx| cx.emit(CircleDrawerEvent::ToggleRightMenu));

            Popup::new(cx, CircleDrawerData::dialog_open, true, |cx| {
                let selected =
                    CircleDrawerData::circles_data.then(CircleData::selected).get(cx).unwrap();

                VStack::new(cx, |cx| {
                    Label::new(
                        cx,
                        &format!(
                            "Adjust diameter of circle at {:?}.",
                            CircleDrawerData::circles_data
                                .then(CircleData::circles)
                                .get(cx)
                                .get(selected)
                                .map(|c| (c.x, c.y))
                                .unwrap()
                        ),
                    );

                    Slider::new(
                        cx,
                        CircleDrawerData::circles_data
                            .then(CircleData::circles)
                            .index(selected)
                            .map(|c| c.r),
                    )
                    .range(4.0..150.0)
                    .on_changing(|cx, value| cx.emit(CircleDrawerEvent::ChangeRadius(value)));
                });
            })
            .class("dialog-box")
            .on_blur(|cx| cx.emit(CircleDrawerEvent::ToggleDialog));

            HStack::new(cx, |cx| {
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Undo"))
                    .disabled(CircleDrawerData::undo_list.map(|v| v.is_empty()))
                    .on_press(|cx| cx.emit(CircleDrawerEvent::Undo));

                Button::new(cx, |_| {}, |cx| Label::new(cx, "Redo"))
                    .disabled(CircleDrawerData::redo_list.map(|v| v.is_empty()))
                    .on_press(|cx| cx.emit(CircleDrawerEvent::Redo));
            })
            .size(Auto)
            .class("header");

            CircleDrawerCanvas::new(cx, CircleDrawerData::circles_data);
        })
    }
}

impl View for CircleDrawer {
    fn element(&self) -> Option<&'static str> {
        Some("circle-drawer")
    }
}

fn main() {
    Application::new(|cx: &mut Context| {
        CircleDrawer::new(cx);
    })
    .title("Circle drawer")
    .run()
}
