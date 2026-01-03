//! Dimensional analysis - cross-type arithmetic operations.

use core::ops::{Div, Mul};

use super::{
    Joules, Kilograms, Meters, MetersPerSecond, MetersPerSecondSquared, NewtonMeters, Newtons,
    Radians, RadiansPerSecond, RadiansPerSecondSquared, Seconds, Watts,
};

// =============================================================================
// Linear Motion
// =============================================================================

// Distance / Time = Velocity
impl Div<Seconds> for Meters {
    type Output = MetersPerSecond;
    #[inline]
    fn div(self, rhs: Seconds) -> Self::Output {
        MetersPerSecond(self.0 / rhs.0)
    }
}

// Velocity * Time = Distance
impl Mul<Seconds> for MetersPerSecond {
    type Output = Meters;
    #[inline]
    fn mul(self, rhs: Seconds) -> Self::Output {
        Meters(self.0 * rhs.0)
    }
}

// Time * Velocity = Distance
impl Mul<MetersPerSecond> for Seconds {
    type Output = Meters;
    #[inline]
    fn mul(self, rhs: MetersPerSecond) -> Self::Output {
        Meters(self.0 * rhs.0)
    }
}

// Velocity / Time = Acceleration
impl Div<Seconds> for MetersPerSecond {
    type Output = MetersPerSecondSquared;
    #[inline]
    fn div(self, rhs: Seconds) -> Self::Output {
        MetersPerSecondSquared(self.0 / rhs.0)
    }
}

// Acceleration * Time = Velocity
impl Mul<Seconds> for MetersPerSecondSquared {
    type Output = MetersPerSecond;
    #[inline]
    fn mul(self, rhs: Seconds) -> Self::Output {
        MetersPerSecond(self.0 * rhs.0)
    }
}

// =============================================================================
// Force and Mass
// =============================================================================

// Force = Mass * Acceleration (F = ma)
impl Mul<MetersPerSecondSquared> for Kilograms {
    type Output = Newtons;
    #[inline]
    fn mul(self, rhs: MetersPerSecondSquared) -> Self::Output {
        Newtons(self.0 * rhs.0)
    }
}

impl Mul<Kilograms> for MetersPerSecondSquared {
    type Output = Newtons;
    #[inline]
    fn mul(self, rhs: Kilograms) -> Self::Output {
        Newtons(self.0 * rhs.0)
    }
}

// Acceleration = Force / Mass (a = F/m)
impl Div<Kilograms> for Newtons {
    type Output = MetersPerSecondSquared;
    #[inline]
    fn div(self, rhs: Kilograms) -> Self::Output {
        MetersPerSecondSquared(self.0 / rhs.0)
    }
}

// =============================================================================
// Torque and Energy
// =============================================================================

// Torque = Force * Distance (also represents Work/Energy, since N*m = J)
impl Mul<Meters> for Newtons {
    type Output = NewtonMeters;
    #[inline]
    fn mul(self, rhs: Meters) -> Self::Output {
        NewtonMeters(self.0 * rhs.0)
    }
}

impl Mul<Newtons> for Meters {
    type Output = NewtonMeters;
    #[inline]
    fn mul(self, rhs: Newtons) -> Self::Output {
        NewtonMeters(self.0 * rhs.0)
    }
}

// =============================================================================
// Power
// =============================================================================

// Power = Energy / Time
impl Div<Seconds> for Joules {
    type Output = Watts;
    #[inline]
    fn div(self, rhs: Seconds) -> Self::Output {
        Watts(self.0 / rhs.0)
    }
}

// Power = Torque * Angular Velocity
impl Mul<RadiansPerSecond> for NewtonMeters {
    type Output = Watts;
    #[inline]
    fn mul(self, rhs: RadiansPerSecond) -> Self::Output {
        Watts(self.0 * rhs.0)
    }
}

impl Mul<NewtonMeters> for RadiansPerSecond {
    type Output = Watts;
    #[inline]
    fn mul(self, rhs: NewtonMeters) -> Self::Output {
        Watts(self.0 * rhs.0)
    }
}

// =============================================================================
// Angular Motion
// =============================================================================

// Angular velocity / Time = Angular acceleration
impl Div<Seconds> for RadiansPerSecond {
    type Output = RadiansPerSecondSquared;
    #[inline]
    fn div(self, rhs: Seconds) -> Self::Output {
        RadiansPerSecondSquared(self.0 / rhs.0)
    }
}

// Angular acceleration * Time = Angular velocity
impl Mul<Seconds> for RadiansPerSecondSquared {
    type Output = RadiansPerSecond;
    #[inline]
    fn mul(self, rhs: Seconds) -> Self::Output {
        RadiansPerSecond(self.0 * rhs.0)
    }
}

// Angle / Time = Angular velocity
impl Div<Seconds> for Radians {
    type Output = RadiansPerSecond;
    #[inline]
    fn div(self, rhs: Seconds) -> Self::Output {
        RadiansPerSecond(self.0 / rhs.0)
    }
}

// Angular velocity * Time = Angle
impl Mul<Seconds> for RadiansPerSecond {
    type Output = Radians;
    #[inline]
    fn mul(self, rhs: Seconds) -> Self::Output {
        Radians(self.0 * rhs.0)
    }
}
