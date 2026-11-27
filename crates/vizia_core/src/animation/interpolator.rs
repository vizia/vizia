use morphorm::Units;
use vizia_style::{
    Angle, BackgroundRepeat, BackgroundSize, ClipPath, Color, ColorStop, Display, Filter, FontSize,
    Gradient, Length, LengthOrPercentage, LengthPercentageOrAuto, LengthValue, LetterSpacing,
    LineDirection, LineHeight, LinearGradient, Opacity, PercentageOrNumber, Position, RGBA, Rect,
    Scale, Shadow, Transform, Translate,
};

use skia_safe::Matrix;

use crate::style::ImageOrGradient;

/// A trait which describes how a property is interpolated for animations.
pub(crate) trait Interpolator {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self;
}

// Implementations of `Interpolator` for various properties.
impl Interpolator for f32 {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        start + (end - start) * t
    }
}

impl Interpolator for i32 {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        ((start + (end - start)) as f32 * t).round() as i32
    }
}

impl Interpolator for (f32, f32) {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        (f32::interpolate(&start.0, &end.0, t), f32::interpolate(&start.1, &end.1, t))
    }
}

impl Interpolator for Units {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        let s = match start {
            Units::Pixels(val) => val,
            Units::Percentage(val) => val,
            Units::Stretch(val) => val,
            Units::Auto => return *end,
        };

        match end {
            Units::Pixels(e) => Units::Pixels(f32::interpolate(s, e, t)),
            Units::Percentage(e) => Units::Percentage(f32::interpolate(s, e, t)),
            Units::Stretch(e) => Units::Stretch(f32::interpolate(s, e, t)),
            Units::Auto => *end,
        }
    }
}

impl Interpolator for Display {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        if t < 0.5 { *start } else { *end }
    }
}

impl Interpolator for Opacity {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        Opacity(start.0 + (end.0 - start.0) * t)
    }
}

impl Interpolator for Color {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        let r = (end.r() as f64 - start.r() as f64).mul_add(t as f64, start.r() as f64) as u8;
        let g = (end.g() as f64 - start.g() as f64).mul_add(t as f64, start.g() as f64) as u8;
        let b = (end.b() as f64 - start.b() as f64).mul_add(t as f64, start.b() as f64) as u8;
        let a = (end.a() as f64 - start.a() as f64).mul_add(t as f64, start.a() as f64) as u8;
        Color::rgba(r, g, b, a)
    }
}

impl Interpolator for RGBA {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        let r = (end.r() as f64 - start.r() as f64).mul_add(t as f64, start.r() as f64) as u8;
        let g = (end.g() as f64 - start.g() as f64).mul_add(t as f64, start.g() as f64) as u8;
        let b = (end.b() as f64 - start.b() as f64).mul_add(t as f64, start.b() as f64) as u8;
        let a = (end.a() as f64 - start.a() as f64).mul_add(t as f64, start.a() as f64) as u8;
        RGBA::rgba(r, g, b, a)
    }
}

impl Interpolator for Filter {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (Filter::Blur(start), Filter::Blur(end)) => {
                Filter::Blur(Length::interpolate(start, end, t))
            }
        }
    }
}

impl Interpolator for LengthValue {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (end, start) {
            (LengthValue::Px(end_val), LengthValue::Px(start_val)) => {
                LengthValue::Px(f32::interpolate(start_val, end_val, t))
            }

            _ => LengthValue::default(),
        }
    }
}

impl Interpolator for Length {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (end, start) {
            (Length::Value(end_val), Length::Value(start_val)) => {
                Length::Value(LengthValue::interpolate(start_val, end_val, t))
            }

            _ => Length::default(),
        }
    }
}

impl Interpolator for LengthOrPercentage {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        fn components(value: &LengthOrPercentage) -> Option<(f32, f32)> {
            match value {
                LengthOrPercentage::Length(length) => length.to_px().map(|px| (0.0, px)),
                LengthOrPercentage::Percentage(percentage) => Some((*percentage, 0.0)),
                LengthOrPercentage::Calc { percentage, px } => Some((*percentage, *px)),
            }
        }

