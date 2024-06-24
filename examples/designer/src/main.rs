use strum::VariantNames;
use vizia::{
    icons::{ICON_ALIGN_CENTER, ICON_ALIGN_LEFT, ICON_ALIGN_RIGHT, ICON_CHEVRON_DOWN},
    prelude::*,
};

use log::LevelFilter;

mod app_data;
use app_data::*;

pub fn setup_logging() -> Result<(), ApplicationError> {
    #[cfg(debug_assertions)]
    const MAIN_LOG_LEVEL: LevelFilter = LevelFilter::Debug;
    #[cfg(not(debug_assertions))]
    const MAIN_LOG_LEVEL: LevelFilter = LevelFilter::Info;

    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(move |out, message, record| {
            out.finish(format_args!("[{}][{}] {}", record.target(), record.level(), message))
        })
        // Add blanket level filter
        .level(MAIN_LOG_LEVEL)
        .level_for("cosmic_text::buffer", LevelFilter::Warn)
        .level_for("selectors::matching", LevelFilter::Warn)
        .level_for("cosmic_text::font::system::std", LevelFilter::Warn)
        // Output to stdout
        .chain(std::io::stdout())
        // Apply globally
        .apply()
        .map_err(|_| ApplicationError::LogError)?;

    Ok(())
}

// fn theme_selection_dropdown(cx: &mut Context) {
//     PickList::new(cx, AppData::theme_options, AppData::selected_theme, true)
//         .on_select(|cx, index| cx.emit(AppEvent::SetThemeMode(index)))
//         .width(Pixels(85.0))
//         .tooltip(|cx| {
//             Tooltip::new(cx, |cx| {
//                 Label::new(cx, "Select Theme Mode");
//             })
//         });
// }

fn labelled_control<T: ToStringLocalized>(
    cx: &mut Context,
    label: impl Res<T> + Clone,
    content: impl Fn(&mut Context),
) {
    VStack::new(cx, |cx| {
        content(cx);
        Label::new(cx, label).font_size(12.0);
    })
    .row_between(Pixels(4.0))
    .child_space(Stretch(1.0))
    .width(Auto)
    .height(Auto);
}

