use std::f32::EPSILON;

#[derive(Debug, Clone, Copy)]
pub enum TimingFunction {
    Linear,
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

    pub fn value(&self, x: f32) -> f32 {
        match *self {
            Self::Linear => x,
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
