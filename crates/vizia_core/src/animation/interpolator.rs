use morphorm::Units;
use vizia_style::{Color, Length, LengthOrPercentage, LengthValue, Opacity, Transform};

use crate::style::Transform2D;

/// A trait which describes a property which can be interpolated for animations.
pub trait Interpolator {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self;
}

// Implementations

impl Interpolator for f32 {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        return start + (end - start) * t;
    }
}

impl Interpolator for i32 {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        return ((start + (end - start)) as f32 * t).round() as i32;
    }
}

impl Interpolator for (f32, f32) {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        return (f32::interpolate(&start.0, &end.0, t), f32::interpolate(&start.1, &end.1, t));
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
            Units::Auto => return *end,
        }
    }
}

impl Interpolator for Opacity {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        return Opacity(start.0 + (end.0 - start.0) * t);
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

impl Interpolator for LengthValue {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (end, start) {
            (LengthValue::Px(end_val), LengthValue::Px(start_val)) => {
                return LengthValue::Px(f32::interpolate(start_val, end_val, t));
            }

            _ => {}
        }

        LengthValue::default()
    }
}

impl Interpolator for Length {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (end, start) {
            (Length::Value(end_val), Length::Value(start_val)) => {
                return Length::Value(LengthValue::interpolate(end_val, start_val, t));
            }

            _ => {}
        }

        Length::default()
    }
}

impl Interpolator for LengthOrPercentage {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (LengthOrPercentage::Length(start_val), LengthOrPercentage::Length(end_val)) => {
                return LengthOrPercentage::Length(Length::interpolate(start_val, end_val, t));
            }

            (
                LengthOrPercentage::Percentage(start_val),
                LengthOrPercentage::Percentage(end_val),
            ) => {
                return LengthOrPercentage::Percentage(f32::interpolate(start_val, end_val, t));
            }

            _ => {}
        }

        LengthOrPercentage::default()
    }
}

impl Interpolator for Transform {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        end.clone()
    }
}

impl Interpolator for Transform2D {
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
            .collect::<Vec<T>>()
    }
}
