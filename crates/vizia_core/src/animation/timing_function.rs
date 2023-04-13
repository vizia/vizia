use std::f32::EPSILON;

#[derive(Debug, Clone, Copy)]
pub enum TimingFunction {
    Linear,

    EaseInSine,
    EaseOutSine,
    EaseInOutSine,

    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,

    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,

    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,

    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,

    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,

    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,

    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,

    EaseInBack,
    EaseOutBack,
    EaseInOutBack,

    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,

    CubicBezier { ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32 },
}

impl Default for TimingFunction {
    fn default() -> Self {
        Self::Linear
    }
}

impl TimingFunction {
    pub fn ease() -> Self {
        Self::new_cubic(0.25, 0.1, 0.25, 1.)
    }
    pub fn ease_in() -> Self {
        Self::new_cubic(0.42, 0., 1., 1.)
    }
    pub fn ease_out() -> Self {
        Self::new_cubic(0., 0., 0.58, 1.)
    }
    pub fn ease_in_out() -> Self {
        Self::new_cubic(0.42, 0., 0.58, 1.)
    }
}

impl TimingFunction {
    pub fn new_cubic(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        let cx = 3.0 * x1;
        let bx = 3.0 * (x2 - x1) - cx;
        let ax = 1.0 - cx - bx;

        let cy = 3.0 * y1;
        let by = 3.0 * (y2 - y1) - cy;
        let ay = 1.0 - cy - by;

        Self::CubicBezier { ax, ay, bx, by, cx, cy }
    }

