//! Telemetry recorder implementations.
//!
//! This module provides concrete implementations of the `TelemetryProvider` trait,
//! including a ring buffer-based memory recorder for storing telemetry history.

#[cfg(feature = "enable_telemetry")]
mod enabled {
    use crate::channel::{ChannelId, ChannelMetadata};
    use crate::TelemetryProvider;
    use alloc::vec;
    use alloc::vec::Vec;
    use vd_math::Vec3;

    extern crate alloc;

    /// Ring buffer configuration.
    #[derive(Debug, Clone, Copy)]
    pub struct RingBufferConfig {
        /// Number of samples to store per channel.
        pub samples_per_channel: usize,
        /// Maximum number of channels.
        pub max_channels: usize,
    }

    impl Default for RingBufferConfig {
        fn default() -> Self {
            Self {
                samples_per_channel: 10_000,
                max_channels: 256,
            }
        }
    }

    impl RingBufferConfig {
        /// Creates config for a given duration and sample rate.
        #[must_use]
        pub fn for_duration(duration_secs: f64, sample_rate_hz: f64, max_channels: usize) -> Self {
            let samples = libm::ceil(duration_secs * sample_rate_hz) as usize;
            Self {
                samples_per_channel: samples,
                max_channels,
            }
        }
    }

    /// In-memory telemetry recorder using ring buffers.
    ///
    /// Pre-allocates storage to avoid allocations during the simulation loop.
    /// When the buffer is full, old data is overwritten (ring buffer behavior).
    pub struct MemoryRecorder {
        /// Channel metadata.
        metadata: Vec<ChannelMetadata>,
        /// Data storage: flat buffer organized as `[ch0_samples, ch1_samples, ...]`.
        data: Vec<f64>,
        /// Write position for each channel (index into the channel's slice).
        write_positions: Vec<usize>,
        /// Number of samples written to each channel (saturates at buffer size).
        sample_counts: Vec<usize>,
        /// Configuration.
        config: RingBufferConfig,
    }

    impl MemoryRecorder {
        /// Creates a new memory recorder with the given configuration.
        #[must_use]
        pub fn new(config: RingBufferConfig) -> Self {
            Self {
                metadata: Vec::with_capacity(config.max_channels),
                data: Vec::new(),
                write_positions: Vec::with_capacity(config.max_channels),
                sample_counts: Vec::with_capacity(config.max_channels),
                config,
            }
        }

        /// Creates a new memory recorder with default configuration.
        #[must_use]
        pub fn with_defaults() -> Self {
            Self::new(RingBufferConfig::default())
        }

        /// Returns the number of registered channels.
        #[must_use]
        pub fn channel_count(&self) -> usize {
            self.metadata.len()
        }

        /// Returns the metadata for a channel.
        #[must_use]
        pub fn channel_metadata(&self, id: ChannelId) -> Option<&ChannelMetadata> {
            self.metadata.get(id.index() as usize)
        }

        /// Returns all channel metadata.
        #[must_use]
        pub fn all_metadata(&self) -> &[ChannelMetadata] {
            &self.metadata
        }

        /// Returns the number of samples stored for a channel.
        #[must_use]
        pub fn sample_count(&self, id: ChannelId) -> usize {
            self.sample_counts
                .get(id.index() as usize)
                .copied()
                .unwrap_or(0)
        }

        /// Returns the data for a channel as a slice.
        ///
        /// The data is returned in chronological order (oldest first).
        #[must_use]
        pub fn get_channel_data(&self, id: ChannelId) -> Option<Vec<f64>> {
            let idx = id.index() as usize;
            if idx >= self.metadata.len() {
                return None;
            }

            let samples = self.config.samples_per_channel;
            let base = idx * samples;
            let count = self.sample_counts[idx];
            let write_pos = self.write_positions[idx];

            if count < samples {
                // Buffer not full yet - data is in order from start
                Some(self.data[base..base + count].to_vec())
            } else {
                // Buffer wrapped - need to reorder
                let mut result = Vec::with_capacity(samples);
                // Oldest data starts at write_pos
                result.extend_from_slice(&self.data[base + write_pos..base + samples]);
                result.extend_from_slice(&self.data[base..base + write_pos]);
                Some(result)
            }
        }

        /// Clears all recorded data but keeps channel registrations.
        pub fn clear(&mut self) {
            for pos in &mut self.write_positions {
                *pos = 0;
            }
            for count in &mut self.sample_counts {
                *count = 0;
            }
            // Reset data to zeros
            for val in &mut self.data {
                *val = 0.0;
            }
        }
    }

    impl TelemetryProvider for MemoryRecorder {
        fn register_channel(&mut self, name: &str, unit: &str) -> ChannelId {
            let id = ChannelId::new(self.metadata.len() as u32);

            if self.metadata.len() >= self.config.max_channels {
                // Return a dummy ID that will be ignored on log
                return ChannelId::new(u32::MAX);
            }

            self.metadata.push(ChannelMetadata::new(name, unit));
            self.write_positions.push(0);
            self.sample_counts.push(0);

            // Extend the data buffer for this channel
            self.data.extend(vec![0.0; self.config.samples_per_channel]);

            id
        }

