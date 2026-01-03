//! # Vehicle Dynamics Math
//!
//! Mathematical primitives and high-performance lookup tables for the Vehicle Dynamics Engine.
//!
//! This crate provides:
//! - Linear algebra wrappers around `nalgebra` types
//! - 1D, 2D, and 3D lookup tables with efficient interpolation
//!
//! ## Features
//!
//! - `std` - Enable standard library support (disabled by default for `no_std`)
//! - `serde` - Enable serialization/deserialization support
//!
//! ## Example
//!
//! ```
//! use vd_math::linear::Vec3;
//! use vd_math::lut::Lut1D;
//!
//! // Linear algebra
//! let v = Vec3::new(1.0, 2.0, 3.0);
//! let magnitude = v.magnitude();
//!
//! // Lookup table for torque curve
//! let rpm = vec![0.0, 1000.0, 2000.0, 3000.0];
//! let torque = vec![0.0, 150.0, 280.0, 250.0];
//! let lut = Lut1D::new(rpm, torque).unwrap();
//! let torque_at_1500 = lut.lookup(1500.0); // Interpolated value
//! ```

#![no_std]
#![warn(missing_docs)]
#![deny(unsafe_code)]

extern crate alloc;

pub mod linear;
pub mod lut;

pub use linear::{Mat3, Quat, Vec3};
pub use lut::{Lut1D, Lut2D, Lut3D, LutError};
