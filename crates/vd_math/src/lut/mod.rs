//! High-performance lookup tables with interpolation.
//!
//! This module provides 1D, 2D, and 3D lookup tables (LUTs) for storing
//! and interpolating tabulated data commonly used in vehicle dynamics
//! (torque curves, tire grip maps, aerodynamic coefficients, etc.).
//!
//! All lookup operations use O(log N) binary search and are designed
//! for real-time performance with no heap allocations during lookup.

mod error;
mod interp;
mod lut1d;
mod lut2d;
mod lut3d;

pub use error::LutError;
pub use lut1d::Lut1D;
pub use lut2d::Lut2D;
pub use lut3d::Lut3D;

use interp::{find_interval, lerp, validate_axis};
