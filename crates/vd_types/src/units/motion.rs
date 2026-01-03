//! Motion unit types (velocity, acceleration).

define_unit!(
    /// Linear velocity in meters per second.
    MetersPerSecond, "m/s"
);

impl MetersPerSecond {
    /// Converts km/h to m/s.
    #[inline]
    #[must_use]
    pub fn from_kmh(kmh: f64) -> Self {
        Self(kmh / 3.6)
    }

    /// Converts m/s to km/h.
    #[inline]
    #[must_use]
    pub fn as_kmh(self) -> f64 {
        self.0 * 3.6
    }

    /// Converts mph to m/s.
    #[inline]
    #[must_use]
    pub fn from_mph(mph: f64) -> Self {
        Self(mph * 0.447_04)
    }

    /// Converts m/s to mph.
    #[inline]
    #[must_use]
    pub fn as_mph(self) -> f64 {
        self.0 / 0.447_04
    }
}

define_unit!(
    /// Linear acceleration in meters per second squared.
    MetersPerSecondSquared, "m/s^2"
);

impl MetersPerSecondSquared {
    /// Converts g-force to m/s^2.
    #[inline]
    #[must_use]
    pub fn from_g(g: f64) -> Self {
        Self(g * 9.806_65)
    }

    /// Converts m/s^2 to g-force.
    #[inline]
    #[must_use]
    pub fn as_g(self) -> f64 {
        self.0 / 9.806_65
    }
}
