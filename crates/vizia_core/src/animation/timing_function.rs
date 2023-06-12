#[derive(Debug, Clone, Copy)]
pub(crate) struct TimingFunction {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
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
        Self { x1, y1, x2, y2 }
    }

    pub fn value(&self, x: f32) -> f32 {
        // Linear
        if self.x1 == self.y1 && self.x2 == self.y2 {
            return x;
        }

        Self::calc_bezier(self.find_t_for_x(x), self.y1, self.y2)
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

    fn find_t_for_x(&self, x: f32) -> f32 {
        let mut guess = x;
        let mut error = f32::MAX;
        for _ in 0..8 {
            let pos = Self::calc_bezier(guess, self.x1, self.x2);
            error = pos - x;
            if error.abs() <= 0.0000001 {
                return guess;
            }
            let slope = Self::calc_bezier_slope(guess, self.x1, self.x2);
            guess -= error / slope;
        }
        if error.abs() <= 0.0000001 {
            guess
        } else {
            x
        }
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
        assert_eq!(timing_func.value(0.25), 0.4085106);
    }
}
