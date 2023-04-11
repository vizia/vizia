use std::f32::EPSILON;

pub struct CubicCurve {
    ax: f32,
    ay: f32,
    bx: f32,
    by: f32,
    cx: f32,
    cy: f32,
}

#[rustfmt::skip]
impl CubicCurve {
    pub fn linear() -> Self { Self::new(0., 0., 1., 1.) }
    pub fn ease() -> Self { Self::new(0.25, 0.1, 0.25, 1.) }
    pub fn ease_in() -> Self { Self::new(0.42, 0., 1., 1.) }
    pub fn ease_out() -> Self { Self::new(0., 0., 0.58, 1.) }
    pub fn ease_in_out() -> Self { Self::new(0.42, 0., 0.58, 1.) }
}

impl CubicCurve {
    pub fn new(c1x: f32, c1y: f32, c2x: f32, c2y: f32) -> Self {
        let cx = 3.0 * c1x;
        let bx = 3.0 * (c2x - c1x) - cx;
        let ax = 1.0 - cx - bx;

        let cy = 3.0 * c1y;
        let by = 3.0 * (c2y - c1y) - cy;
        let ay = 1.0 - cy - by;

        Self {
            ax,
            ay,
            bx,
            by,
            cx,
            cy,
        }
    }

    pub fn curve_y(&self, x: f32) -> f32 {
        let mut t = x;
        let mut error = x - self.cubic_x(t);
        let mut iteration = 0;
        while error.abs() > EPSILON {
            if iteration >= 8 {
                return self.bisect(t, x);
            }

            let d = self.cubic_derivative_x(t);
            if d.abs() < EPSILON {
                break;
            }

            t -= error / d;
            error = x - self.cubic_x(t);
            iteration += 1;
        }
        self.cubic_y(t)
    }

    fn bisect(&self, mut t: f32, x: f32) -> f32 {
        let mut t0 = 0.;
        let mut t1 = 1.;
        let mut error = x - self.cubic_x(t);

        while error.abs() > EPSILON {
            t = (t0 + t1) / 2.0;
            error = x - self.cubic_x(t);
            if error > 0.0 {
                t0 = t;
            } else {
                t1 = t;
            }
        }

        self.cubic_y(t)
    }

    fn cubic_x(&self, t: f32) -> f32 {
        ((self.ax * t + self.bx) * t + self.cx) * t
    }

    fn cubic_y(&self, t: f32) -> f32 {
        ((self.ay * t + self.by) * t + self.cy) * t
    }

    fn cubic_derivative_x(&self, t: f32) -> f32 {
        (3. * self.ax * t + 2. * self.bx) * t + self.cx
    }
}