        match (start, end) {
            (LengthOrPercentage::Length(start_val), LengthOrPercentage::Length(end_val)) => {
                LengthOrPercentage::Length(Length::interpolate(start_val, end_val, t))
            }

            (
                LengthOrPercentage::Percentage(start_val),
                LengthOrPercentage::Percentage(end_val),
            ) => LengthOrPercentage::Percentage(f32::interpolate(start_val, end_val, t)),

            (LengthOrPercentage::Calc { .. }, _) | (_, LengthOrPercentage::Calc { .. }) => {
                if let (Some((start_percent, start_px)), Some((end_percent, end_px))) =
                    (components(start), components(end))
                {
                    LengthOrPercentage::Calc {
                        percentage: f32::interpolate(&start_percent, &end_percent, t),
                        px: f32::interpolate(&start_px, &end_px, t),
                    }
                } else if t < 0.5 {
                    start.clone()
                } else {
                    end.clone()
                }
            }

            _ => LengthOrPercentage::default(),
        }
    }
}

impl Interpolator for LengthPercentageOrAuto {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (
                LengthPercentageOrAuto::LengthPercentage(start_val),
                LengthPercentageOrAuto::LengthPercentage(end_val),
            ) => LengthPercentageOrAuto::LengthPercentage(LengthOrPercentage::interpolate(
                start_val, end_val, t,
            )),

            _ => end.clone(),
        }
    }
}

impl Interpolator for PercentageOrNumber {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (PercentageOrNumber::Number(start_val), PercentageOrNumber::Number(end_val)) => {
                PercentageOrNumber::Number(f32::interpolate(start_val, end_val, t))
            }

            (
                PercentageOrNumber::Percentage(start_val),
                PercentageOrNumber::Percentage(end_val),
            ) => PercentageOrNumber::Percentage(f32::interpolate(start_val, end_val, t)),

            _ => PercentageOrNumber::default(),
        }
    }
}

impl Interpolator for Translate {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        let x = LengthOrPercentage::interpolate(&start.x, &end.x, t);
        let y = LengthOrPercentage::interpolate(&start.y, &end.y, t);
        Translate { x, y }
    }
}

impl Interpolator for Scale {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        let x = PercentageOrNumber::interpolate(&start.x, &end.x, t);
        let y = PercentageOrNumber::interpolate(&start.y, &end.y, t);
        Scale { x, y }
    }
}

impl Interpolator for Angle {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        let r = start.to_radians() + (end.to_radians() - start.to_radians()) * t;
        Angle::Rad(r)
    }
}

impl Interpolator for Transform {
    fn interpolate(_start: &Self, end: &Self, _t: f32) -> Self {
        end.clone()
    }
}

// TODO: Split this into interpolated matrices for translation, rotation, scale, and skew
impl Interpolator for Matrix {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        let mut transform = *start;

        transform[0] = f32::interpolate(&start[0], &end[0], t);
        transform[1] = f32::interpolate(&start[1], &end[1], t);
        transform[2] = f32::interpolate(&start[2], &end[2], t);
        transform[3] = f32::interpolate(&start[3], &end[3], t);
        transform[4] = f32::interpolate(&start[4], &end[4], t);
        transform[5] = f32::interpolate(&start[5], &end[5], t);

        transform
    }
}

macro_rules! interpolate_vec {
    ($($item:ty),+ $(,)?) => {
        $(
            impl Interpolator for Vec<$item> {
                fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
                    start.iter().zip(end.iter())
                        .map(|(start, end)| <$item>::interpolate(start, end, t))
                        .collect()
                }
            }
        )+
    };
}

interpolate_vec!(Transform, ImageOrGradient, Position, BackgroundRepeat, BackgroundSize);

impl Interpolator for Vec<Shadow> {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        fn transparent_shadow(reference: &Shadow) -> Shadow {
            let color = match reference.color {
                Some(Color::RGBA(color)) => Color::rgba(color.r(), color.g(), color.b(), 0),
                _ => Color::rgba(0, 0, 0, 0),
            };
            Shadow::new(
                Length::zero(),
                Length::zero(),
                reference.blur_radius.as_ref().map(|_| Length::zero()),
                reference.spread_radius.as_ref().map(|_| Length::zero()),
                Some(color),
                reference.inset,
            )
        }

