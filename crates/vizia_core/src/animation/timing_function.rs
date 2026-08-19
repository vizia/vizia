use vizia_style::{EasingFunction, StepPosition};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum TimingFunction {
    CubicBezier {
        x1: f32,
        x2: f32,
        y1: f32,
        y2: f32,
    },
    Steps {
        steps: u32,
        position: StepPosition,
    },
    /// Resolve to the animation-level timing function when sampling CSS keyframes.
    AnimationDefault,
}

impl Default for TimingFunction {
    fn default() -> Self {
        Self::ease_in_out()
    }
}

impl TimingFunction {
    pub fn linear() -> Self {
        Self::new(0., 0., 1., 1.)
    }
    pub fn ease() -> Self {
        Self::new(0.25, 0.1, 0.25, 1.)
    }
    pub fn ease_in() -> Self {
        Self::new(0.42, 0., 1., 1.)
    }
    pub fn ease_out() -> Self {
        Self::new(0., 0., 0.58, 1.)
    }
    pub fn ease_in_out() -> Self {
        Self::new(0.42, 0., 0.58, 1.)
    }

    pub fn from_easing(value: EasingFunction) -> Self {
        match value {
            EasingFunction::Linear => Self::linear(),
            EasingFunction::Ease => Self::ease(),
            EasingFunction::EaseIn => Self::ease_in(),
            EasingFunction::EaseOut => Self::ease_out(),
            EasingFunction::EaseInOut => Self::ease_in_out(),
            EasingFunction::CubicBezier(x1, y1, x2, y2) => Self::new(x1, y1, x2, y2),
            EasingFunction::Steps(steps, position) => Self::Steps { steps, position },
        }
    }

    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self::CubicBezier { x1, x2, y1, y2 }
    }

    pub fn value(&self, x: f32) -> f32 {
        self.value_with_before(x, false)
    }

    pub fn value_with_before(&self, x: f32, before: bool) -> f32 {
        match *self {
            Self::AnimationDefault => Self::ease().value_with_before(x, before),
            Self::Steps { steps, position } => {
                let steps = steps.max(1) as f32;
                let mut current = (x * steps).floor();
                if matches!(
                    position,
                    StepPosition::JumpStart | StepPosition::JumpBoth | StepPosition::Start
                ) {
                    current += 1.0;
                }
                if before && (x * steps).fract().abs() <= f32::EPSILON {
                    current -= 1.0;
                }
                if x >= 0.0 && current < 0.0 {
                    current = 0.0;
                }
                let jumps = match position {
                    StepPosition::JumpNone => steps - 1.0,
                    StepPosition::JumpBoth => steps + 1.0,
                    _ => steps,
                };
                if x <= 1.0 && current > jumps {
                    current = jumps;
                }
                current / jumps.max(1.0)
            }
            Self::CubicBezier { x1, x2, y1, y2 } => {
                if x1 == y1 && x2 == y2 {
                    return x;
                }
                Self::calc_bezier(Self::find_t_for_x(x, x1, x2), y1, y2)
            }
        }
    }

    fn calc_bezier(t: f32, a1: f32, a2: f32) -> f32 {
        let a = |a1: f32, a2: f32| 1.0 - 3.0 * a2 + 3.0 * a1;
        let b = |a1: f32, a2: f32| 3.0 * a2 - 6.0 * a1;
        let c = |a1: f32| 3.0 * a1;
        ((a(a1, a2) * t + b(a1, a2)) * t + c(a1)) * t
    }

    fn calc_bezier_slope(t: f32, a1: f32, a2: f32) -> f32 {
        let a = |a1: f32, a2: f32| 1.0 - 3.0 * a2 + 3.0 * a1;
        let b = |a1: f32, a2: f32| 3.0 * a2 - 6.0 * a1;
        let c = |a1: f32| 3.0 * a1;
        3.0 * a(a1, a2) * t * t + 2.0 * b(a1, a2) * t + c(a1)
    }

    fn find_t_for_x(x: f32, x1: f32, x2: f32) -> f32 {
        let mut guess = x.clamp(0.0, 1.0);
        for _ in 0..8 {
            let error = Self::calc_bezier(guess, x1, x2) - x;
            if error.abs() <= 0.0000001 {
                return guess;
            }
            let slope = Self::calc_bezier_slope(guess, x1, x2);
            if slope.abs() <= f32::EPSILON {
                break;
            }
            guess = (guess - error / slope).clamp(0.0, 1.0);
        }
        guess
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_and_bezier_presets() {
        assert_eq!(TimingFunction::linear().value(0.5), 0.5);
        assert!((TimingFunction::ease().value(0.25) - 0.4085106).abs() < 0.00001);
    }

    #[test]
    fn steps_follow_css_easing_boundaries() {
        let start = TimingFunction::Steps { steps: 4, position: StepPosition::JumpStart };
        let end = TimingFunction::Steps { steps: 4, position: StepPosition::JumpEnd };
        assert_eq!(start.value(0.0), 0.25);
        assert_eq!(start.value_with_before(0.0, true), 0.0);
        assert_eq!(end.value(0.0), 0.0);
        assert_eq!(end.value(1.0), 1.0);
        assert_eq!(end.value_with_before(0.5, true), 0.25);
    }
}
