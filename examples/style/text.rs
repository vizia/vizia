use vizia::prelude::*;

const STYLE: &str = r#"
    .font_size {
        font-size: xx-large;
    }

    .font_color {
        color: red;
    }

    .font_weight {
        font-weight: bold;
    }

    .font_slant {
        font-slant: italic;
    }

    .font_width {
        font-width: condensed;
    }

    .text_overflow {
        text-overflow: ellipsis;
        line-clamp: 1;
        width: 1s;
        padding: 1s;
    }

    .caret_color:checked .textbox_content {
        caret-color: #00FF00;
        selection-color: #c8646488;
    }

    .text-decoration {
        text-decoration-line: underline;
    }

"#;

#[derive(Lens)]
pub struct AppData {
    text: String,
    fonts: Vec<String>,
    weights: Vec<&'static str>,
    sizes: Vec<u32>,
    selected_font: String,
    selected_weight: FontWeight,
    selected_size: u32,
    selected_align: TextAlign,
}

pub enum AppEvent {
    SetSelectedFont(String),
    SetSelectedWeight(String),
    SetSelectedSize(u32),
    SetSelectedAlign(TextAlign),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetSelectedFont(font) => {
                self.selected_font = font.clone();
            }

            AppEvent::SetSelectedWeight(weight_keyword) => {
                let weight: FontWeight = match weight_keyword.as_str() {
                    "Thin" => FontWeightKeyword::Thin,
                    "Extra Light" => FontWeightKeyword::ExtraLight,
                    "Light" => FontWeightKeyword::Light,
                    "Normal" => FontWeightKeyword::Normal,
                    "Medium" => FontWeightKeyword::Medium,
                    "Bold" => FontWeightKeyword::Bold,
                    "Semi Bold" => FontWeightKeyword::SemiBold,
                    "Extra Bold" => FontWeightKeyword::ExtraBold,
                    "Black" => FontWeightKeyword::Black,
                    _ => unreachable!(),
                }
                .into();

                self.selected_weight = weight;
            }

            AppEvent::SetSelectedSize(size) => {
                self.selected_size = *size;
            }

            AppEvent::SetSelectedAlign(align) => {
                self.selected_align = *align;
            }
        })
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let fonts: Vec<String> = cx.text_context.default_font_manager.family_names().collect();
        AppData {
            text: "This text is editable!".to_string(),
            fonts,
            selected_font: String::from("Arial"),
            selected_weight: FontWeight::from(FontWeightKeyword::Normal),
            selected_size: 10,
            selected_align: TextAlign::Left,
            weights: vec![
                "Thin",
                "Extra Light",
                "Light",
                "Normal",
                "Medium",
                "Bold",
                "Semi Bold",
                "Extra Bold",
                "Black",
            ],
            sizes: vec![10, 11, 12, 13, 14, 15, 16, 20, 24, 32, 36, 40, 48, 64, 96, 128],
        }
        .build(cx);

        // HStack::new(cx, |cx|{
        //     VStack::new(cx, |cx|{
        //         Label::new(cx, "This is some tester text which is sufficiently long as to wrap multiple lines if wrapping is enabled.")
        //         .width(Pixels(200.0))
        //         .height(Auto)
        //         .border_color(Color::gray())
        //         .border_width(Pixels(1.0))
        //         .font_family(AppData::selected_font.map(|font| vec![FamilyOwned::Named(font.clone())]))
        //         .font_weight(AppData::selected_weight)
        //         .font_size(AppData::selected_size)
        //         .text_align(AppData::selected_align);
        //     }).alignment(Alignment::Center);

        //     VStack::new(cx, |cx|{
        //         PickList::new(cx, AppData::fonts, true, move |index, label| {
        //             label.font_family(
        //                 AppData::fonts.map(move |font| vec![FamilyOwned::Named(font[index].clone())]),
        //             )
        //         })
        //         .width(Stretch(1.0))
        //         .on_item_select(|cx, item| cx.emit(AppEvent::SetSelectedFont(item.clone())));
        //         HStack::new(cx, |cx|{
        //             PickList::new(cx, AppData::weights, true, |i, l| l)
        //                 .width(Stretch(1.0))
        //                 .on_item_select(|cx, item| cx.emit(AppEvent::SetSelectedWeight(item.to_owned())));

        //             PickList::new(cx, AppData::sizes, true, |i, l| l)
        //                 .width(Stretch(1.0))
        //                 .on_item_select(|cx, item| cx.emit(AppEvent::SetSelectedSize(item)));

        //         }).horizontal_gap(Pixels(4.0)).height(Auto);

        //         ButtonGroup::new(cx, |cx|{
        //             ToggleButton::new(cx, AppData::selected_align.map(|align| *align == TextAlign::Left), |cx|{
        //                 Svg::new(cx, ICON_ALIGN_LEFT)
        //             })
        //             .on_toggle(|cx| cx.emit(AppEvent::SetSelectedAlign(TextAlign::Left)));
        //             ToggleButton::new(cx, AppData::selected_align.map(|align| *align == TextAlign::Center), |cx|{
        //                 Svg::new(cx, ICON_ALIGN_CENTER)
        //             })
        //             .on_toggle(|cx| cx.emit(AppEvent::SetSelectedAlign(TextAlign::Center)));
        //             ToggleButton::new(cx, AppData::selected_align.map(|align| *align == TextAlign::Right), |cx|{
        //                 Svg::new(cx, ICON_ALIGN_RIGHT)
        //             })
        //             .on_toggle(|cx| cx.emit(AppEvent::SetSelectedAlign(TextAlign::Right)));
        //             ToggleButton::new(cx, AppData::selected_align.map(|align| *align == TextAlign::Justify), |cx|{
        //                 Svg::new(cx, ICON_ALIGN_JUSTIFIED)
        //             })
        //             .on_toggle(|cx| cx.emit(AppEvent::SetSelectedAlign(TextAlign::Justify)));

        //         });
        //     }).width(Pixels(200.0)).shadow("-2px 0px 10px #22222255").padding(Pixels(10.0)).vertical_gap(Pixels(10.0));
        // });

        VStack::new(cx, |cx| {
            Label::new(cx, "Font Size").class("font_size");
            Label::new(cx, "Font Color").class("font_color");
            Label::new(cx, "Font Weight").class("font_weight");
            Label::new(cx, "Font Slant").class("font_slant");
            Label::new(cx, "Font Width").class("font_width");
            Label::new(cx, "Text Decoration").class("text-decoration");
            Label::new(cx, "Text Overflow")
                .width(Stretch(1.0))
                .text_overflow(TextOverflow::Ellipsis)
                .line_clamp(1)
                .text_align(TextAlign::Center);
        })
        .vertical_gap(Pixels(10.0))
        .padding(Pixels(10.0));

        // Textbox::new(cx, AppData::text)
        //     .on_edit(|cx, text| cx.emit(AppDataSetter::Text(text)))
        //     .width(Pixels(200.0))
        //     .class("caret_color");
    })
    .run()
}