        (0..start.len().max(end.len()))
            .filter_map(|index| match (start.get(index), end.get(index)) {
                (Some(start), Some(end)) => Some(Shadow::interpolate(start, end, t)),
                (Some(start), None) if matches!(start.color, None | Some(Color::CurrentColor)) => {
                    if t < 0.5 {
                        Some(start.clone())
                    } else {
                        None
                    }
                }
                (None, Some(end)) if matches!(end.color, None | Some(Color::CurrentColor)) => {
                    if t < 0.5 { None } else { Some(end.clone()) }
                }
                (Some(start), None) => {
                    Some(Shadow::interpolate(start, &transparent_shadow(start), t))
                }
                (None, Some(end)) => Some(Shadow::interpolate(&transparent_shadow(end), end, t)),
                (None, None) => unreachable!(),
            })
            .collect()
    }
}

impl Interpolator for ImageOrGradient {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (
                ImageOrGradient::Gradient(gradient_start),
                ImageOrGradient::Gradient(gradient_end),
            ) => ImageOrGradient::Gradient(Gradient::interpolate(gradient_start, gradient_end, t)),
            _ => end.clone(),
        }
    }
}

impl Interpolator for BackgroundSize {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (
                BackgroundSize::Explicit { width: start_width, height: start_height },
                BackgroundSize::Explicit { width: end_width, height: end_height },
            ) => {
                let width = LengthPercentageOrAuto::interpolate(start_width, end_width, t);
                let height = LengthPercentageOrAuto::interpolate(start_height, end_height, t);
                BackgroundSize::Explicit { width, height }
            }

            _ => end.clone(),
        }
    }
}

impl Interpolator for BackgroundRepeat {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        if t < 0.5 { *start } else { *end }
    }
}

impl Interpolator for Position {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        fn interpolate_axis<T: Copy + Into<LengthOrPercentage>>(
            start: &vizia_style::PositionComponent<T>,
            end: &vizia_style::PositionComponent<T>,
            t: f32,
        ) -> vizia_style::PositionComponent<T> {
            let start_value = start.to_length_or_percentage();
            let end_value = end.to_length_or_percentage();

            match (&start_value, &end_value) {
                (LengthOrPercentage::Length(_), LengthOrPercentage::Length(_))
                | (LengthOrPercentage::Percentage(_), LengthOrPercentage::Percentage(_))
                | (LengthOrPercentage::Calc { .. }, _)
                | (_, LengthOrPercentage::Calc { .. }) => vizia_style::PositionComponent::Length(
                    LengthOrPercentage::interpolate(&start_value, &end_value, t),
                ),

                _ => {
                    if t < 0.5 {
                        start.clone()
                    } else {
                        end.clone()
                    }
                }
            }
        }

        Position {
            x: interpolate_axis(&start.x, &end.x, t),
            y: interpolate_axis(&start.y, &end.y, t),
        }
    }
}

impl Interpolator for Gradient {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (Gradient::Linear(start_gradient), Gradient::Linear(end_gradient)) => {
                Gradient::Linear(LinearGradient::interpolate(start_gradient, end_gradient, t))
            }

            _ => end.clone(),
        }
    }
}

impl Interpolator for LineDirection {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (LineDirection::Angle(start_angle), LineDirection::Angle(end_angle)) => {
                LineDirection::Angle(Angle::interpolate(start_angle, end_angle, t))
            }

            _ => *end,
        }
    }
}

impl Interpolator for LinearGradient {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        if start.stops.len() == end.stops.len() {
            LinearGradient {
                direction: LineDirection::interpolate(&start.direction, &end.direction, t),
                stops: start
                    .stops
                    .iter()
                    .zip(end.stops.iter())
                    .enumerate()
                    .map(|(index, (start_stop, end_stop))| {
                        let num_stops = start.stops.len();
                        let start_pos =
                            start_stop.position.clone().unwrap_or(LengthOrPercentage::Percentage(
                                index as f32 / (num_stops - 1) as f32 * 100.0,
                            ));
                        let end_pos =
                            end_stop.position.clone().unwrap_or(LengthOrPercentage::Percentage(
                                index as f32 / (num_stops - 1) as f32 * 100.0,
                            ));
                        ColorStop {
                            color: Color::interpolate(&start_stop.color, &end_stop.color, t),
                            position: Some(LengthOrPercentage::interpolate(
                                &start_pos, &end_pos, t,
                            )),
                        }
                    })
                    .collect::<Vec<_>>(),
            }
        } else {
            end.clone()
        }
    }
}

