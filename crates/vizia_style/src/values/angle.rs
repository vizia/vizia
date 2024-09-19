use crate::{impl_parse, Calc, Parse, TryAdd};

/// A value representing an angle expressed in degrees, gradians, radians, or turns.
#[derive(Debug, Clone, Copy)]
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

impl Default for Angle {
    fn default() -> Self {
        Self::Rad(0.0)
    }
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
            Self::Deg(deg) => deg * RAD_PER_DEG,
            Self::Rad(rad) => *rad,
            Self::Grad(grad) => grad * 180.0 / 200.0 * RAD_PER_DEG,
            Self::Turn(turn) => turn * 360.0 * RAD_PER_DEG,
        }
    }

    pub fn to_degrees(&self) -> f32 {
        const DEG_PER_RAD: f32 = 180.0 / std::f32::consts::PI;
        match self {
            Self::Deg(deg) => *deg,
            Self::Rad(rad) => rad * DEG_PER_RAD,
            Self::Grad(grad) => grad * 180.0 / 200.0,
            Self::Turn(turn) => turn * 360.0,
        }
    }
}

impl From<Calc<Self>> for Angle {
    fn from(calc: Calc<Self>) -> Self {
        match calc {
            Calc::Value(v) => *v,
            _ => unreachable!(),
        }
    }
}

impl From<Angle> for Calc<Angle> {
    fn from(angle: Angle) -> Self {
        Self::Value(Box::new(angle))
    }
}

impl std::ops::Mul<f32> for Angle {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        match self {
            Self::Deg(v) => Self::Deg(v * other),
            Self::Rad(v) => Self::Deg(v * other),
            Self::Grad(v) => Self::Deg(v * other),
            Self::Turn(v) => Self::Deg(v * other),
        }
    }
}

impl std::ops::Add<Self> for Angle {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::Deg(self.to_degrees() + other.to_degrees())
    }
}

impl TryAdd<Self> for Angle {
    fn try_add(&self, other: &Self) -> Option<Self> {
        Some(Self::Deg(self.to_degrees() + other.to_degrees()))
    }
}

impl std::cmp::PartialEq<f32> for Angle {
    fn eq(&self, other: &f32) -> bool {
        match self {
            Self::Deg(a) | Self::Rad(a) | Self::Grad(a) | Self::Turn(a) => a == other,
        }
    }
}

impl std::cmp::PartialEq<Self> for Angle {
    fn eq(&self, other: &Self) -> bool {
        self.to_degrees() == other.to_degrees()
    }
}

impl std::cmp::PartialOrd<f32> for Angle {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        match self {
            Self::Deg(a) | Self::Rad(a) | Self::Grad(a) | Self::Turn(a) => a.partial_cmp(other),
        }
    }
}

impl std::cmp::PartialOrd<Self> for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
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
