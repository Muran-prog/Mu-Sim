//! # Vehicle Dynamics Types
//!
//! Fundamental physical types and SI units for the Vehicle Dynamics Engine.
//!
//! This crate provides type-safe wrappers around physical quantities to prevent
//! unit confusion errors at compile time. All types use the newtype pattern
//! with `#[repr(transparent)]` for zero-cost abstractions.
//!
//! ## Features
//!
//! - `std` - Enable standard library support (disabled by default for `no_std`)
//! - `serde` - Enable serialization/deserialization support
//!
//! ## Example
//!
//! ```
//! use vd_types::units::{Meters, Seconds, MetersPerSecond};
//!
//! let distance = Meters(100.0);
//! let time = Seconds(10.0);
//! let velocity = distance / time;
//! assert!((velocity.0 - 10.0).abs() < 1e-10);
//! ```

#![no_std]
#![warn(missing_docs)]
#![deny(unsafe_code)]

pub mod constants;
pub mod units;

pub use constants::*;
pub use units::*;
