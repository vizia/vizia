use vizia::prelude::*;

const STYLE: &str = r#"

    .row {
        child-space: 1s;
        col-between: 20px;
    }

    element {
        size: 100px;
        border-radius: 20px;
        background-color: rgb(200, 200, 200);
    }

    .border {
        border: black 5px;
    }

    .border:hover {
        border: 10px blue;
        transition: border 0.1s;
    }

    .border_radius {
        border-radius: 5px 10px 15px 20px;
    }

    .border_radius:hover {
        border-radius: 10px 20px 30px 40px;
        transition: border-radius 0.1s;
    }

    .border_shape {
        border-radius: 30px;
        border-corner-shape: round bevel round bevel;
    }

    .border_shape:hover {
        border-radius: 30px;
        border-corner-shape: bevel round bevel round;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    border_top_right_radius: LengthOrPercentage,
    border_bottom_right_radius: LengthOrPercentage,
    border_bottom_left_radius: LengthOrPercentage,
    border_top_left_radius: LengthOrPercentage,

    border_top_right_shape: BorderCornerShape,
    border_bottom_right_shape: BorderCornerShape,
    border_bottom_left_shape: BorderCornerShape,
    border_top_left_shape: BorderCornerShape,

    borer_corner_shapes: Vec<&'static str>,

    border_width: LengthOrPercentage,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetBorderTopRightRadius(val) => {
                self.border_top_right_radius = match self.border_top_left_radius {
                    LengthOrPercentage::Length(_) => LengthOrPercentage::Length(Length::px(*val)),
                    LengthOrPercentage::Percentage(_) => {
                        LengthOrPercentage::Percentage(*val / 100.0)
                    }
                }
            }

            AppEvent::SetBorderBottomRightRadius(val) => {
                self.border_bottom_right_radius = match self.border_top_left_radius {
                    LengthOrPercentage::Length(_) => LengthOrPercentage::Length(Length::px(*val)),
                    LengthOrPercentage::Percentage(_) => {
                        LengthOrPercentage::Percentage(*val / 100.0)
                    }
                }
            }

            AppEvent::SetBorderBottomLeftRadius(val) => {
                self.border_bottom_left_radius = match self.border_top_left_radius {
                    LengthOrPercentage::Length(_) => LengthOrPercentage::Length(Length::px(*val)),
                    LengthOrPercentage::Percentage(_) => {
                        LengthOrPercentage::Percentage(*val / 100.0)
                    }
                }
            }

            AppEvent::SetBorderTopLeftRadius(val) => {
                self.border_top_left_radius = match self.border_top_left_radius {
                    LengthOrPercentage::Length(_) => LengthOrPercentage::Length(Length::px(*val)),
                    LengthOrPercentage::Percentage(_) => {
                        LengthOrPercentage::Percentage(*val / 100.0)
                    }
                }
            }
            AppEvent::SetBorderTopRightShape(shape) => self.border_top_right_shape = *shape,
            AppEvent::SetBorderBottomRightShape(shape) => self.border_bottom_right_shape = *shape,
            AppEvent::SetBorderBottomLeftShape(shape) => self.border_bottom_left_shape = *shape,
            AppEvent::SetBorderTopLeftShape(shape) => self.border_top_left_shape = *shape,
            AppEvent::SetBorderWidth(val) => {
                self.border_width = match self.border_width {
                    LengthOrPercentage::Length(_) => LengthOrPercentage::Length(Length::px(*val)),
                    LengthOrPercentage::Percentage(_) => {
                        LengthOrPercentage::Percentage(*val / 100.0)
                    }
                }
            }
        })
    }
}

pub enum AppEvent {
    SetBorderTopRightRadius(f32),
    SetBorderBottomRightRadius(f32),
    SetBorderBottomLeftRadius(f32),
    SetBorderTopLeftRadius(f32),

    SetBorderTopRightShape(BorderCornerShape),
    SetBorderBottomRightShape(BorderCornerShape),
    SetBorderBottomLeftShape(BorderCornerShape),
    SetBorderTopLeftShape(BorderCornerShape),

    SetBorderWidth(f32),
}

