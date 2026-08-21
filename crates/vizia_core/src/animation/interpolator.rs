use morphorm::Units;
use vizia_style::{
    Angle, AnimationComposition, BackgroundRepeat, BackgroundSize, ClipPath, Color, ColorStop,
    Display, Filter, FontSize, Gradient, Length, LengthOrPercentage, LengthPercentageOrAuto,
    LengthValue, LetterSpacing, LineDirection, LineHeight, LinearGradient, Matrix as CssMatrix,
    Opacity, PercentageOrNumber, Position, RGBA, Rect, Scale, Shadow, Transform, Translate,
};

use skia_safe::Matrix;

use crate::style::ImageOrGradient;

/// A trait which describes how a property is interpolated for animations.
pub(crate) trait Interpolator {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self;
}

/// Property-specific Level 2 effect composition.
///
/// Types without a meaningful additive/accumulative operation intentionally use replacement.
pub(crate) trait Compositor: Clone {
    fn compose(underlying: &Self, effect: &Self, composition: AnimationComposition) -> Self;
}

macro_rules! replace_compositor {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl Compositor for $ty {
                fn compose(
                    _underlying: &Self,
                    effect: &Self,
                    _composition: AnimationComposition,
                ) -> Self {
                    effect.clone()
                }
            }
        )+
    };
}

fn add_length_or_percentage(
    underlying: &LengthOrPercentage,
    effect: &LengthOrPercentage,
) -> Option<LengthOrPercentage> {
    match (underlying, effect) {
        (LengthOrPercentage::Length(a), LengthOrPercentage::Length(b)) => match (a, b) {
            (Length::Value(LengthValue::Px(a)), Length::Value(LengthValue::Px(b))) => {
                Some(LengthOrPercentage::Length(Length::px(a + b)))
            }
            _ => None,
        },
        (LengthOrPercentage::Percentage(a), LengthOrPercentage::Percentage(b)) => {
            Some(LengthOrPercentage::Percentage(a + b))
        }
        _ => None,
    }
}

fn add_units(underlying: &Units, effect: &Units) -> Option<Units> {
    match (underlying, effect) {
        (Units::Pixels(a), Units::Pixels(b)) => Some(Units::Pixels(a + b)),
        (Units::Percentage(a), Units::Percentage(b)) => Some(Units::Percentage(a + b)),
        (Units::Stretch(a), Units::Stretch(b)) => Some(Units::Stretch(a + b)),
        _ => None,
    }
}

fn multiply_scale_component(
    underlying: PercentageOrNumber,
    effect: PercentageOrNumber,
) -> PercentageOrNumber {
    PercentageOrNumber::Number(underlying.to_factor() * effect.to_factor())
}

fn accumulate_transform(underlying: &Transform, effect: &Transform) -> Option<Transform> {
    match (underlying, effect) {
        (Transform::Translate((ax, ay)), Transform::Translate((bx, by))) => {
            Some(Transform::Translate((
                add_length_or_percentage(ax, bx)?,
                add_length_or_percentage(ay, by)?,
            )))
        }
        (Transform::TranslateX(a), Transform::TranslateX(b)) => {
            Some(Transform::TranslateX(add_length_or_percentage(a, b)?))
        }
        (Transform::TranslateY(a), Transform::TranslateY(b)) => {
            Some(Transform::TranslateY(add_length_or_percentage(a, b)?))
        }
        (Transform::Rotate(a), Transform::Rotate(b)) => {
            Some(Transform::Rotate(Angle::Rad(a.to_radians() + b.to_radians())))
        }
        (Transform::Scale((ax, ay)), Transform::Scale((bx, by))) => Some(Transform::Scale((
            multiply_scale_component(*ax, *bx),
            multiply_scale_component(*ay, *by),
        ))),
        (Transform::ScaleX(a), Transform::ScaleX(b)) => {
            Some(Transform::ScaleX(multiply_scale_component(*a, *b)))
        }
        (Transform::ScaleY(a), Transform::ScaleY(b)) => {
            Some(Transform::ScaleY(multiply_scale_component(*a, *b)))
        }
        (Transform::Skew(ax, ay), Transform::Skew(bx, by)) => Some(Transform::Skew(
            Angle::Rad(ax.to_radians() + bx.to_radians()),
            Angle::Rad(ay.to_radians() + by.to_radians()),
        )),
        (Transform::SkewX(a), Transform::SkewX(b)) => {
            Some(Transform::SkewX(Angle::Rad(a.to_radians() + b.to_radians())))
        }
        (Transform::SkewY(a), Transform::SkewY(b)) => {
            Some(Transform::SkewY(Angle::Rad(a.to_radians() + b.to_radians())))
        }
        _ => None,
    }
}