fn main() -> Result<(), ApplicationError> {
    setup_logging()?;

    Application::new(|cx: &mut Context| {
        cx.add_stylesheet(include_style!("src/style.css")).expect("Failed to add stylesheet");

        AppData {
            corner_top_right_radius: LengthOrPercentage::default(),
            corner_bottom_right_radius: LengthOrPercentage::default(),
            corner_bottom_left_radius: LengthOrPercentage::default(),
            corner_top_left_radius: LengthOrPercentage::default(),

            corner_top_right_shape: CornerShape::default(),
            corner_bottom_right_shape: CornerShape::default(),
            corner_bottom_left_shape: CornerShape::default(),
            corner_top_left_shape: CornerShape::default(),

            borer_corner_shapes: vec!["Round", "Bevel"],
            shadow_types: vec!["Outset", "Inset"],

            fonts: cx.text_context.default_font_manager.family_names().collect(),
            selected_font: 0,
            font: String::new(),

            selected_border_position: 0,

            border_width: LengthOrPercentage::default(),

            shadows: vec![Shadow {
                x_offset: Pixels(5.0).into(),
                y_offset: Pixels(5.0).into(),
                blur_radius: Some(Pixels(5.0).into()),
                spread_radius: Some(Pixels(5.0).into()),
                color: Some(Color::black()),
                inset: false,
            }],
            corner_top_right_smoothing: 0.0,
            corner_bottom_right_smoothing: 0.0,
            corner_bottom_left_smoothing: 0.0,
            corner_top_left_smoothing: 0.0,

            text_align: TextAlign::Left,

            font_size: 16.0,
            font_sizes: vec!["12"],
            selected_font_size: 0,
        }
        .build(cx);

        cx.emit(AppEvent::SetFont(0));

        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Element::new(cx)
                    .size(Pixels(200.0))
                    .background_color(Color::gray())
                    .corner_top_right_radius(AppData::corner_top_right_radius)
                    .corner_bottom_right_radius(AppData::corner_bottom_right_radius)
                    .corner_bottom_left_radius(AppData::corner_bottom_left_radius)
                    .corner_top_left_radius(AppData::corner_top_left_radius)
                    .corner_top_right_shape(AppData::corner_top_right_shape)
                    .corner_bottom_right_shape(AppData::corner_bottom_right_shape)
                    .corner_bottom_left_shape(AppData::corner_bottom_left_shape)
                    .corner_top_left_shape(AppData::corner_top_left_shape)
                    .corner_top_right_smoothing(AppData::corner_top_right_smoothing)
                    .corner_bottom_right_smoothing(AppData::corner_bottom_right_smoothing)
                    .corner_bottom_left_smoothing(AppData::corner_bottom_left_smoothing)
                    .corner_top_left_smoothing(AppData::corner_top_left_smoothing)
                    .border_width(AppData::border_width)
                    .border_color(Color::black())
                    // .shadows(AppData::shadows)
                    .text("value")
                    .font_family(AppData::font.map(|fam| vec![FamilyOwned::Named(fam.clone())]))
                    .font_size(AppData::font_size)
                    .text_align(AppData::text_align);
            })
            .child_space(Stretch(1.0));
            Divider::new(cx);
            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    labelled_control(cx, "Color", |cx| {
                        Button::new(cx, |cx| Label::new(cx, "col"));
                    });

                    labelled_control(cx, "Position", |cx| {
                        PickList::new(
                            cx,
                            &BorderPosition::VARIANTS,
                            AppData::selected_border_position,
                            true,
                        )
                        .width(Pixels(100.0));
                    });

                    labelled_control(cx, "Width", |cx| {
                        Textbox::new(
                            cx,
                            AppData::border_width.map(|l| match l {
                                LengthOrPercentage::Length(length) => length.to_px().unwrap(),
                                LengthOrPercentage::Percentage(percent) => *percent * 100.0,
                            }),
                        )
                        .on_submit(|cx, val, _| cx.emit(AppEvent::SetBorderWidth(val)))
                        .width(Pixels(70.0));
                    });
                })
                .height(Auto)
                .col_between(Pixels(8.0));

                ComboBox::new(cx, AppData::fonts, AppData::selected_font)
                    .width(Stretch(1.0))
                    .on_select(|cx, val| cx.emit(AppEvent::SetFont(val)));

                ButtonGroup::new(cx, |cx| {
                    ToggleButton::new(
                        cx,
                        AppData::text_align.map(|ta| *ta == TextAlign::Left),
                        |cx| Icon::new(cx, ICON_ALIGN_LEFT),
                    )
                    .on_toggle(|cx| cx.emit(AppEvent::SetTextAlign(TextAlign::Left)));

                    ToggleButton::new(
                        cx,
                        AppData::text_align.map(|ta| *ta == TextAlign::Center),
                        |cx| Icon::new(cx, ICON_ALIGN_CENTER),
                    )
                    .on_toggle(|cx| cx.emit(AppEvent::SetTextAlign(TextAlign::Center)));

                    ToggleButton::new(
                        cx,
                        AppData::text_align.map(|ta| *ta == TextAlign::Right),
                        |cx| Icon::new(cx, ICON_ALIGN_RIGHT),
                    )
                    .on_toggle(|cx| cx.emit(AppEvent::SetTextAlign(TextAlign::Right)));
                });

                Dropdown::new(
                    cx,
                    |cx| {
                        ButtonGroup::new(cx, |cx| {
                            Textbox::new(cx, AppData::font_size)
                                .width(Pixels(100.0))
                                .on_submit(|cx, val, _| cx.emit(AppEvent::SetFontSize(val)));
                            IconButton::new(cx, ICON_CHEVRON_DOWN)
                                .on_press(|cx| cx.emit(PopupEvent::Switch));
                        });
                    },
                    |cx| {
                        ScrollList::new(
                            cx,
                            StaticLens::new(&FONT_SIZES),
                            AppData::selected_font_size,
                        )
                        .on_select(|cx, idx| {
                            cx.emit(AppEvent::SetSelectedFontSize(idx));
                            cx.emit(PopupEvent::Close);
                        })
                        .width(Stretch(1.0));
                    },
                );

                // Label::new(cx, "Corners").font_variation_settings(vec!["\"wght\" 800.0".into()]);
                // VStack::new(cx, |cx| {
                //     HStack::new(cx, |cx| {
                //         Slider::new(cx, AppData::corner_top_right_smoothing).on_changing(
                //             |cx, val| cx.emit(AppEvent::SetCornerTopRightSmoothing(val)),
                //         );

                //         Textbox::new(
                //             cx,
                //             AppData::corner_top_right_radius.map(|l| match l {
                //                 LengthOrPercentage::Length(length) => length.to_px().unwrap(),
                //                 LengthOrPercentage::Percentage(percent) => *percent * 100.0,
                //             }),
                //         )
                //         .width(Pixels(70.0))
                //         .on_submit(|cx, val, _| cx.emit(AppEvent::SetCornerTopRightRadius(val)));

                //         PickList::new(
                //             cx,
                //             AppData::borer_corner_shapes,
                //             AppData::corner_top_right_shape.map(|s| *s as usize),
                //             true,
                //         )
                //         .width(Pixels(75.0))
                //         .top(Stretch(1.0))
                //         .on_select(|cx, val| {
                //             cx.emit(AppEvent::SetCornerTopRightShape(if val == 0 {
                //                 CornerShape::Round
                //             } else {
                //                 CornerShape::Bevel
                //             }))
                //         });
                //     })
                //     .col_between(Pixels(8.0))
                //     .height(Auto);
                //     HStack::new(cx, |cx| {
                //         UnitEditor::new(
                //             cx,
                //             "Corner Bottom Right Radius",
                //             AppData::corner_bottom_right_radius,
                //         )
                //         .on_change(|cx, val| cx.emit(AppEvent::SetCornerBottomRightRadius(val)));

                //         PickList::new(
                //             cx,
                //             AppData::borer_corner_shapes,
                //             AppData::corner_bottom_right_shape.map(|s| *s as usize),
                //             true,
                //         )
                //         .width(Pixels(75.0))
                //         .top(Stretch(1.0))
                //         .on_select(|cx, val| {
                //             cx.emit(AppEvent::SetCornerBottomRightShape(if val == 0 {
                //                 CornerShape::Round
                //             } else {
                //                 CornerShape::Bevel
                //             }))
                //         });
                //     })
                //     .col_between(Pixels(8.0))
                //     .height(Auto);
                //     HStack::new(cx, |cx| {
                //         UnitEditor::new(
                //             cx,
                //             "Corner Bottom Left Radius",
                //             AppData::corner_bottom_left_radius,
                //         )
                //         .on_change(|cx, val| cx.emit(AppEvent::SetCornerBottomLeftRadius(val)));

                //         PickList::new(
                //             cx,
                //             AppData::borer_corner_shapes,
                //             AppData::corner_bottom_left_shape.map(|s| *s as usize),
                //             true,
                //         )
                //         .width(Pixels(75.0))
                //         .top(Stretch(1.0))
                //         .on_select(|cx, val| {
                //             cx.emit(AppEvent::SetCornerBottomLeftShape(if val == 0 {
                //                 CornerShape::Round
                //             } else {
                //                 CornerShape::Bevel
                //             }))
                //         });
                //     })
                //     .col_between(Pixels(8.0))
                //     .height(Auto);
                //     HStack::new(cx, |cx| {
                //         UnitEditor::new(
                //             cx,
                //             "Corner Top Left Radius",
                //             AppData::corner_top_left_radius,
                //         )
                //         .on_change(|cx, val| cx.emit(AppEvent::SetCornerTopLeftRadius(val)));

                //         PickList::new(
                //             cx,
                //             AppData::borer_corner_shapes,
                //             AppData::corner_top_left_shape.map(|s| *s as usize),
                //             true,
                //         )
                //         .width(Pixels(75.0))
                //         .top(Stretch(1.0))
                //         .on_select(|cx, val| {
                //             cx.emit(AppEvent::SetCornerTopLeftShape(if val == 0 {
                //                 CornerShape::Round
                //             } else {
                //                 CornerShape::Bevel
                //             }))
                //         });
                //     })
                //     .col_between(Pixels(8.0))
                //     .height(Auto);
                // })
                // .height(Auto);
                // Divider::new(cx);
                // Label::new(cx, "Border").font_variation_settings(vec!["\"wght\" 800.0".into()]);
                // VStack::new(cx, |cx| {
                //     UnitEditor::new(cx, "Border Width", AppData::border_width)
                //         .on_change(|cx, val| cx.emit(AppEvent::SetBorderWidth(val)));
                // })
                // .height(Auto);
                // Divider::new(cx);
                // Label::new(cx, "Shadows").font_variation_settings(vec!["\"wght\" 800.0".into()]);
                // VStack::new(cx, |cx| {
                //     List::new(cx, AppData::shadows, |cx, idx, item| {
                //         HStack::new(cx, |cx| {
                //             Element::new(cx)
                //                 .height(Stretch(1.0))
                //                 .width(Pixels(50.0))
                //                 .background_color(
                //                     item.map(|shadow| shadow.color.unwrap_or_default()),
                //                 );

                //             Textbox::new(
                //                 cx,
                //                 item.map(|shadow| shadow.x_offset.to_px().unwrap_or_default()),
                //             )
                //             .width(Pixels(60.0))
                //             .on_submit(move |cx, val, _| cx.emit(AppEvent::SetShadowX(idx, val)));

                //             Textbox::new(
                //                 cx,
                //                 item.map(|shadow| shadow.y_offset.to_px().unwrap_or_default()),
                //             )
                //             .width(Pixels(60.0))
                //             .on_submit(move |cx, val, _| cx.emit(AppEvent::SetShadowY(idx, val)));

                //             Textbox::new(
                //                 cx,
                //                 item.map(|shadow| {
                //                     shadow
                //                         .blur_radius
                //                         .clone()
                //                         .unwrap_or_default()
                //                         .to_px()
                //                         .unwrap_or_default()
                //                 }),
                //             )
                //             .width(Pixels(60.0))
                //             .on_submit(move |cx, val, _| {
                //                 cx.emit(AppEvent::SetShadowBlur(idx, val))
                //             });

                //             Textbox::new(
                //                 cx,
                //                 item.map(|shadow| {
                //                     shadow
                //                         .spread_radius
                //                         .clone()
                //                         .unwrap_or_default()
                //                         .to_px()
                //                         .unwrap_or_default()
                //                 }),
                //             )
                //             .width(Pixels(60.0))
                //             .on_submit(move |cx, val, _| {
                //                 cx.emit(AppEvent::SetShadowSpread(idx, val))
                //             });

                //             PickList::new(
                //                 cx,
                //                 AppData::shadow_types,
                //                 item.map(|s| if s.inset { 1 } else { 0 }),
                //                 true,
                //             )
                //             .width(Pixels(75.0))
                //             .top(Stretch(1.0))
                //             .on_select(move |cx, val| {
                //                 cx.emit(AppEvent::SetShadowType(idx, val == 1))
                //             });
                //         })
                //         .col_between(Pixels(8.0))
                //         .width(Auto)
                //         .height(Auto);
                //     })
                //     .width(Auto);
                // })
                // .width(Auto)
                // .height(Auto);
            })
            .row_between(Pixels(8.0))
            .child_space(Pixels(20.0))
            .width(Auto)
            .min_width(Pixels(400.0));
        });
    })
    .title("Widget Gallery")
    .inner_size((1400, 600))
    .min_inner_size(Some((900, 300)))
    .run()
}
