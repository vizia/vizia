
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