impl Compositor for f32 {
    fn compose(underlying: &Self, effect: &Self, composition: AnimationComposition) -> Self {
        match composition {
            AnimationComposition::Replace => *effect,
            AnimationComposition::Add | AnimationComposition::Accumulate => underlying + effect,
        }
    }
}

impl Compositor for i32 {
    fn compose(underlying: &Self, effect: &Self, composition: AnimationComposition) -> Self {
        match composition {
            AnimationComposition::Replace => *effect,
            AnimationComposition::Add | AnimationComposition::Accumulate => underlying + effect,
        }
    }
}

impl Compositor for Opacity {
    fn compose(underlying: &Self, effect: &Self, composition: AnimationComposition) -> Self {
        match composition {
            AnimationComposition::Replace => *effect,
            AnimationComposition::Add | AnimationComposition::Accumulate => {
                Opacity((underlying.0 + effect.0).clamp(0.0, 1.0))
            }
        }
    }
}

impl Compositor for Color {
    fn compose(underlying: &Self, effect: &Self, composition: AnimationComposition) -> Self {
        match composition {
            AnimationComposition::Replace => *effect,
            AnimationComposition::Add | AnimationComposition::Accumulate => Color::rgba(
                underlying.r().saturating_add(effect.r()),
                underlying.g().saturating_add(effect.g()),
                underlying.b().saturating_add(effect.b()),
                underlying.a().saturating_add(effect.a()),
            ),
        }
    }
}

impl Compositor for RGBA {
    fn compose(underlying: &Self, effect: &Self, composition: AnimationComposition) -> Self {
        match composition {
            AnimationComposition::Replace => *effect,
            AnimationComposition::Add | AnimationComposition::Accumulate => RGBA::rgba(
                underlying.r().saturating_add(effect.r()),
                underlying.g().saturating_add(effect.g()),
                underlying.b().saturating_add(effect.b()),
                underlying.a().saturating_add(effect.a()),
            ),
        }
    }
}

impl Compositor for Units {
    fn compose(underlying: &Self, effect: &Self, composition: AnimationComposition) -> Self {
        match composition {
            AnimationComposition::Replace => *effect,
            AnimationComposition::Add | AnimationComposition::Accumulate => {
                add_units(underlying, effect).unwrap_or(*effect)
            }
        }
    }
}

impl Compositor for LengthOrPercentage {
    fn compose(underlying: &Self, effect: &Self, composition: AnimationComposition) -> Self {
        match composition {
            AnimationComposition::Replace => effect.clone(),
            AnimationComposition::Add | AnimationComposition::Accumulate => {
                add_length_or_percentage(underlying, effect).unwrap_or_else(|| effect.clone())
            }
        }
    }
}

impl Compositor for Translate {
    fn compose(underlying: &Self, effect: &Self, composition: AnimationComposition) -> Self {
        match composition {
            AnimationComposition::Replace => effect.clone(),
            AnimationComposition::Add | AnimationComposition::Accumulate => Translate {
                x: add_length_or_percentage(&underlying.x, &effect.x)
                    .unwrap_or_else(|| effect.x.clone()),
                y: add_length_or_percentage(&underlying.y, &effect.y)
                    .unwrap_or_else(|| effect.y.clone()),
            },
        }
    }
}

impl Compositor for Scale {
    fn compose(underlying: &Self, effect: &Self, composition: AnimationComposition) -> Self {
        match composition {
            AnimationComposition::Replace => *effect,
            AnimationComposition::Add | AnimationComposition::Accumulate => Scale {
                x: multiply_scale_component(underlying.x, effect.x),
                y: multiply_scale_component(underlying.y, effect.y),
            },
        }
    }
}

impl Compositor for Angle {
    fn compose(underlying: &Self, effect: &Self, composition: AnimationComposition) -> Self {
        match composition {
            AnimationComposition::Replace => *effect,
            AnimationComposition::Add | AnimationComposition::Accumulate => {
                Angle::Rad(underlying.to_radians() + effect.to_radians())
            }
        }
    }
}

