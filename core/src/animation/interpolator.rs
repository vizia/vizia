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
