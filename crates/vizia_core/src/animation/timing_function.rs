use std::f32::EPSILON;

#[derive(Debug, Clone, Copy)]
pub struct TimingFunction {
    ax: f32,
    ay: f32,
    bx: f32,
    by: f32,
    cx: f32,
    cy: f32,
}

impl Default for TimingFunction {
    fn default() -> Self {
        Self::linear()
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
}

impl TimingFunction {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        let cx = 3.0 * x1;
        let bx = 3.0 * (x2 - x1) - cx;
        let ax = 1.0 - cx - bx;

        let cy = 3.0 * y1;
        let by = 3.0 * (y2 - y1) - cy;
        let ay = 1.0 - cy - by;

        Self { ax, ay, bx, by, cx, cy }
    }

    pub fn value(&self, x: f32) -> f32 {
        // Newton Raphson
        let mut guess = x;
        let mut error = x - self.calc_bezier_x(guess);
        for _ in 0..8 {
            if error.abs() <= EPSILON {
                return self.calc_bezier_y(guess);
            }

            let pos = self.calc_bezier_x(guess);
            error = pos - x;

            let derivative = self.calc_derivative(guess);
            guess -= error / derivative;
        }

        // Bisect
        let mut t0 = 0.;
        let mut t1 = 1.;

        while error.abs() > EPSILON {
            guess = (t0 + t1) / 2.0;
            error = x - self.calc_bezier_x(guess);
            if error > 0.0 {
                t0 = guess;
            } else {
                t1 = guess;
            }
        }

        self.calc_bezier_y(guess)
    }

    fn calc_bezier_x(&self, t: f32) -> f32 {
        ((self.ax * t + self.bx) * t + self.cx) * t
    }

    fn calc_bezier_y(&self, t: f32) -> f32 {
        ((self.ay * t + self.by) * t + self.cy) * t
    }

    fn calc_derivative(&self, t: f32) -> f32 {
        (3. * self.ax * t + 2. * self.bx) * t + self.cx
    }
}

#[cfg(test)]
mod tests {
    use super::TimingFunction;

    #[test]
    fn linear() {
        let timing_func = TimingFunction::linear();
        assert_eq!(timing_func.value(0.5), 0.5);
    }

    #[test]
    fn ease() {
        let timing_func = TimingFunction::ease();
        assert_eq!(timing_func.value(0.25), 0.40851063);
    }
}