pub struct UnitEditor {
    on_change: Option<Box<dyn Fn(&mut EventContext, f32)>>,
}

impl UnitEditor {
    pub fn new<T: ToStringLocalized>(
        cx: &mut Context,
        label: impl Res<T> + Clone,
        lens: impl Lens<Target = LengthOrPercentage>,
    ) -> Handle<Self> {
        Self { on_change: None }
            .build(cx, |cx| {
                Label::new(cx, label);
                HStack::new(cx, |cx| {
                    Slider::new(
                        cx,
                        lens.map(|l| match l {
                            LengthOrPercentage::Length(length) => length.to_px().unwrap(),
                            LengthOrPercentage::Percentage(percent) => *percent * 100.0,
                        }),
                    )
                    .step(1.0)
                    .range(0.0..100.0)
                    .on_changing(|cx, val| cx.emit(UnitEditorEvent::SetValue(val)));
                    // .range(lens.map(|l| match l {
                    //     LengthOrPercentage::Length(_) => 0.0f32..100.0f32,
                    //     LengthOrPercentage::Percentage(_) => 0.0f32..100.0f32,
                    // }));
                    Textbox::new(
                        cx,
                        lens.map(|l| match l {
                            LengthOrPercentage::Length(length) => length.to_px().unwrap(),
                            LengthOrPercentage::Percentage(percent) => *percent * 100.0,
                        }),
                    )
                    .width(Pixels(70.0));
                })
                .col_between(Pixels(8.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .height(Auto);
            })
            .height(Auto)
    }
}

enum UnitEditorEvent {
    SetValue(f32),
}

impl View for UnitEditor {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|unit_editor_event, _| match unit_editor_event {
            UnitEditorEvent::SetValue(val) => {
                if let Some(callback) = &self.on_change {
                    (callback)(cx, *val);
                }
            }
        })
    }
}

