// use strum::VariantNames;
// use strum_macros::VariantNames;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    pub corner_top_right_radius: LengthOrPercentage,
    pub corner_bottom_right_radius: LengthOrPercentage,
    pub corner_bottom_left_radius: LengthOrPercentage,
    pub corner_top_left_radius: LengthOrPercentage,

    pub corner_top_right_smoothing: f32,
    pub corner_bottom_right_smoothing: f32,
    pub corner_bottom_left_smoothing: f32,
    pub corner_top_left_smoothing: f32,

    pub corner_top_right_shape: CornerShape,
    pub corner_bottom_right_shape: CornerShape,
    pub corner_bottom_left_shape: CornerShape,
    pub corner_top_left_shape: CornerShape,

    pub borer_corner_shapes: Vec<&'static str>,
    pub shadow_types: Vec<&'static str>,
    pub font_sizes: Vec<&'static str>,

    pub selected_font_size: usize,

    pub fonts: Vec<String>,
    pub selected_font: usize,
    pub font: String,

    pub text_align: TextAlign,

    pub selected_border_position: usize,

    pub shadows: Vec<Shadow>,

    pub border_width: LengthOrPercentage,

    pub font_size: f32,

    pub text_decoration_line: TextDecorationLine,
}

// #[derive(Clone, Debug, VariantNames)]
// #[strum(serialize_all = "title_case")]
// pub enum BorderPosition {
//     Inside,
//     Center,
//     Outside,
// }

// impl BorderPosition {
//     pub fn variants() -> impl Lens<Target = &'static [&'static str]> {
//         StaticLens::new(&Self::VARIANTS)
//     }
// }

pub const FONT_SIZES: &'static [f32] = &[12.0, 14.0];

pub enum AppEvent {
    SetCornerTopRightRadius(f32),
    SetCornerBottomRightRadius(f32),
    SetCornerBottomLeftRadius(f32),
    SetCornerTopLeftRadius(f32),

    SetCornerTopRightShape(CornerShape),
    SetCornerBottomRightShape(CornerShape),
    SetCornerBottomLeftShape(CornerShape),
    SetCornerTopLeftShape(CornerShape),

    SetCornerTopRightSmoothing(f32),
    SetCornerBottomRightSmoothing(f32),
    SetCornerBottomLeftSmoothing(f32),
    SetCornerTopLeftSmoothing(f32),

    SetBorderWidth(f32),

    SetFont(usize),

    // SetShadowColor(usize, Color),
    SetShadowX(usize, f32),
    SetShadowY(usize, f32),
    SetShadowBlur(usize, f32),
    SetShadowSpread(usize, f32),
    SetShadowType(usize, bool),

    SetTextAlign(TextAlign),

    SetFontSize(f32),
    SetSelectedFontSize(usize),
    ToggleUnderline,
    ToggleOverline,
    ToggleStrikethrough,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetCornerTopRightRadius(val) => {
                self.corner_top_right_radius = match self.corner_top_left_radius {
                    LengthOrPercentage::Length(_) => LengthOrPercentage::Length(Length::px(*val)),
                    LengthOrPercentage::Percentage(_) => {
                        LengthOrPercentage::Percentage(*val / 100.0)
                    }
                }
            }

            AppEvent::SetCornerBottomRightRadius(val) => {
                self.corner_bottom_right_radius = match self.corner_top_left_radius {
                    LengthOrPercentage::Length(_) => LengthOrPercentage::Length(Length::px(*val)),
                    LengthOrPercentage::Percentage(_) => {
                        LengthOrPercentage::Percentage(*val / 100.0)
                    }
                }
            }

            AppEvent::SetCornerBottomLeftRadius(val) => {
                self.corner_bottom_left_radius = match self.corner_top_left_radius {
                    LengthOrPercentage::Length(_) => LengthOrPercentage::Length(Length::px(*val)),
                    LengthOrPercentage::Percentage(_) => {
                        LengthOrPercentage::Percentage(*val / 100.0)
                    }
                }
            }

            AppEvent::SetCornerTopLeftRadius(val) => {
                self.corner_top_left_radius = match self.corner_top_left_radius {
                    LengthOrPercentage::Length(_) => LengthOrPercentage::Length(Length::px(*val)),
                    LengthOrPercentage::Percentage(_) => {
                        LengthOrPercentage::Percentage(*val / 100.0)
                    }
                }
            }
            AppEvent::SetCornerTopRightShape(shape) => self.corner_top_right_shape = *shape,
            AppEvent::SetCornerBottomRightShape(shape) => self.corner_bottom_right_shape = *shape,
            AppEvent::SetCornerBottomLeftShape(shape) => self.corner_bottom_left_shape = *shape,
            AppEvent::SetCornerTopLeftShape(shape) => self.corner_top_left_shape = *shape,
            AppEvent::SetBorderWidth(val) => {
                self.border_width = match self.border_width {
                    LengthOrPercentage::Length(_) => LengthOrPercentage::Length(Length::px(*val)),
                    LengthOrPercentage::Percentage(_) => {
                        LengthOrPercentage::Percentage(*val / 100.0)
                    }
                }
            }
            // AppEvent::SetShadowColor(idx, col) => todo!(),
            AppEvent::SetShadowX(idx, val) => {
                self.shadows[*idx].x_offset = Length::px(*val).into();
            }
            AppEvent::SetShadowY(idx, val) => {
                self.shadows[*idx].y_offset = Length::px(*val).into();
            }
            AppEvent::SetShadowBlur(idx, val) => {
                self.shadows[*idx].blur_radius = Some(Length::px(*val).into());
            }
            AppEvent::SetShadowSpread(idx, val) => {
                self.shadows[*idx].spread_radius = Some(Length::px(*val).into());
            }
            AppEvent::SetShadowType(idx, val) => {
                self.shadows[*idx].inset = *val;
            }
            AppEvent::SetCornerTopRightSmoothing(val) => self.corner_top_right_smoothing = *val,
            AppEvent::SetCornerBottomRightSmoothing(val) => {
                self.corner_bottom_right_smoothing = *val
            }
            AppEvent::SetCornerBottomLeftSmoothing(val) => self.corner_bottom_left_smoothing = *val,
            AppEvent::SetCornerTopLeftSmoothing(val) => self.corner_top_left_smoothing = *val,
            AppEvent::SetFont(val) => {
                self.selected_font = *val;
                self.font = self.fonts[self.selected_font].clone();
            }
            AppEvent::SetTextAlign(val) => self.text_align = *val,

            AppEvent::SetFontSize(val) => {
                self.font_size = *val;
            }

            AppEvent::SetSelectedFontSize(val) => {
                self.font_size = FONT_SIZES[*val];
                self.selected_font_size = *val;
            }
            AppEvent::ToggleUnderline => {
                self.text_decoration_line.toggle(TextDecorationLine::Underline)
            }
            AppEvent::ToggleOverline => {
                self.text_decoration_line.toggle(TextDecorationLine::Overline)
            }
            AppEvent::ToggleStrikethrough => {
                self.text_decoration_line.toggle(TextDecorationLine::Strikethrough)
            }
        })
    }
}
