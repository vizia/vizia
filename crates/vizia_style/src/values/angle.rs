use crate::{impl_parse, Calc, Parse, TryAdd};

/// A value representing an angle expressed in degrees, gradians, radians, or turns.
#[derive(Debug, Clone)]
pub enum Angle {
    /// An angle expressed in degrees.
    Deg(f32),
    /// An angle expressed in gradians.
    Grad(f32),
    /// An angle expressed in radians.
    Rad(f32),
    /// An angle expressed in turns.
    Turn(f32),
}

impl_parse! {
    Angle,

    tokens {
        dimension {
            "deg" => Angle::Deg,
            "grad" => Angle::Grad,
            "turn" => Angle::Turn,
            "rad" => Angle::Rad,
        }
    }
}

impl Angle {
    pub fn is_zero(&self) -> bool {
        use Angle::*;
        match self {
            Deg(v) | Rad(v) | Grad(v) | Turn(v) => *v == 0.0,
        }
    }

    pub fn to_radians(&self) -> f32 {
        const RAD_PER_DEG: f32 = std::f32::consts::PI / 180.0;
        match self {
            Angle::Deg(deg) => deg * RAD_PER_DEG,
            Angle::Rad(rad) => *rad,
            Angle::Grad(grad) => grad * 180.0 / 200.0 * RAD_PER_DEG,
            Angle::Turn(turn) => turn * 360.0 * RAD_PER_DEG,
        }
    }

    pub fn to_degrees(&self) -> f32 {
        const DEG_PER_RAD: f32 = 180.0 / std::f32::consts::PI;
        match self {
            Angle::Deg(deg) => *deg,
            Angle::Rad(rad) => rad * DEG_PER_RAD,
            Angle::Grad(grad) => grad * 180.0 / 200.0,
            Angle::Turn(turn) => turn * 360.0,
        }
    }
}

impl From<Calc<Angle>> for Angle {
    fn from(calc: Calc<Angle>) -> Self {
        match calc {
            Calc::Value(v) => *v,
            _ => unreachable!(),
        }
    }
}

impl From<Angle> for Calc<Angle> {
    fn from(angle: Angle) -> Self {
        Calc::Value(Box::new(angle))
    }
}

impl std::ops::Mul<f32> for Angle {
    type Output = Self;

    fn mul(self, other: f32) -> Angle {
        match self {
            Angle::Deg(v) => Angle::Deg(v * other),
            Angle::Rad(v) => Angle::Deg(v * other),
            Angle::Grad(v) => Angle::Deg(v * other),
            Angle::Turn(v) => Angle::Deg(v * other),
        }
    }
}

impl std::ops::Add<Angle> for Angle {
    type Output = Self;

    fn add(self, other: Angle) -> Angle {
        Angle::Deg(self.to_degrees() + other.to_degrees())
    }
}

impl TryAdd<Angle> for Angle {
    fn try_add(&self, other: &Angle) -> Option<Angle> {
        Some(Angle::Deg(self.to_degrees() + other.to_degrees()))
    }
}

impl std::cmp::PartialEq<f32> for Angle {
    fn eq(&self, other: &f32) -> bool {
        match self {
            Angle::Deg(a) | Angle::Rad(a) | Angle::Grad(a) | Angle::Turn(a) => a == other,
        }
    }
}

impl std::cmp::PartialEq<Angle> for Angle {
    fn eq(&self, other: &Angle) -> bool {
        self.to_degrees() == other.to_degrees()
    }
}

impl std::cmp::PartialOrd<f32> for Angle {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        match self {
            Angle::Deg(a) | Angle::Rad(a) | Angle::Grad(a) | Angle::Turn(a) => a.partial_cmp(other),
        }
    }
}

impl std::cmp::PartialOrd<Angle> for Angle {
    fn partial_cmp(&self, other: &Angle) -> Option<std::cmp::Ordering> {
        self.to_degrees().partial_cmp(&other.to_degrees())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        Angle, assert_angle,

        dimension {
            "deg" => Angle::Deg,
            "grad" => Angle::Grad,
            "turn" => Angle::Turn,
            "rad" => Angle::Rad,
        }
    }
}