    #[allow(non_upper_case_globals)]
    pub fn value(&self, x: f32) -> f32 {
        match *self {
            Self::Linear => x,
            Self::EaseInSine => 1. - f32::cos((x * std::f32::consts::PI) / 2.),
            Self::EaseOutSine => f32::cos((x * std::f32::consts::PI) / 2.),
            Self::EaseInOutSine => -(f32::cos(std::f32::consts::PI * x) - 1.) / 2.,
            Self::EaseInCubic => x * x * x,
            Self::EaseOutCubic => 1. - f32::powi(1. - x, 3),
            Self::EaseInOutCubic => {
                if x < 0.5 {
                    4. * x * x * x
                } else {
                    1. - f32::powi(-2. * x + 2., 3) / 2.
                }
            }
            Self::EaseInQuint => x * x * x * x,
            Self::EaseOutQuint => 1. - f32::powi(1. - x, 5),
            Self::EaseInOutQuint => {
                if x < 0.5 {
                    16. * x * x * x * x * x
                } else {
                    1. - f32::powi(-2. * x + 2., 5) / 2.
                }
            }
            Self::EaseInCirc => 1. - f32::sqrt(1. - f32::powi(x, 2)),
            Self::EaseOutCirc => f32::sqrt(1. - f32::powi(x - 1., 2)),
            Self::EaseInOutCirc => {
                if x < 0.5 {
                    (1. - f32::sqrt(1. - f32::powi(2. * x, 2))) / 2.
                } else {
                    (f32::sqrt(1. - f32::powi(-2. * x + 2., 2)) + 1.) / 2.
                }
            }
            Self::EaseInElastic => {
                const c4: f32 = (2. * std::f32::consts::PI) / 3.;
                if x == 0. {
                    0.
                } else if x == 1. {
                    1.
                } else {
                    -f32::powf(2., 10. * x - 10.) * f32::sin((x * 10. - 10.75) * c4)
                }
            }
            Self::EaseOutElastic => {
                const c4: f32 = (2. * std::f32::consts::PI) / 3.;
                if x == 0. {
                    0.
                } else if x == 1. {
                    1.
                } else {
                    f32::powf(2., -10. * x) * f32::sin((x * 10. - 0.75) * c4) + 1.
                }
            }
            Self::EaseInOutElastic => {
                const c5: f32 = (2. * std::f32::consts::PI) / 4.5;
                if x == 0. {
                    0.
                } else if x == 1. {
                    1.
                } else if x < 0.5 {
                    -(f32::powf(2., 20. * x - 10.) * f32::sin((20. * x - 11.125) * c5)) / 2.
                } else {
                    (f32::powf(2., -20. * x + 10.) * f32::sin((20. * x - 11.125) * c5)) / 2. + 1.
                }
            }
            Self::EaseInQuad => x * x,
            Self::EaseOutQuad => 1. - (1. - x) * (1. - x),
            Self::EaseInOutQuad => {
                if x < 0.5 {
                    2. * x * x
                } else {
                    1. - f32::powi(-2. * x + 2., 2) / 2.
                }
            }
            Self::EaseInQuart => x * x * x * x,
            Self::EaseOutQuart => 1. - f32::powi(1. - x, 4),
            Self::EaseInOutQuart => {
                if x < 0.5 {
                    8. * x * x * x * x
                } else {
                    1. - f32::powi(-2. * x + 2., 4) / 2.
                }
            }
            Self::EaseInExpo => {
                if x == 0. {
                    0.
                } else {
                    f32::powf(2., 10. * x - 10.)
                }
            }
            Self::EaseOutExpo => {
                if x == 1. {
                    1.
                } else {
                    1. - f32::powf(2., -10. * x)
                }
            }
            Self::EaseInOutExpo => {
                if x == 0. {
                    0.
                } else if x == 1. {
                    1.
                } else if x < 0.5 {
                    f32::powf(2., 20. * x - 10.) / 2.
                } else {
                    (2. - f32::powf(2., -20. * x + 10.)) / 2.
                }
            }
            Self::EaseInBack => {
                const c1: f32 = 1.70158;
                const c3: f32 = c1 + 1.;
                c3 * x * x * x - c1 * x * x
            }
            Self::EaseOutBack => {
                const c1: f32 = 1.70158;
                const c3: f32 = c1 + 1.;
                1. + c3 * f32::powi(x - 1., 3) + c1 * f32::powi(x - 1., 2)
            }
            Self::EaseInOutBack => {
                const c1: f32 = 1.70158;
                const c2: f32 = c1 * 1.525;

                if x < 0.5 {
                    (f32::powi(2. * x, 2) * ((c2 + 1.) * 2. * x - c2)) / 2.
                } else {
                    (f32::powi(2. * x - 2., 2) * ((c2 + 1.) * (x * 2. - 2.) + c2) + 2.) / 2.
                }
            }
            Self::EaseInBounce => 1. - ease_out_bounce(1. - x),
            Self::EaseOutBounce => ease_out_bounce(x),
            Self::EaseInOutBounce => {
                if x < 0.5 {
                    (1. - ease_out_bounce(1. - 2. * x)) / 2.
                } else {
                    (1. + ease_out_bounce(2. * x - 1.)) / 2.
                }
            }
            Self::CubicBezier { ax, ay, bx, by, cx, cy } => {
                // Newton Raphson
                let mut guess = x;
                let mut error = x - calc_bezier(guess, ax, bx, cx);
                for _ in 0..8 {
                    if error.abs() <= EPSILON {
                        return calc_bezier(guess, ay, by, cy);
                    }

                    let pos = calc_bezier(guess, ax, bx, cx);
                    error = pos - x;

                    let derivative = calc_derivative(guess, ax, bx, cx);
                    guess -= error / derivative;
                }

                // Bisect
                let mut t0 = 0.;
                let mut t1 = 1.;

                while error.abs() > EPSILON {
                    guess = (t0 + t1) / 2.0;
                    error = x - calc_bezier(guess, ax, bx, cx);
                    if error > 0.0 {
                        t0 = guess;
                    } else {
                        t1 = guess;
                    }
                }

                calc_bezier(guess, ay, by, cy)
            }
        }
    }
}

fn calc_bezier(t: f32, a: f32, b: f32, c: f32) -> f32 {
    ((a * t + b) * t + c) * t
}

fn calc_derivative(t: f32, a: f32, b: f32, c: f32) -> f32 {
    (3. * a * t + 2. * b) * t + c
}

#[allow(non_upper_case_globals)]
fn ease_out_bounce(mut x: f32) -> f32 {
    const n1: f32 = 7.5625;
    const d1: f32 = 2.75;
    if x < 1. / d1 {
        n1 * x * x
    } else if x < 2. / d1 {
        x -= 1.5 / d1;
        n1 * x * x + 0.75
    } else if x < 2.5 / d1 {
        x -= 2.25 / d1;
        n1 * x * x + 0.9375
    } else {
        x -= 2.625 / d1;
        n1 * x * x + 0.984375
    }
}

#[cfg(test)]
mod tests {
    use super::TimingFunction;

    #[test]
    fn linear() {
        let timing_func = TimingFunction::Linear;
        assert_eq!(timing_func.value(0.43), 0.43);
    }

    #[test]
    fn ease() {
        let timing_func = TimingFunction::ease();
        assert_eq!(timing_func.value(0.25), 0.40851063);
    }
}