impl Interpolator for Shadow {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        Shadow {
            x_offset: Length::interpolate(&start.x_offset, &end.x_offset, t),
            y_offset: Length::interpolate(&start.y_offset, &end.y_offset, t),
            blur_radius: Option::interpolate(&start.blur_radius, &end.blur_radius, t),
            spread_radius: Option::interpolate(&start.spread_radius, &end.spread_radius, t),
            color: Option::interpolate(&start.color, &end.color, t),
            inset: end.inset,
        }
    }
}

impl<T: Interpolator + Clone + Default> Interpolator for Option<T> {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (Some(s), Some(e)) => Some(T::interpolate(s, e, t)),
            (None, Some(e)) => Some(T::interpolate(&T::default(), e, t)),
            (Some(s), None) => Some(T::interpolate(s, &T::default(), t)),
            _ => end.clone(),
        }
    }
}

impl Interpolator for FontSize {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        FontSize(Length::interpolate(&start.0, &end.0, t))
    }
}

impl Interpolator for LetterSpacing {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (LetterSpacing::Normal, LetterSpacing::Normal) => LetterSpacing::Normal,
            (LetterSpacing::Length(start), LetterSpacing::Length(end)) => {
                LetterSpacing::Length(Length::interpolate(start, end, t))
            }
            _ => end.clone(),
        }
    }
}

impl Interpolator for LineHeight {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (LineHeight::Normal, LineHeight::Normal) => LineHeight::Normal,
            (LineHeight::Number(start), LineHeight::Number(end)) => {
                LineHeight::Number(f32::interpolate(start, end, t))
            }
            (LineHeight::Percentage(start), LineHeight::Percentage(end)) => {
                LineHeight::Percentage(f32::interpolate(start, end, t))
            }
            (LineHeight::Length(start), LineHeight::Length(end)) => {
                LineHeight::Length(Length::interpolate(start, end, t))
            }
            _ => end.clone(),
        }
    }
}

impl<T: Interpolator> Interpolator for Rect<T> {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        Rect(
            T::interpolate(&start.0, &end.0, t),
            T::interpolate(&start.1, &end.1, t),
            T::interpolate(&start.2, &end.2, t),
            T::interpolate(&start.3, &end.3, t),
        )
    }
}

impl Interpolator for ClipPath {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (ClipPath::Shape(s), ClipPath::Shape(e)) => ClipPath::Shape(Rect::interpolate(s, e, t)),
            _ => end.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn background_edge_offsets_interpolate_without_losing_percentage() {
        let edge = |px| {
            Position::new(
                vizia_style::HorizontalPosition::Length(LengthOrPercentage::Calc {
                    percentage: 100.0,
                    px,
                }),
                vizia_style::VerticalPosition::Center,
            )
        };
        let start = edge(-10.0);
        let end = edge(-20.0);
        let middle = Position::interpolate(&start, &end, 0.5);
        assert_eq!(middle.x.to_length_or_percentage().to_pixels(100.0, 1.0), 85.0);
        assert_eq!(middle.x.to_length_or_percentage().to_pixels(200.0, 2.0), 170.0);

        let middle = Position::interpolate(&start, &Position::center(), 0.5);
        assert_eq!(middle.x.to_length_or_percentage().to_pixels(100.0, 1.0), 70.0);
        assert_eq!(middle.x.to_length_or_percentage().to_pixels(200.0, 2.0), 140.0);
        assert!(Position::interpolate(&start, &Position::center(), 1.0).is_center());
    }

    #[test]
    fn standard_box_shadow_none_transitions_preserve_visible_shadows() {
        let shadow = Shadow::new(
            Length::px(20.0),
            Length::zero(),
            Some(Length::px(4.0)),
            None,
            Some(Color::rgb(255, 0, 0)),
            true,
        );
        let visible = vec![shadow.clone()];
        let empty = vec![];
        for result in [
            Vec::<Shadow>::interpolate(&empty, &visible, 0.5),
            Vec::<Shadow>::interpolate(&visible, &empty, 0.5),
        ] {
            assert_eq!(result.len(), 1);
            assert_eq!(result[0].x_offset, Length::px(10.0));
            assert_eq!(result[0].blur_radius, Some(Length::px(2.0)));
            assert_eq!(result[0].color, Some(Color::rgba(255, 0, 0, 127)));
            assert!(result[0].inset);
        }

        let current = vec![Shadow { color: None, ..shadow }];
        assert!(Vec::<Shadow>::interpolate(&empty, &current, 0.25).is_empty());
        assert_eq!(Vec::<Shadow>::interpolate(&empty, &current, 0.75), current);
    }
}