pub trait UnitEditorModifiers {
    fn on_change(self, callback: impl Fn(&mut EventContext, f32) + 'static) -> Self;
}

impl<'a> UnitEditorModifiers for Handle<'a, UnitEditor> {
    fn on_change(self, callback: impl Fn(&mut EventContext, f32) + 'static) -> Self {
        self.modify(|unit_editor| unit_editor.on_change = Some(Box::new(callback)))
    }
}
fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData {
            border_top_right_radius: LengthOrPercentage::default(),
            border_bottom_right_radius: LengthOrPercentage::default(),
            border_bottom_left_radius: LengthOrPercentage::default(),
            border_top_left_radius: LengthOrPercentage::default(),

            border_top_right_shape: BorderCornerShape::default(),
            border_bottom_right_shape: BorderCornerShape::default(),
            border_bottom_left_shape: BorderCornerShape::default(),
            border_top_left_shape: BorderCornerShape::default(),

            borer_corner_shapes: vec!["Round", "Bevel"],

            border_width: LengthOrPercentage::default(),
        }
        .build(cx);

        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Element::new(cx)
                    .size(Pixels(200.0))
                    .background_color(Color::gray())
                    .border_top_right_radius(AppData::border_top_right_radius)
                    .border_bottom_right_radius(AppData::border_bottom_right_radius)
                    .border_bottom_left_radius(AppData::border_bottom_left_radius)
                    .border_top_left_radius(AppData::border_top_left_radius)
                    .border_top_right_shape(AppData::border_top_right_shape)
                    .border_bottom_right_shape(AppData::border_bottom_right_shape)
                    .border_bottom_left_shape(AppData::border_bottom_left_shape)
                    .border_top_left_shape(AppData::border_top_left_shape)
                    .border_width(AppData::border_width)
                    .border_color(Color::black());
            })
            .child_space(Stretch(1.0));
            VStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        UnitEditor::new(
                            cx,
                            "Border Top Right Radius",
                            AppData::border_top_right_radius,
                        )
                        .on_change(|cx, val| cx.emit(AppEvent::SetBorderTopRightRadius(val)));

                        PickList::new(
                            cx,
                            AppData::borer_corner_shapes,
                            AppData::border_top_right_shape.map(|s| *s as usize),
                            true,
                        )
                        .width(Pixels(75.0))
                        .top(Stretch(1.0))
                        .on_select(|cx, val| {
                            cx.emit(AppEvent::SetBorderTopRightShape(if val == 0 {
                                BorderCornerShape::Round
                            } else {
                                BorderCornerShape::Bevel
                            }))
                        });
                    })
                    .col_between(Pixels(8.0))
                    .height(Auto);
                    HStack::new(cx, |cx| {
                        UnitEditor::new(
                            cx,
                            "Border Bottom Right Radius",
                            AppData::border_bottom_right_radius,
                        )
                        .on_change(|cx, val| cx.emit(AppEvent::SetBorderBottomRightRadius(val)));

                        PickList::new(
                            cx,
                            AppData::borer_corner_shapes,
                            AppData::border_bottom_right_shape.map(|s| *s as usize),
                            true,
                        )
                        .width(Pixels(75.0))
                        .top(Stretch(1.0))
                        .on_select(|cx, val| {
                            cx.emit(AppEvent::SetBorderBottomRightShape(if val == 0 {
                                BorderCornerShape::Round
                            } else {
                                BorderCornerShape::Bevel
                            }))
                        });
                    })
                    .col_between(Pixels(8.0))
                    .height(Auto);
                    HStack::new(cx, |cx| {
                        UnitEditor::new(
                            cx,
                            "Border Bottom Left Radius",
                            AppData::border_bottom_left_radius,
                        )
                        .on_change(|cx, val| cx.emit(AppEvent::SetBorderBottomLeftRadius(val)));

                        PickList::new(
                            cx,
                            AppData::borer_corner_shapes,
                            AppData::border_bottom_left_shape.map(|s| *s as usize),
                            true,
                        )
                        .width(Pixels(75.0))
                        .top(Stretch(1.0))
                        .on_select(|cx, val| {
                            cx.emit(AppEvent::SetBorderBottomLeftShape(if val == 0 {
                                BorderCornerShape::Round
                            } else {
                                BorderCornerShape::Bevel
                            }))
                        });
                    })
                    .col_between(Pixels(8.0))
                    .height(Auto);
                    HStack::new(cx, |cx| {
                        UnitEditor::new(
                            cx,
                            "Border Top Left Radius",
                            AppData::border_top_left_radius,
                        )
                        .on_change(|cx, val| cx.emit(AppEvent::SetBorderTopLeftRadius(val)));

                        PickList::new(
                            cx,
                            AppData::borer_corner_shapes,
                            AppData::border_top_left_shape.map(|s| *s as usize),
                            true,
                        )
                        .width(Pixels(75.0))
                        .top(Stretch(1.0))
                        .on_select(|cx, val| {
                            cx.emit(AppEvent::SetBorderTopLeftShape(if val == 0 {
                                BorderCornerShape::Round
                            } else {
                                BorderCornerShape::Bevel
                            }))
                        });
                    })
                    .col_between(Pixels(8.0))
                    .height(Auto);
                })
                .height(Auto);
                VStack::new(cx, |cx| {
                    UnitEditor::new(cx, "Border Width", AppData::border_width)
                        .on_change(|cx, val| cx.emit(AppEvent::SetBorderWidth(val)));
                })
                .height(Auto);
            })
            .row_between(Pixels(8.0))
            .child_space(Pixels(20.0));
        });

        // cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        // HStack::new(cx, |cx| {
        //     Element::new(cx).class("border");
        //     Element::new(cx).class("border_radius");
        //     Element::new(cx).class("border_shape");
        // })
        // .class("row");

        // HStack::new(cx, |cx| {
        //     Element::new(cx).border_color(Color::black()).border_width(Pixels(10.0));

        //     Element::new(cx).border_radius((
        //         Length::Value(LengthValue::Px(5.0)),
        //         Pixels(20.0),
        //         "30px",
        //         LengthValue::Px(40.0),
        //     ));

        //     Element::new(cx).border_radius(Pixels(30.0)).border_corner_shape((
        //         BorderCornerShape::Bevel,
        //         BorderCornerShape::Round,
        //         BorderCornerShape::Bevel,
        //         BorderCornerShape::Round,
        //     ));
        // })
        // .class("row");
    })
    .title("Border")
    .run()
}
