//! Macro definitions for unit types.

/// Helper macro to define a unit type with common trait implementations.
#[macro_export]
macro_rules! define_unit {
    (
        $(#[$meta:meta])*
        $name:ident, $unit:expr
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
        #[repr(transparent)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name(pub f64);

        impl $name {
            /// Creates a new instance with the given value.
            #[inline]
            #[must_use]
            pub const fn new(value: f64) -> Self {
                Self(value)
            }

            /// Returns the raw value.
            #[inline]
            #[must_use]
            pub const fn value(self) -> f64 {
                self.0
            }

            /// Returns the absolute value.
            #[inline]
            #[must_use]
            pub fn abs(self) -> Self {
                Self(self.0.abs())
            }

            /// Returns the minimum of two values.
            #[inline]
            #[must_use]
            pub fn min(self, other: Self) -> Self {
                Self(self.0.min(other.0))
            }

            /// Returns the maximum of two values.
            #[inline]
            #[must_use]
            pub fn max(self, other: Self) -> Self {
                Self(self.0.max(other.0))
            }

            /// Clamps the value to the given range.
            #[inline]
            #[must_use]
            pub fn clamp(self, min: Self, max: Self) -> Self {
                Self(self.0.clamp(min.0, max.0))
            }

            /// Returns true if the value is finite.
            #[inline]
            #[must_use]
            pub fn is_finite(self) -> bool {
                self.0.is_finite()
            }

            /// Returns true if the value is NaN.
            #[inline]
            #[must_use]
            pub fn is_nan(self) -> bool {
                self.0.is_nan()
            }

            /// Zero value.
            pub const ZERO: Self = Self(0.0);

            /// Unit symbol for display purposes.
            pub const UNIT: &'static str = $unit;
        }

        impl core::ops::Add for $name {
            type Output = Self;
            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl core::ops::Sub for $name {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl core::ops::Neg for $name {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }

        impl core::ops::Mul<f64> for $name {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: f64) -> Self::Output {
                Self(self.0 * rhs)
            }
        }

        impl core::ops::Mul<$name> for f64 {
            type Output = $name;
            #[inline]
            fn mul(self, rhs: $name) -> Self::Output {
                $name(self * rhs.0)
            }
        }

        impl core::ops::Div<f64> for $name {
            type Output = Self;
            #[inline]
            fn div(self, rhs: f64) -> Self::Output {
                Self(self.0 / rhs)
            }
        }

        impl core::ops::Div<$name> for $name {
            type Output = f64;
            #[inline]
            fn div(self, rhs: $name) -> Self::Output {
                self.0 / rhs.0
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{} {}", self.0, Self::UNIT)
            }
        }
    };
}
