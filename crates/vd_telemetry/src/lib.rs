//! # Vehicle Dynamics Telemetry
//!
//! Zero-cost telemetry system for the Vehicle Dynamics Engine.
//!
//! This crate provides a trait-oriented telemetry system that can be compiled
//! out completely when the `enable_telemetry` feature is disabled, resulting
//! in zero runtime overhead.
//!
//! ## Features
//!
//! - `std` - Enable standard library support
//! - `alloc` - Enable allocation support (required for `enable_telemetry`)
//! - `serde` - Enable serialization/deserialization support
//! - `enable_telemetry` - Enable actual telemetry recording (zero-cost when disabled)
//!
//! ## Example
//!
//! ```
//! use vd_telemetry::{TelemetryProvider, NoOpTelemetry};
//!
//! fn simulate<T: TelemetryProvider>(telemetry: &mut T, dt: f64) {
//!     let speed_id = telemetry.register_channel("vehicle.speed", "m/s");
//!     
//!     // Simulation loop
//!     let speed = 10.0; // computed value
//!     telemetry.log(speed_id, speed);
//! }
//!
//! // With telemetry disabled (zero-cost)
//! let mut noop = NoOpTelemetry;
//! simulate(&mut noop, 0.001);
//! ```
//!
//! ## Zero-Cost Abstraction
//!
//! When `enable_telemetry` is disabled:
//! - `NoOpTelemetry` is a zero-sized type (ZST)
//! - All `log` calls compile to nothing
//! - `ChannelId` becomes a unit struct
//!
//! This allows telemetry code to remain in place without any runtime cost
//! in release builds where telemetry is not needed.

#![no_std]
#![warn(missing_docs)]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod channel;
pub mod recorder;

pub use channel::{ChannelId, ChannelValue};

#[cfg(feature = "enable_telemetry")]
pub use recorder::{MemoryRecorder, RingBufferConfig};

use vd_math::Vec3;

/// Trait for telemetry providers.
///
/// This trait defines the interface for recording telemetry data during
/// simulation. Implementations can range from no-op (for zero-cost disabled
/// telemetry) to full memory recorders with ring buffers.
///
/// # Performance
///
/// The `log` method is designed to be called thousands of times per second
/// in the simulation hot loop. Implementations should:
/// - Avoid heap allocations in `log`
/// - Use simple array indexing
/// - Be inlineable for optimization
pub trait TelemetryProvider {
    /// Registers a new telemetry channel.
    ///
    /// Returns a `ChannelId` that can be used for fast logging.
    /// The ID should be stored and reused - do not call this in the hot loop.
    ///
    /// # Arguments
    ///
    /// * `name` - Human-readable name (e.g., `vehicle.speed`, `tire.fl.slip_ratio`)
    /// * `unit` - Physical unit (e.g., "m/s", "rad", "N")
    fn register_channel(&mut self, name: &str, unit: &str) -> ChannelId;

    /// Logs a scalar value to a channel.
    ///
    /// This method is optimized for hot-path usage. When telemetry is disabled,
    /// this compiles to nothing.
    fn log(&mut self, id: ChannelId, value: f64);

    /// Logs a 3D vector to three channels (x, y, z components).
    ///
    /// Requires pre-registered channel IDs for each component.
    fn log_vector(&mut self, id_x: ChannelId, id_y: ChannelId, id_z: ChannelId, vec: &Vec3);

    /// Logs a boolean value to a channel (stored as 0.0 or 1.0).
    #[inline]
    fn log_bool(&mut self, id: ChannelId, value: bool) {
        self.log(id, if value { 1.0 } else { 0.0 });
    }
}

/// No-op telemetry provider for zero-cost disabled telemetry.
///
/// When telemetry is disabled, this type is a zero-sized type (ZST) and
/// all method calls compile to nothing, providing true zero-cost abstraction.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoOpTelemetry;

impl TelemetryProvider for NoOpTelemetry {
    #[inline]
    fn register_channel(&mut self, _name: &str, _unit: &str) -> ChannelId {
        ChannelId::new(0)
    }

    #[inline]
    fn log(&mut self, _id: ChannelId, _value: f64) {
        // Intentionally empty - compiles to nothing
    }

    #[inline]
    fn log_vector(&mut self, _id_x: ChannelId, _id_y: ChannelId, _id_z: ChannelId, _vec: &Vec3) {
        // Intentionally empty - compiles to nothing
    }
}

/// Helper struct for registering vector channels (x, y, z components).
#[derive(Debug, Clone, Copy)]
pub struct VectorChannelIds {
    /// Channel ID for X component.
    pub x: ChannelId,
    /// Channel ID for Y component.
    pub y: ChannelId,
    /// Channel ID for Z component.
    pub z: ChannelId,
}

impl VectorChannelIds {
    /// Registers three channels for a vector (`{base_name}.x`, `.y`, `.z`).
    #[must_use]
    pub fn register<T: TelemetryProvider>(telemetry: &mut T, base_name: &str, unit: &str) -> Self {
        #[cfg(feature = "alloc")]
        {
            use alloc::format;
            Self {
                x: telemetry.register_channel(&format!("{base_name}.x"), unit),
                y: telemetry.register_channel(&format!("{base_name}.y"), unit),
                z: telemetry.register_channel(&format!("{base_name}.z"), unit),
            }
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = (base_name, unit);
            Self {
                x: telemetry.register_channel("", ""),
                y: telemetry.register_channel("", ""),
                z: telemetry.register_channel("", ""),
            }
        }
    }

    /// Logs a vector to the registered channels.
    #[inline]
    pub fn log<T: TelemetryProvider>(&self, telemetry: &mut T, vec: &Vec3) {
        telemetry.log_vector(self.x, self.y, self.z, vec);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_telemetry_is_zst() {
        assert_eq!(core::mem::size_of::<NoOpTelemetry>(), 0);
    }

    #[test]
    fn test_noop_telemetry_works() {
        let mut telemetry = NoOpTelemetry;
        let id = telemetry.register_channel("test", "unit");
        telemetry.log(id, 42.0);
        telemetry.log_bool(id, true);
        // Should compile and run without issues
    }

    #[test]
    fn test_channel_id_size() {
        assert_eq!(core::mem::size_of::<ChannelId>(), 4);
    }

    #[test]
    fn test_vector_channel_ids_with_noop() {
        let mut telemetry = NoOpTelemetry;
        let ids = VectorChannelIds::register(&mut telemetry, "position", "m");
        let vec = Vec3::new(1.0, 2.0, 3.0);
        ids.log(&mut telemetry, &vec);
        // Should compile and run without issues
    }
}
