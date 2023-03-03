use morphorm::Units;
use vizia_style::{
    BoxShadow, Clip, Color, ColorStop, FontSize, Gradient, Length, LengthOrPercentage, LengthValue,
    LinearGradient, Opacity, Rect, Transform, RGBA,
};

use femtovg::Transform2D;

/// A trait which describes a property which can be interpolated for animations.
pub trait Interpolator {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self;
}

// Implementations

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

impl Interpolator for RGBA {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        let r = (end.r() as f64 - start.r() as f64).mul_add(t as f64, start.r() as f64) as u8;
        let g = (end.g() as f64 - start.g() as f64).mul_add(t as f64, start.g() as f64) as u8;
        let b = (end.b() as f64 - start.b() as f64).mul_add(t as f64, start.b() as f64) as u8;
        let a = (end.a() as f64 - start.a() as f64).mul_add(t as f64, start.a() as f64) as u8;
        RGBA::rgba(r, g, b, a)
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
                return Length::Value(LengthValue::interpolate(start_val, end_val, t));
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
    fn interpolate(_start: &Self, end: &Self, _t: f32) -> Self {
        end.clone()
    }
}

// TODO: Split this into interpolated matrices for translation, rotation, scale, and skew
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

impl Interpolator for LinearGradient {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        if start.direction == end.direction && start.stops.len() == end.stops.len() {
            LinearGradient {
                direction: start.direction,
                stops: start
                    .stops
                    .iter()
                    .zip(end.stops.iter())
                    .enumerate()
                    .map(|(index, (start_stop, end_stop))| {
                        let num_stops = start.stops.len();
                        let start_pos = start_stop.position.clone().unwrap_or(
                            LengthOrPercentage::Percentage(index as f32 / (num_stops - 1) as f32),
                        );
                        let end_pos = end_stop.position.clone().unwrap_or(
                            LengthOrPercentage::Percentage(index as f32 / (num_stops - 1) as f32),
                        );
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

impl Interpolator for BoxShadow {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        BoxShadow {
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
        FontSize(f32::interpolate(&start.0, &end.0, t))
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

impl Interpolator for Clip {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (Clip::Shape(s), Clip::Shape(e)) => Clip::Shape(Rect::interpolate(s, e, t)),
            _ => end.clone(),
        }
    }
}
