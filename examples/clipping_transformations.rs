use vizia::prelude::*;

const OVERFLOW_OPTIONS: [&str; 2] = ["Visible", "Hidden"];
const CLIP_OPTIONS: [&str; 2] = ["Auto", "Inset"];

#[derive(Debug, Clone, Copy, PartialEq, Data)]
enum OverflowMode {
    Visible,
    Hidden,
}

impl From<OverflowMode> for Overflow {
    fn from(value: OverflowMode) -> Self {
        match value {
            OverflowMode::Visible => Overflow::Visible,
            OverflowMode::Hidden => Overflow::Hidden,
        }
    }
}

fn overflow_mode_from_index(index: usize) -> OverflowMode {
    match index {
        1 => OverflowMode::Hidden,
        _ => OverflowMode::Visible,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Data)]
enum ClipMode {
    Auto,
    Inset(f32),
}

impl From<ClipMode> for ClipPath {
    fn from(value: ClipMode) -> Self {
        match value {
            ClipMode::Auto => ClipPath::Auto,
            ClipMode::Inset(inset) => LengthOrPercentage::from(Pixels(inset.max(0.0))).into(),
        }
    }
}

fn clip_inset_from_state(state: &NodeState) -> ClipMode {
    match state.clip_shape_index {
        1 => ClipMode::Inset(state.inset),
        _ => ClipMode::Auto,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
    x: f32,
    y: f32,
}

impl Pos {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn translate(self) -> Translate {
        Translate::new(Pixels(self.x), Pixels(self.y))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct NodeState {
    overflow_x_index: usize,
    overflow_y_index: usize,
    clip_shape_index: usize,
    inset: f32,
    border_width: f32,
    pos: Pos,
    rotate_deg: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum NodeKind {
    Parent,
    Child,
    Grandchild,
}

#[derive(Debug, Lens)]
struct AppData {
    overflow_options: Vec<&'static str>,
    clip_options: Vec<&'static str>,
    parent_open: bool,
    child_open: bool,
    grandchild_open: bool,
    parent: NodeState,
    child: NodeState,
    grandchild: NodeState,
}

impl AppData {
    fn node_mut(&mut self, node: NodeKind) -> &mut NodeState {
        match node {
            NodeKind::Parent => &mut self.parent,
            NodeKind::Child => &mut self.child,
            NodeKind::Grandchild => &mut self.grandchild,
        }
    }
}

enum AppEvent {
    ToggleCollapse(NodeKind),
    SetOverflowX(NodeKind, usize),
    SetOverflowY(NodeKind, usize),
    SetClipShape(NodeKind, usize),
    SetInset(NodeKind, f32),
    SetBorderWidth(NodeKind, f32),
    SetTranslateX(NodeKind, f32),
    SetTranslateY(NodeKind, f32),
    SetRotation(NodeKind, f32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleCollapse(node) => match node {
                NodeKind::Parent => self.parent_open = !self.parent_open,
                NodeKind::Child => self.child_open = !self.child_open,
                NodeKind::Grandchild => self.grandchild_open = !self.grandchild_open,
            },
            AppEvent::SetOverflowX(node, value) => self.node_mut(*node).overflow_x_index = *value,
            AppEvent::SetOverflowY(node, value) => self.node_mut(*node).overflow_y_index = *value,
            AppEvent::SetClipShape(node, value) => self.node_mut(*node).clip_shape_index = *value,
            AppEvent::SetInset(node, value) => self.node_mut(*node).inset = *value,
            AppEvent::SetBorderWidth(node, value) => self.node_mut(*node).border_width = *value,
            AppEvent::SetTranslateX(node, value) => self.node_mut(*node).pos.x = *value,
            AppEvent::SetTranslateY(node, value) => self.node_mut(*node).pos.y = *value,
            AppEvent::SetRotation(node, value) => {
                println!("Setting rotation of {:?} to {} degrees", node, value);
                self.node_mut(*node).rotate_deg = *value;
            }
        });
    }
}

fn label_cell(cx: &mut Context, text: &str) {
    Label::new(cx, text).width(Pixels(88.0)).font_size(12.0).color(Color::rgb(175, 181, 194));
}

fn slider_row<LV, LF, FE>(
    cx: &mut Context,
    label: &str,
    lens_value: LV,
    lens_fmt: LF,
    min: f32,
    max: f32,
    suffix: &'static str,
    emit: FE,
) where
    LV: 'static + Lens<Target = f32>,
    LF: 'static + Lens<Target = f32>,
    FE: 'static + Fn(&mut EventContext, f32),
{
    HStack::new(cx, |cx| {
        label_cell(cx, label);

        Slider::new(cx, lens_value)
            .range(min..max)
            .on_change(move |cx, value| emit(cx, value))
            .width(Stretch(1.0));

        Label::new(cx, lens_fmt.map(move |&value| format!("{:+.0}{}", value, suffix)))
            .width(Pixels(56.0))
            .font_size(12.0)
            .color(Color::rgb(104, 208, 215))
            .alignment(Alignment::Right);
    })
    .height(Auto)
    .alignment(Alignment::Center)
    .horizontal_gap(Pixels(10.0));
}

fn pick_row<L, FE>(
    cx: &mut Context,
    label: &str,
    options: impl Lens<Target = Vec<&'static str>> + Clone,
    selected: L,
    emit: FE,
) where
    L: 'static + Lens<Target = usize>,
    FE: 'static + Fn(&mut EventContext, usize),
{
    HStack::new(cx, |cx| {
        label_cell(cx, label);

        PickList::new(cx, options, selected, true)
            .on_select(move |cx, index| emit(cx, index))
            .width(Stretch(1.0));
    })
    .height(Auto)
    .alignment(Alignment::Center)
    .horizontal_gap(Pixels(10.0));
}

fn controls_block(
    cx: &mut Context,
    title: &str,
    node: NodeKind,
    open_lens: impl Lens<Target = bool> + Clone + 'static,
    overflow_x_lens: impl Lens<Target = usize> + Clone + 'static,
    overflow_y_lens: impl Lens<Target = usize> + Clone + 'static,
    clip_lens: impl Lens<Target = usize> + Clone + 'static,
    inset_lens: impl Lens<Target = f32> + Clone + 'static,
    border_lens: impl Lens<Target = f32> + Clone + 'static,
    tx_lens: impl Lens<Target = f32> + Clone + 'static,
    ty_lens: impl Lens<Target = f32> + Clone + 'static,
    rot_lens: impl Lens<Target = f32> + Clone + 'static,
) {
    Collapsible::new(
        cx,
        move |cx| {
            Button::new(cx, |cx| {
                Label::new(cx, title)
                    .font_size(16.0)
                    .color(Color::rgb(230, 233, 240))
                    .width(Stretch(1.0))
            })
            .width(Stretch(1.0))
            .variant(ButtonVariant::Text)
            .on_press(move |cx| cx.emit(AppEvent::ToggleCollapse(node)));
        },
        move |cx| {
            VStack::new(cx, |cx| {
                pick_row(
                    cx,
                    "Overflow X",
                    AppData::overflow_options,
                    overflow_x_lens.clone(),
                    move |cx, index| {
                        cx.emit(AppEvent::SetOverflowX(node, index));
                    },
                );

                pick_row(
                    cx,
                    "Overflow Y",
                    AppData::overflow_options,
                    overflow_y_lens.clone(),
                    move |cx, index| {
                        cx.emit(AppEvent::SetOverflowY(node, index));
                    },
                );

                pick_row(
                    cx,
                    "Clip Shape",
                    AppData::clip_options,
                    clip_lens.clone(),
                    move |cx, index| {
                        cx.emit(AppEvent::SetClipShape(node, index));
                    },
                );

                slider_row(
                    cx,
                    "Inset",
                    inset_lens.clone(),
                    inset_lens,
                    0.0,
                    64.0,
                    "px",
                    move |cx, value| cx.emit(AppEvent::SetInset(node, value)),
                );

                slider_row(
                    cx,
                    "Border Width",
                    border_lens.clone(),
                    border_lens,
                    0.0,
                    48.0,
                    "px",
                    move |cx, value| cx.emit(AppEvent::SetBorderWidth(node, value)),
                );

                slider_row(
                    cx,
                    "Translate X",
                    tx_lens.clone(),
                    tx_lens,
                    -120.0,
                    120.0,
                    "px",
                    move |cx, value| cx.emit(AppEvent::SetTranslateX(node, value)),
                );

                slider_row(
                    cx,
                    "Translate Y",
                    ty_lens.clone(),
                    ty_lens,
                    -120.0,
                    120.0,
                    "px",
                    move |cx, value| cx.emit(AppEvent::SetTranslateY(node, value)),
                );

                slider_row(
                    cx,
                    "Rotation",
                    rot_lens.clone(),
                    rot_lens,
                    -180.0,
                    180.0,
                    "deg",
                    move |cx, value| cx.emit(AppEvent::SetRotation(node, value)),
                );
            })
            .width(Stretch(1.0))
            .height(Auto)
            .gap(Pixels(10.0))
            .padding_top(Pixels(8.0));
        },
    )
    .open(open_lens)
    .class("control-block")
    .width(Stretch(1.0))
    .height(Auto)
    .gap(Pixels(10.0))
    .padding(Pixels(14.0))
    .background_color(Color::rgb(29, 32, 43))
    .border_width(Pixels(1.0))
    .border_color(Color::rgb(49, 54, 72));
}

fn demo_panel(cx: &mut Context) {
    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Element::new(cx)
                    .size(Pixels(110.0))
                    .background_color(Color::rgb(54, 158, 88))
                    .border_width(AppData::grandchild.map(|state| Pixels(state.border_width)))
                    .border_color(Color::rgb(116, 230, 149))
                    .corner_radius(Pixels(20.0))
                    .overflowx(
                        AppData::grandchild
                            .map(|state| overflow_mode_from_index(state.overflow_x_index)),
                    )
                    .overflowy(
                        AppData::grandchild
                            .map(|state| overflow_mode_from_index(state.overflow_y_index)),
                    )
                    .clip_path(AppData::grandchild.map(clip_inset_from_state))
                    .translate(AppData::grandchild.map(|state| state.pos.translate()))
                    .rotate(AppData::grandchild.map(|state| Angle::Deg(state.rotate_deg)));
            })
            .size(Pixels(180.0))
            .background_color(Color::rgb(196, 96, 58))
            .border_width(AppData::child.map(|state| Pixels(state.border_width)))
            .border_color(Color::rgb(246, 170, 128))
            .corner_radius(Pixels(20.0))
            .alignment(Alignment::Center)
            .overflowx(AppData::child.map(|state| overflow_mode_from_index(state.overflow_x_index)))
            .overflowy(AppData::child.map(|state| overflow_mode_from_index(state.overflow_y_index)))
            .clip_path(AppData::child.map(clip_inset_from_state))
            .translate(AppData::child.map(|state| state.pos.translate()))
            .rotate(AppData::child.map(|state| Angle::Deg(state.rotate_deg)));
        })
        .size(Pixels(280.0))
        .background_color(Color::rgb(30, 78, 144))
        .border_width(AppData::parent.map(|state| Pixels(state.border_width)))
        .border_color(Color::rgb(121, 179, 255))
        .corner_radius(Pixels(20.0))
        .alignment(Alignment::Center)
        .overflowx(AppData::parent.map(|state| overflow_mode_from_index(state.overflow_x_index)))
        .overflowy(AppData::parent.map(|state| overflow_mode_from_index(state.overflow_y_index)))
        .clip_path(AppData::parent.map(clip_inset_from_state))
        .translate(AppData::parent.map(|state| state.pos.translate()))
        .rotate(AppData::parent.map(|state| Angle::Deg(state.rotate_deg)));
    })
    .size(Stretch(1.0))
    .alignment(Alignment::Center)
    .border_width(Pixels(1.0))
    .border_color(Color::rgb(46, 50, 67))
    .background_color(Color::rgb(11, 13, 20));
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData {
            overflow_options: OVERFLOW_OPTIONS.to_vec(),
            clip_options: CLIP_OPTIONS.to_vec(),
            parent_open: true,
            child_open: true,
            grandchild_open: true,
            parent: NodeState {
                overflow_x_index: 1,
                overflow_y_index: 1,
                clip_shape_index: 0,
                inset: 18.0,
                border_width: 10.0,
                pos: Pos::new(0.0, 0.0),
                rotate_deg: 0.0,
            },
            child: NodeState {
                overflow_x_index: 1,
                overflow_y_index: 1,
                clip_shape_index: 0,
                inset: 18.0,
                border_width: 10.0,
                pos: Pos::new(0.0, 0.0),
                rotate_deg: 0.0,
            },
            grandchild: NodeState {
                overflow_x_index: 1,
                overflow_y_index: 1,
                clip_shape_index: 0,
                inset: 18.0,
                border_width: 10.0,
                pos: Pos::new(0.0, 0.0),
                rotate_deg: 0.0,
            },
        }
        .build(cx);

        HStack::new(cx, |cx| {
            ScrollView::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    Label::new(cx, "Nested Clipping Controls")
                        .font_size(24.0)
                        .color(Color::white())
                        .padding_bottom(Pixels(4.0));

                    Label::new(
                        cx,
                        "Each node has independent overflow, clip shape, inset, border width, translation, and rotation controls.",
                    )
                    .font_size(12.0)
                    .color(Color::rgb(158, 164, 178))
                    .padding_bottom(Pixels(16.0));

                    controls_block(
                        cx,
                        "Parent",
                        NodeKind::Parent,
                        AppData::parent_open,
                        AppData::parent.map(|state| state.overflow_x_index),
                        AppData::parent.map(|state| state.overflow_y_index),
                        AppData::parent.map(|state| state.clip_shape_index),
                        AppData::parent.map(|state| state.inset),
                        AppData::parent.map(|state| state.border_width),
                        AppData::parent.map(|state| state.pos.x),
                        AppData::parent.map(|state| state.pos.y),
                        AppData::parent.map(|state| state.rotate_deg),
                    );

                    controls_block(
                        cx,
                        "Child",
                        NodeKind::Child,
                        AppData::child_open,
                        AppData::child.map(|state| state.overflow_x_index),
                        AppData::child.map(|state| state.overflow_y_index),
                        AppData::child.map(|state| state.clip_shape_index),
                        AppData::child.map(|state| state.inset),
                        AppData::child.map(|state| state.border_width),
                        AppData::child.map(|state| state.pos.x),
                        AppData::child.map(|state| state.pos.y),
                        AppData::child.map(|state| state.rotate_deg),
                    );

                    controls_block(
                        cx,
                        "Grandchild",
                        NodeKind::Grandchild,
                        AppData::grandchild_open,
                        AppData::grandchild.map(|state| state.overflow_x_index),
                        AppData::grandchild.map(|state| state.overflow_y_index),
                        AppData::grandchild.map(|state| state.clip_shape_index),
                        AppData::grandchild.map(|state| state.inset),
                        AppData::grandchild.map(|state| state.border_width),
                        AppData::grandchild.map(|state| state.pos.x),
                        AppData::grandchild.map(|state| state.pos.y),
                        AppData::grandchild.map(|state| state.rotate_deg),
                    );
                })
                .vertical_gap(Pixels(14.0))
                .padding(Pixels(18.0))
                .width(Stretch(1.0));
            })
            .width(Pixels(420.0));

            demo_panel(cx);
        })
        .size(Stretch(1.0))
        .background_color(Color::rgb(16, 18, 26));
    })
    .title("Nested Clipping Controls")
    .inner_size((1180, 760))
    .run()
}
