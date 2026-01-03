//! Telemetry channel types and identifiers.
//!
//! This module defines the types used to identify and store telemetry data.

use vd_math::Vec3;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Lightweight handle for fast channel access.
///
/// This is a simple index into the channel storage, designed for
/// zero-overhead lookups in hot paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ChannelId(pub u32);

impl ChannelId {
    /// Creates a new channel ID from a raw index.
    #[inline]
    #[must_use]
    pub const fn new(index: u32) -> Self {
        Self(index)
    }

    /// Returns the raw index value.
    #[inline]
    #[must_use]
    pub const fn index(self) -> u32 {
        self.0
    }
}

/// Value types that can be logged to telemetry channels.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ChannelValue {
    /// Scalar floating-point value.
    Float(f64),
    /// Boolean flag value.
    Bool(bool),
    /// 3D vector value (x, y, z components).
    Vector3 {
        /// X component.
        x: f64,
        /// Y component.
        y: f64,
        /// Z component.
        z: f64,
    },
}

impl ChannelValue {
    /// Creates a float channel value.
    #[inline]
    #[must_use]
    pub const fn float(value: f64) -> Self {
        Self::Float(value)
    }

    /// Creates a boolean channel value.
    #[inline]
    #[must_use]
    pub const fn bool(value: bool) -> Self {
        Self::Bool(value)
    }

    /// Creates a vector3 channel value from components.
    #[inline]
    #[must_use]
    pub const fn vector3(x: f64, y: f64, z: f64) -> Self {
        Self::Vector3 { x, y, z }
    }

    /// Creates a vector3 channel value from a Vec3.
    #[inline]
    #[must_use]
    pub fn from_vec3(v: &Vec3) -> Self {
        Self::Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }

    /// Extracts the float value, returning 0.0 for non-float variants.
    #[inline]
    #[must_use]
    pub const fn as_float(&self) -> f64 {
        match self {
            Self::Float(v) => *v,
            Self::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            Self::Vector3 { x, .. } => *x,
        }
    }
}

impl From<f64> for ChannelValue {
    #[inline]
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for ChannelValue {
    #[inline]
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<Vec3> for ChannelValue {
    #[inline]
    fn from(value: Vec3) -> Self {
        Self::Vector3 {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

/// Metadata for a telemetry channel.
#[cfg(feature = "enable_telemetry")]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChannelMetadata {
    /// Human-readable name of the channel.
    pub name: alloc::string::String,
    /// Physical unit of the channel (e.g., "m/s", "rad", "N").
    pub unit: alloc::string::String,
}

#[cfg(feature = "enable_telemetry")]
extern crate alloc;

#[cfg(feature = "enable_telemetry")]
impl ChannelMetadata {
    /// Creates new channel metadata.
    #[must_use]
    pub fn new(name: &str, unit: &str) -> Self {
        Self {
            name: alloc::string::String::from(name),
            unit: alloc::string::String::from(unit),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_id() {
        let id = ChannelId::new(42);
        assert_eq!(id.index(), 42);
    }

    #[test]
    fn test_channel_value_float() {
        let v = ChannelValue::float(3.14);
        assert!((v.as_float() - 3.14).abs() < 1e-10);
    }

    #[test]
    fn test_channel_value_bool() {
        let v_true = ChannelValue::bool(true);
        let v_false = ChannelValue::bool(false);
        assert!((v_true.as_float() - 1.0).abs() < 1e-10);
        assert!(v_false.as_float().abs() < 1e-10);
    }

    #[test]
    fn test_channel_value_from_f64() {
        let v: ChannelValue = 2.718.into();
        assert!((v.as_float() - 2.718).abs() < 1e-10);
    }
}