        #[inline]
        fn log(&mut self, id: ChannelId, value: f64) {
            let idx = id.index() as usize;
            if idx >= self.metadata.len() {
                return;
            }

            let samples = self.config.samples_per_channel;
            let base = idx * samples;
            let write_pos = self.write_positions[idx];

            // Direct write - no bounds check needed due to modular arithmetic
            self.data[base + write_pos] = value;

            // Advance write position (ring buffer wrap)
            self.write_positions[idx] = (write_pos + 1) % samples;

            // Update sample count (saturates at buffer size)
            if self.sample_counts[idx] < samples {
                self.sample_counts[idx] += 1;
            }
        }

        fn log_vector(&mut self, id_x: ChannelId, id_y: ChannelId, id_z: ChannelId, vec: &Vec3) {
            self.log(id_x, vec.x);
            self.log(id_y, vec.y);
            self.log(id_z, vec.z);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_register_channel() {
            let mut recorder = MemoryRecorder::with_defaults();
            let id = recorder.register_channel("speed", "m/s");
            assert_eq!(id.index(), 0);
            assert_eq!(recorder.channel_count(), 1);

            let meta = recorder
                .channel_metadata(id)
                .expect("metadata should exist");
            assert_eq!(meta.name, "speed");
            assert_eq!(meta.unit, "m/s");
        }

        #[test]
        fn test_log_and_retrieve() {
            let config = RingBufferConfig {
                samples_per_channel: 10,
                max_channels: 4,
            };
            let mut recorder = MemoryRecorder::new(config);
            let id = recorder.register_channel("test", "unit");

            for i in 0..5 {
                recorder.log(id, i as f64);
            }

            let data = recorder.get_channel_data(id).expect("data should exist");
            assert_eq!(data.len(), 5);
            for (i, &v) in data.iter().enumerate() {
                assert!((v - i as f64).abs() < 1e-10);
            }
        }

        #[test]
        fn test_ring_buffer_overwrite() {
            let config = RingBufferConfig {
                samples_per_channel: 5,
                max_channels: 4,
            };
            let mut recorder = MemoryRecorder::new(config);
            let id = recorder.register_channel("test", "unit");

            // Write 8 values into a buffer of size 5
            for i in 0..8 {
                recorder.log(id, i as f64);
            }

            // Should have values 3, 4, 5, 6, 7 in chronological order
            let data = recorder.get_channel_data(id).expect("data should exist");
            assert_eq!(data.len(), 5);
            assert!((data[0] - 3.0).abs() < 1e-10);
            assert!((data[1] - 4.0).abs() < 1e-10);
            assert!((data[2] - 5.0).abs() < 1e-10);
            assert!((data[3] - 6.0).abs() < 1e-10);
            assert!((data[4] - 7.0).abs() < 1e-10);
        }

        #[test]
        fn test_sine_wave_integrity() {
            use core::f64::consts::PI;

            let config = RingBufferConfig {
                samples_per_channel: 100,
                max_channels: 4,
            };
            let mut recorder = MemoryRecorder::new(config);
            let id = recorder.register_channel("sine", "");

            // Record a sine wave
            for i in 0..100 {
                let t = i as f64 / 100.0 * 2.0 * PI;
                recorder.log(id, libm::sin(t));
            }

            // Verify data integrity
            let data = recorder.get_channel_data(id).expect("data should exist");
            assert_eq!(data.len(), 100);

            for (i, &v) in data.iter().enumerate() {
                let t = i as f64 / 100.0 * 2.0 * PI;
                let expected = libm::sin(t);
                assert!(
                    (v - expected).abs() < 1e-10,
                    "mismatch at {}: {} != {}",
                    i,
                    v,
                    expected
                );
            }
        }

        #[test]
        fn test_log_vector() {
            let config = RingBufferConfig {
                samples_per_channel: 10,
                max_channels: 10,
            };
            let mut recorder = MemoryRecorder::new(config);
            let id_x = recorder.register_channel("pos.x", "m");
            let id_y = recorder.register_channel("pos.y", "m");
            let id_z = recorder.register_channel("pos.z", "m");

            let vec = Vec3::new(1.0, 2.0, 3.0);
            recorder.log_vector(id_x, id_y, id_z, &vec);

            let data_x = recorder.get_channel_data(id_x).expect("x data");
            let data_y = recorder.get_channel_data(id_y).expect("y data");
            let data_z = recorder.get_channel_data(id_z).expect("z data");

            assert!((data_x[0] - 1.0).abs() < 1e-10);
            assert!((data_y[0] - 2.0).abs() < 1e-10);
            assert!((data_z[0] - 3.0).abs() < 1e-10);
        }

        #[test]
        fn test_clear() {
            let config = RingBufferConfig {
                samples_per_channel: 10,
                max_channels: 4,
            };
            let mut recorder = MemoryRecorder::new(config);
            let id = recorder.register_channel("test", "unit");

            recorder.log(id, 42.0);
            assert_eq!(recorder.sample_count(id), 1);

            recorder.clear();
            assert_eq!(recorder.sample_count(id), 0);
        }
    }
}

#[cfg(feature = "enable_telemetry")]
pub use enabled::*;