impl Compositor for Vec<Transform> {
    fn compose(underlying: &Self, effect: &Self, composition: AnimationComposition) -> Self {
        match composition {
            AnimationComposition::Replace => effect.clone(),
            AnimationComposition::Add => {
                let mut result = underlying.clone();
                result.extend(effect.iter().cloned());
                result
            }
            AnimationComposition::Accumulate => {
                if underlying.len() == effect.len() {
                    let mut result = Vec::with_capacity(effect.len());
                    for (a, b) in underlying.iter().zip(effect) {
                        let Some(value) = accumulate_transform(a, b) else {
                            let mut fallback = underlying.clone();
                            fallback.extend(effect.iter().cloned());
                            return fallback;
                        };
                        result.push(value);
                    }
                    result
                } else {
                    let mut result = underlying.clone();
                    result.extend(effect.iter().cloned());
                    result
                }
            }
        }
    }
}

impl Compositor for Filter {
    fn compose(underlying: &Self, effect: &Self, composition: AnimationComposition) -> Self {
        match composition {
            AnimationComposition::Replace => effect.clone(),
            AnimationComposition::Add => {
                fn append(filter: &Filter, out: &mut Vec<Filter>) {
                    match filter {
                        Filter::None => {}
                        Filter::List(values) => out.extend(values.iter().cloned()),
                        value => out.push(value.clone()),
                    }
                }
                let mut values = Vec::new();
                append(underlying, &mut values);
                append(effect, &mut values);
                match values.len() {
                    0 => Filter::None,
                    1 => values.remove(0),
                    _ => Filter::List(values),
                }
            }
            AnimationComposition::Accumulate => match (underlying, effect) {
                (Filter::Blur(a), Filter::Blur(b)) => {
                    let a = LengthOrPercentage::Length(a.clone());
                    let b = LengthOrPercentage::Length(b.clone());
                    match add_length_or_percentage(&a, &b) {
                        Some(LengthOrPercentage::Length(value)) => Filter::Blur(value),
                        _ => effect.clone(),
                    }
                }
                (Filter::List(a), Filter::List(b)) if a.len() == b.len() => Filter::List(
                    a.iter()
                        .zip(b)
                        .map(|(a, b)| Filter::compose(a, b, AnimationComposition::Accumulate))
                        .collect(),
                ),
                _ => Filter::compose(underlying, effect, AnimationComposition::Add),
            },
        }
    }
}

replace_compositor!(
    (f32, f32),
    Display,
    ClipPath,
    LengthValue,
    Length,
    LengthPercentageOrAuto,
    PercentageOrNumber,
    ImageOrGradient,
    BackgroundSize,
    BackgroundRepeat,
    Position,
    FontSize,
    LetterSpacing,
    LineHeight,
    Shadow,
    Gradient,
    LinearGradient,
    Matrix,
);

