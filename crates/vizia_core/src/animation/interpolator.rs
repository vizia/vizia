use morphorm::Units;
use vizia_style::{Color, Opacity, Length};

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

// impl Interpolator for Length {
//     fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        
//     }
// }
