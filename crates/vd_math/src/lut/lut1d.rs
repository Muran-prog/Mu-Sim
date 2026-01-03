//! 1D lookup table implementation.

use alloc::vec::Vec;

use super::{find_interval, lerp, validate_axis, LutError};

/// 1D lookup table for y = f(x) interpolation.
///
/// # Example
///
/// ```
/// use vd_math::lut::Lut1D;
///
/// // Engine torque curve: RPM -> Nm
/// let rpm = vec![0.0, 1000.0, 2000.0, 3000.0, 4000.0];
/// let torque = vec![0.0, 100.0, 200.0, 180.0, 150.0];
/// let lut = Lut1D::new(rpm, torque).unwrap();
///
/// // Interpolate torque at 1500 RPM
/// let t = lut.lookup(1500.0);
/// assert!((t - 150.0).abs() < 1e-10); // Linear interpolation
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Lut1D {
    x_axis: Vec<f64>,
    data: Vec<f64>,
}

impl Lut1D {
    /// Creates a new 1D lookup table.
    ///
    /// # Arguments
    ///
    /// * `x_axis` - The independent variable axis (must be strictly ascending)
    /// * `data` - The dependent variable values (must match `x_axis` length)
    ///
    /// # Errors
    ///
    /// Returns `LutError` if:
    /// - `x_axis` is empty
    /// - `x_axis` is not strictly ascending
    /// - `data` length doesn't match `x_axis` length
    pub fn new(x_axis: Vec<f64>, data: Vec<f64>) -> Result<Self, LutError> {
        validate_axis(&x_axis, "X", LutError::EmptyXAxis)?;

        if data.len() != x_axis.len() {
            return Err(LutError::DimensionMismatch {
                expected: x_axis.len(),
                actual: data.len(),
            });
        }

        Ok(Self { x_axis, data })
    }

    /// Looks up and interpolates a value at the given x coordinate.
    ///
    /// Uses linear interpolation between adjacent points.
    /// Values outside the axis range are clamped to boundary values.
    #[inline]
    #[must_use]
    pub fn lookup(&self, x: f64) -> f64 {
        let (i, t) = find_interval(&self.x_axis, x);
        lerp(self.data[i], self.data[i + 1], t)
    }

    /// Returns the X axis values.
    #[must_use]
    pub fn x_axis(&self) -> &[f64] {
        &self.x_axis
    }

    /// Returns the data values.
    #[must_use]
    pub fn data(&self) -> &[f64] {
        &self.data
    }

    /// Returns the number of data points.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the LUT has no data points.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_exact_match() {
        let lut =
            Lut1D::new(vec![0.0, 1.0, 2.0, 3.0], vec![10.0, 20.0, 30.0, 40.0]).expect("valid LUT");

        assert!((lut.lookup(0.0) - 10.0).abs() < 1e-10);
        assert!((lut.lookup(1.0) - 20.0).abs() < 1e-10);
        assert!((lut.lookup(2.0) - 30.0).abs() < 1e-10);
        assert!((lut.lookup(3.0) - 40.0).abs() < 1e-10);
    }

    #[test]
    fn test_interpolation_midpoint() {
        let lut = Lut1D::new(vec![0.0, 2.0, 4.0], vec![0.0, 100.0, 200.0]).expect("valid LUT");

        assert!((lut.lookup(1.0) - 50.0).abs() < 1e-10);
        assert!((lut.lookup(3.0) - 150.0).abs() < 1e-10);
    }

    #[test]
    fn test_interpolation_quarter() {
        let lut = Lut1D::new(vec![0.0, 4.0], vec![0.0, 100.0]).expect("valid LUT");

        assert!((lut.lookup(1.0) - 25.0).abs() < 1e-10);
        assert!((lut.lookup(2.0) - 50.0).abs() < 1e-10);
        assert!((lut.lookup(3.0) - 75.0).abs() < 1e-10);
    }

    #[test]
    fn test_out_of_bounds_low() {
        let lut = Lut1D::new(vec![10.0, 20.0, 30.0], vec![100.0, 200.0, 300.0]).expect("valid LUT");

        assert!((lut.lookup(0.0) - 100.0).abs() < 1e-10);
        assert!((lut.lookup(-100.0) - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_out_of_bounds_high() {
        let lut = Lut1D::new(vec![10.0, 20.0, 30.0], vec![100.0, 200.0, 300.0]).expect("valid LUT");

        assert!((lut.lookup(40.0) - 300.0).abs() < 1e-10);
        assert!((lut.lookup(1000.0) - 300.0).abs() < 1e-10);
    }

    #[test]
    fn test_error_empty_axis() {
        let result = Lut1D::new(vec![], vec![]);
        assert!(matches!(result, Err(LutError::EmptyXAxis)));
    }

    #[test]
    fn test_error_unsorted_axis() {
        let result = Lut1D::new(vec![0.0, 2.0, 1.0], vec![0.0, 1.0, 2.0]);
        assert!(matches!(
            result,
            Err(LutError::UnsortedAxis {
                axis: "X",
                index: 2
            })
        ));
    }

    #[test]
    fn test_error_duplicate_values() {
        let result = Lut1D::new(vec![0.0, 1.0, 1.0, 2.0], vec![0.0, 1.0, 2.0, 3.0]);
        assert!(matches!(
            result,
            Err(LutError::UnsortedAxis {
                axis: "X",
                index: 2
            })
        ));
    }

    #[test]
    fn test_error_dimension_mismatch() {
        let result = Lut1D::new(vec![0.0, 1.0, 2.0], vec![0.0, 1.0]);
        assert!(matches!(
            result,
            Err(LutError::DimensionMismatch {
                expected: 3,
                actual: 2
            })
        ));
    }

    #[test]
    fn test_accessors() {
        let lut = Lut1D::new(vec![1.0, 2.0, 3.0], vec![10.0, 20.0, 30.0]).expect("valid LUT");

        assert_eq!(lut.x_axis(), &[1.0, 2.0, 3.0]);
        assert_eq!(lut.data(), &[10.0, 20.0, 30.0]);
        assert_eq!(lut.len(), 3);
        assert!(!lut.is_empty());
    }

    #[test]
    fn test_many_lookups() {
        let n = 100;
        let x_axis: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let data: Vec<f64> = (0..n).map(|i| (i * i) as f64).collect();
        let lut = Lut1D::new(x_axis, data).expect("valid LUT");

        for i in 0..10000 {
            let x = (i % 99) as f64 + 0.5;
            let _ = lut.lookup(x);
        }
    }
}
