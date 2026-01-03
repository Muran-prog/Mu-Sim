//! Derived SI unit types (force, pressure, energy, power).

define_unit!(
    /// Force in Newtons (kg*m/s^2).
    Newtons, "N"
);

define_unit!(
    /// Pressure in Pascals (N/m^2).
    Pascals, "Pa"
);

impl Pascals {
    /// Converts bar to Pascals.
    #[inline]
    #[must_use]
    pub fn from_bar(bar: f64) -> Self {
        Self(bar * 100_000.0)
    }

    /// Converts Pascals to bar.
    #[inline]
    #[must_use]
    pub fn as_bar(self) -> f64 {
        self.0 / 100_000.0
    }

    /// Converts kPa to Pascals.
    #[inline]
    #[must_use]
    pub fn from_kpa(kpa: f64) -> Self {
        Self(kpa * 1_000.0)
    }

    /// Converts Pascals to kPa.
    #[inline]
    #[must_use]
    pub fn as_kpa(self) -> f64 {
        self.0 / 1_000.0
    }

    /// Converts PSI to Pascals.
    #[inline]
    #[must_use]
    pub fn from_psi(psi: f64) -> Self {
        Self(psi * 6_894.757)
    }

    /// Converts Pascals to PSI.
    #[inline]
    #[must_use]
    pub fn as_psi(self) -> f64 {
        self.0 / 6_894.757
    }

    /// Standard atmospheric pressure (101325 Pa).
    pub const ATMOSPHERIC: Self = Self(101_325.0);
}

define_unit!(
    /// Torque in Newton-meters (N*m).
    NewtonMeters, "N*m"
);

impl NewtonMeters {
    /// Converts torque to energy (same dimension: N*m = J).
    #[inline]
    #[must_use]
    pub const fn as_joules(self) -> Joules {
        Joules(self.0)
    }

    /// Creates torque from energy (same dimension: J = N*m).
    #[inline]
    #[must_use]
    pub const fn from_joules(joules: Joules) -> Self {
        Self(joules.0)
    }
}

define_unit!(
    /// Energy in Joules (N*m = kg*m^2/s^2).
    Joules, "J"
);

impl Joules {
    /// Converts energy to torque (same dimension: J = N*m).
    #[inline]
    #[must_use]
    pub const fn as_newton_meters(self) -> NewtonMeters {
        NewtonMeters(self.0)
    }

    /// Creates energy from torque (same dimension: N*m = J).
    #[inline]
    #[must_use]
    pub const fn from_newton_meters(nm: NewtonMeters) -> Self {
        Self(nm.0)
    }
}

define_unit!(
    /// Power in Watts (J/s = kg*m^2/s^3).
    Watts, "W"
);
