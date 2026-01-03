//! Error types for LUT operations.

use core::fmt;

/// Error type for LUT creation failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LutError {
    /// The X axis is empty.
    EmptyXAxis,
    /// The Y axis is empty.
    EmptyYAxis,
    /// The Z axis is empty.
    EmptyZAxis,
    /// Axis lengths don't match data length.
    DimensionMismatch {
        /// Expected data length based on axis sizes.
        expected: usize,
        /// Actual data length provided.
        actual: usize,
    },
    /// Axis values are not strictly ascending.
    UnsortedAxis {
        /// Name of the problematic axis.
        axis: &'static str,
        /// Index where the violation was found.
        index: usize,
    },
}

impl fmt::Display for LutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyXAxis => write!(f, "X axis cannot be empty"),
            Self::EmptyYAxis => write!(f, "Y axis cannot be empty"),
            Self::EmptyZAxis => write!(f, "Z axis cannot be empty"),
            Self::DimensionMismatch { expected, actual } => {
                write!(f, "Data length mismatch: expected {expected}, got {actual}")
            }
            Self::UnsortedAxis { axis, index } => {
                write!(f, "{axis} axis is not strictly ascending at index {index}")
            }
        }
    }
}
