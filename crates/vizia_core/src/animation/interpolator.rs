use morphorm::Units;
use vizia_style::{
    Angle, BackgroundSize, ClipPath, Color, ColorStop, Display, Filter, FontSize, Gradient, Length,
    LengthOrPercentage, LengthPercentageOrAuto, LengthValue, LineDirection, LinearGradient,
    Opacity, PercentageOrNumber, Rect, Scale, Shadow, Transform, Translate, RGBA,
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
        ((start + (end - start)) as f32 * t).round() as Self
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
            Self::Pixels(val) => val,
            Self::Percentage(val) => val,
            Self::Stretch(val) => val,
            Self::Auto => return *end,
        };

        match end {
            Self::Pixels(e) => Self::Pixels(f32::interpolate(s, e, t)),
            Self::Percentage(e) => Self::Percentage(f32::interpolate(s, e, t)),
            Self::Stretch(e) => Self::Stretch(f32::interpolate(s, e, t)),
            Self::Auto => *end,
        }
    }
}

impl Interpolator for Display {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        if t < 0.5 {
            *start
        } else {
            *end
        }
    }
}

impl Interpolator for Opacity {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        Self(start.0 + (end.0 - start.0) * t)
    }
}

impl Interpolator for Color {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        let r = (end.r() as f64 - start.r() as f64).mul_add(t as f64, start.r() as f64) as u8;
        let g = (end.g() as f64 - start.g() as f64).mul_add(t as f64, start.g() as f64) as u8;
        let b = (end.b() as f64 - start.b() as f64).mul_add(t as f64, start.b() as f64) as u8;
        let a = (end.a() as f64 - start.a() as f64).mul_add(t as f64, start.a() as f64) as u8;
        Self::rgba(r, g, b, a)
    }
}

impl Interpolator for RGBA {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        let r = (end.r() as f64 - start.r() as f64).mul_add(t as f64, start.r() as f64) as u8;
        let g = (end.g() as f64 - start.g() as f64).mul_add(t as f64, start.g() as f64) as u8;
        let b = (end.b() as f64 - start.b() as f64).mul_add(t as f64, start.b() as f64) as u8;
        let a = (end.a() as f64 - start.a() as f64).mul_add(t as f64, start.a() as f64) as u8;
        Self::rgba(r, g, b, a)
    }
}

impl Interpolator for Filter {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (Self::Blur(start), Self::Blur(end)) => {
                Self::Blur(Length::interpolate(start, end, t))
            }
        }
    }
}

impl Interpolator for LengthValue {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (end, start) {
            (Self::Px(end_val), Self::Px(start_val)) => {
                Self::Px(f32::interpolate(start_val, end_val, t))
            }

            _ => Self::default(),
        }
    }
}

impl Interpolator for Length {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (end, start) {
            (Self::Value(end_val), Self::Value(start_val)) => {
                Self::Value(LengthValue::interpolate(start_val, end_val, t))
            }

            _ => Self::default(),
        }
    }
}

impl Interpolator for LengthOrPercentage {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (Self::Length(start_val), Self::Length(end_val)) => {
                Self::Length(Length::interpolate(start_val, end_val, t))
            }

            (
                Self::Percentage(start_val),
                Self::Percentage(end_val),
            ) => Self::Percentage(f32::interpolate(start_val, end_val, t)),

            _ => Self::default(),
        }
    }
}

impl Interpolator for LengthPercentageOrAuto {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (
                Self::LengthPercentage(start_val),
                Self::LengthPercentage(end_val),
            ) => Self::LengthPercentage(LengthOrPercentage::interpolate(
                start_val, end_val, t,
            )),

            _ => end.clone(),
        }
    }
}

impl Interpolator for PercentageOrNumber {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (Self::Number(start_val), Self::Number(end_val)) => {
                Self::Number(f32::interpolate(start_val, end_val, t))
            }

            (
                Self::Percentage(start_val),
                Self::Percentage(end_val),
            ) => Self::Percentage(f32::interpolate(start_val, end_val, t)),

            _ => Self::default(),
        }
    }
}

impl Interpolator for Translate {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        let x = LengthOrPercentage::interpolate(&start.x, &end.x, t);
        let y = LengthOrPercentage::interpolate(&start.y, &end.y, t);
        Self { x, y }
    }
}

impl Interpolator for Scale {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        let x = PercentageOrNumber::interpolate(&start.x, &end.x, t);
        let y = PercentageOrNumber::interpolate(&start.y, &end.y, t);
        Self { x, y }
    }
}

impl Interpolator for Angle {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        let r = start.to_radians() + (end.to_radians() - start.to_radians()) * t;
        Self::Rad(r)
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

impl<T: Interpolator> Interpolator for Vec<T> {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        start
            .iter()
            .zip(end.iter())
            .map(|(start, end)| T::interpolate(start, end, t))
            .collect::<Self>()
    }
}

impl Interpolator for ImageOrGradient {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (
                Self::Gradient(gradient_start),
                Self::Gradient(gradient_end),
            ) => Self::Gradient(Gradient::interpolate(gradient_start, gradient_end, t)),
            _ => end.clone(),
        }
    }
}

impl Interpolator for BackgroundSize {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (
                Self::Explicit { width: start_width, height: start_height },
                Self::Explicit { width: end_width, height: end_height },
            ) => {
                let width = LengthPercentageOrAuto::interpolate(start_width, end_width, t);
                let height = LengthPercentageOrAuto::interpolate(start_height, end_height, t);
                Self::Explicit { width, height }
            }

            _ => end.clone(),
        }
    }
}

impl Interpolator for Gradient {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (Self::Linear(start_gradient), Self::Linear(end_gradient)) => {
                Self::Linear(LinearGradient::interpolate(start_gradient, end_gradient, t))
            }

            _ => end.clone(),
        }
    }
}

impl Interpolator for LineDirection {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (Self::Angle(start_angle), Self::Angle(end_angle)) => {
                Self::Angle(Angle::interpolate(start_angle, end_angle, t))
            }

            _ => *end,
        }
    }
}

impl Interpolator for LinearGradient {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        if start.stops.len() == end.stops.len() {
            Self {
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
        Self {
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
        Self(f32::interpolate(&start.0, &end.0, t))
    }
}

impl<T: Interpolator> Interpolator for Rect<T> {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        Self(
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
            (Self::Shape(s), Self::Shape(e)) => Self::Shape(Rect::interpolate(s, e, t)),
            _ => end.clone(),
        }
    }
}