replace_compositor!(
    Vec<ImageOrGradient>,
    Vec<Position>,
    Vec<BackgroundRepeat>,
    Vec<BackgroundSize>,
    Vec<Shadow>,
);

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
            (Filter::None, Filter::None) => Filter::None,
            (Filter::Blur(start), Filter::Blur(end)) => {
                Filter::Blur(Length::interpolate(start, end, t))
            }
            (Filter::List(start), Filter::List(end)) if start.len() == end.len() => {
                let compatible = start.iter().zip(end).all(|(a, b)| {
                    matches!(
                        (a, b),
                        (Filter::Blur(_), Filter::Blur(_)) | (Filter::None, Filter::None)
                    )
                });
                if compatible {
                    Filter::List(
                        start.iter().zip(end).map(|(a, b)| Filter::interpolate(a, b, t)).collect(),
                    )
                } else if t < 0.5 {
                    Filter::List(start.clone())
                } else {
                    Filter::List(end.clone())
                }
            }
            _ if t < 0.5 => start.clone(),
            _ => end.clone(),
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
        match (start, end) {
            (LengthOrPercentage::Length(start_val), LengthOrPercentage::Length(end_val)) => {
                LengthOrPercentage::Length(Length::interpolate(start_val, end_val, t))
            }

            (
                LengthOrPercentage::Percentage(start_val),
                LengthOrPercentage::Percentage(end_val),
            ) => LengthOrPercentage::Percentage(f32::interpolate(start_val, end_val, t)),

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
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        match (start, end) {
            (Transform::Translate((sx, sy)), Transform::Translate((ex, ey))) => {
                Transform::Translate((
                    LengthOrPercentage::interpolate(sx, ex, t),
                    LengthOrPercentage::interpolate(sy, ey, t),
                ))
            }
            (Transform::TranslateX(start), Transform::TranslateX(end)) => {
                Transform::TranslateX(LengthOrPercentage::interpolate(start, end, t))
            }
            (Transform::TranslateY(start), Transform::TranslateY(end)) => {
                Transform::TranslateY(LengthOrPercentage::interpolate(start, end, t))
            }
            (Transform::Scale((sx, sy)), Transform::Scale((ex, ey))) => Transform::Scale((
                PercentageOrNumber::interpolate(sx, ex, t),
                PercentageOrNumber::interpolate(sy, ey, t),
            )),
            (Transform::ScaleX(start), Transform::ScaleX(end)) => {
                Transform::ScaleX(PercentageOrNumber::interpolate(start, end, t))
            }
            (Transform::ScaleY(start), Transform::ScaleY(end)) => {
                Transform::ScaleY(PercentageOrNumber::interpolate(start, end, t))
            }
            (Transform::Rotate(start), Transform::Rotate(end)) => {
                Transform::Rotate(Angle::interpolate(start, end, t))
            }
            (Transform::Skew(sx, sy), Transform::Skew(ex, ey)) => {
                Transform::Skew(Angle::interpolate(sx, ex, t), Angle::interpolate(sy, ey, t))
            }
            (Transform::SkewX(start), Transform::SkewX(end)) => {
                Transform::SkewX(Angle::interpolate(start, end, t))
            }
            (Transform::SkewY(start), Transform::SkewY(end)) => {
                Transform::SkewY(Angle::interpolate(start, end, t))
            }
            (Transform::Matrix(start), Transform::Matrix(end)) => {
                Transform::Matrix(CssMatrix::new(
                    f32::interpolate(&start.a, &end.a, t),
                    f32::interpolate(&start.b, &end.b, t),
                    f32::interpolate(&start.c, &end.c, t),
                    f32::interpolate(&start.d, &end.d, t),
                    f32::interpolate(&start.e, &end.e, t),
                    f32::interpolate(&start.f, &end.f, t),
                ))
            }
            _ if t < 0.5 => start.clone(),
            _ => end.clone(),
        }
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
            .collect::<Vec<T>>()
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

#[cfg(test)]
mod composition_tests {
    use super::*;

    #[test]
    fn translate_add_composes_with_underlying_value() {
        let base = Translate {
            x: LengthOrPercentage::Length(Length::px(10.0)),
            y: LengthOrPercentage::Length(Length::px(5.0)),
        };
        let effect = Translate {
            x: LengthOrPercentage::Length(Length::px(20.0)),
            y: LengthOrPercentage::Length(Length::px(7.0)),
        };
        let result = Translate::compose(&base, &effect, AnimationComposition::Add);
        assert_eq!(result.x, LengthOrPercentage::Length(Length::px(30.0)));
        assert_eq!(result.y, LengthOrPercentage::Length(Length::px(12.0)));
    }

    #[test]
    fn transform_interpolates_compatible_translate_functions() {
        let start = Transform::TranslateX(LengthOrPercentage::Length(Length::px(-100.0)));
        let end = Transform::TranslateX(LengthOrPercentage::Length(Length::px(100.0)));
        let mid = Transform::interpolate(&start, &end, 0.5);

        let Transform::TranslateX(LengthOrPercentage::Length(Length::Value(LengthValue::Px(
            value,
        )))) = mid
        else {
            panic!("expected interpolated translateX");
        };
        assert!(value.abs() < 0.001);
    }

    #[test]
    fn transform_add_concatenates_effect_lists_in_order() {
        let base = vec![Transform::TranslateX(LengthOrPercentage::Length(Length::px(10.0)))];
        let effect = vec![Transform::Rotate(Angle::Deg(45.0))];
        let result = Vec::<Transform>::compose(&base, &effect, AnimationComposition::Add);
        assert_eq!(result.len(), 2);
        assert!(matches!(result[0], Transform::TranslateX(_)));
        assert!(matches!(result[1], Transform::Rotate(_)));
    }

    #[test]
    fn scale_accumulate_uses_property_specific_multiplication() {
        let base = Scale::new(2.0, 3.0);
        let effect = Scale::new(1.5, 0.5);
        let result = Scale::compose(&base, &effect, AnimationComposition::Accumulate);
        assert!((result.x.to_factor() - 3.0).abs() < 0.001);
        assert!((result.y.to_factor() - 1.5).abs() < 0.001);
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
                | (LengthOrPercentage::Percentage(_), LengthOrPercentage::Percentage(_)) => {
                    vizia_style::PositionComponent::Length(LengthOrPercentage::interpolate(
                        &start_value,
                        &end_value,
                        t,
                    ))
                }

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
