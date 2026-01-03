//! Base SI unit types.

define_unit!(
    /// Time duration in seconds (SI base unit).
    Seconds, "s"
);

define_unit!(
    /// Length/distance in meters (SI base unit).
    Meters, "m"
);

define_unit!(
    /// Mass in kilograms (SI base unit).
    Kilograms, "kg"
);

define_unit!(
    /// Temperature in Kelvin (SI base unit).
    Kelvin, "K"
);

impl Kelvin {
    /// Converts Celsius to Kelvin.
    #[inline]
    #[must_use]
    pub fn from_celsius(celsius: f64) -> Self {
        Self(celsius + 273.15)
    }

    /// Converts Kelvin to Celsius.
    #[inline]
    #[must_use]
    pub fn as_celsius(self) -> f64 {
        self.0 - 273.15
    }

    /// Converts Fahrenheit to Kelvin.
    #[inline]
    #[must_use]
    pub fn from_fahrenheit(fahrenheit: f64) -> Self {
        Self((fahrenheit - 32.0) * 5.0 / 9.0 + 273.15)
    }

    /// Converts Kelvin to Fahrenheit.
    #[inline]
    #[must_use]
    pub fn as_fahrenheit(self) -> f64 {
        (self.0 - 273.15) * 9.0 / 5.0 + 32.0
    }

    /// Absolute zero (0 K).
    pub const ABSOLUTE_ZERO: Self = Self(0.0);

    /// Standard temperature (288.15 K = 15 C).
    pub const STANDARD: Self = Self(288.15);
}
