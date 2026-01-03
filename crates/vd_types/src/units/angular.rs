//! Angular unit types (radians, angular velocity, RPM).

define_unit!(
    /// Angle in radians.
    Radians, "rad"
);

impl Radians {
    /// Converts degrees to radians.
    #[inline]
    #[must_use]
    #[allow(clippy::suboptimal_flops)] // `to_radians()` requires std
    pub fn from_degrees(degrees: f64) -> Self {
        Self(degrees * core::f64::consts::PI / 180.0)
    }

    /// Converts radians to degrees.
    #[inline]
    #[must_use]
    #[allow(clippy::suboptimal_flops)] // `to_degrees()` requires std
    pub fn as_degrees(self) -> f64 {
        self.0 * 180.0 / core::f64::consts::PI
    }

    /// Returns the sine of the angle.
    #[inline]
    #[must_use]
    pub fn sin(self) -> f64 {
        libm::sin(self.0)
    }

    /// Returns the cosine of the angle.
    #[inline]
    #[must_use]
    pub fn cos(self) -> f64 {
        libm::cos(self.0)
    }

    /// Returns the tangent of the angle.
    #[inline]
    #[must_use]
    pub fn tan(self) -> f64 {
        libm::tan(self.0)
    }

    /// Normalizes the angle to the range [0, 2*PI).
    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        let two_pi = 2.0 * core::f64::consts::PI;
        let mut result = libm::fmod(self.0, two_pi);
        if result < 0.0 {
            result += two_pi;
        }
        Self(result)
    }

    /// Full rotation (2*PI radians).
    pub const FULL_ROTATION: Self = Self(2.0 * core::f64::consts::PI);

    /// Half rotation (PI radians).
    pub const HALF_ROTATION: Self = Self(core::f64::consts::PI);

    /// Quarter rotation (PI/2 radians).
    pub const QUARTER_ROTATION: Self = Self(core::f64::consts::FRAC_PI_2);
}

define_unit!(
    /// Angular velocity in radians per second.
    RadiansPerSecond, "rad/s"
);

define_unit!(
    /// Rotational speed in revolutions per minute.
    RPM, "rpm"
);

impl RPM {
    /// Converts RPM to radians per second.
    #[inline]
    #[must_use]
    pub fn to_rad_per_sec(self) -> RadiansPerSecond {
        RadiansPerSecond(self.0 * core::f64::consts::PI / 30.0)
    }

    /// Creates RPM from radians per second.
    #[inline]
    #[must_use]
    pub fn from_rad_per_sec(rps: RadiansPerSecond) -> Self {
        Self(rps.0 * 30.0 / core::f64::consts::PI)
    }
}

define_unit!(
    /// Angular acceleration in radians per second squared.
    RadiansPerSecondSquared, "rad/s^2"
);
