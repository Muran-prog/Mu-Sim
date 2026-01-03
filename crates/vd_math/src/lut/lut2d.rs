//! 2D lookup table implementation.

use alloc::vec::Vec;

use super::{find_interval, lerp, validate_axis, LutError};

/// 2D lookup table for z = f(x, y) interpolation.
///
/// Data is stored in row-major order: `data[yi * nx + xi]`
///
/// # Example
///
/// ```
/// use vd_math::lut::Lut2D;
///
/// // Tire grip map: slip_angle x slip_ratio -> grip coefficient
/// let slip_angle = vec![0.0, 5.0, 10.0];
/// let slip_ratio = vec![0.0, 0.1];
/// let grip = vec![
///     0.0, 0.8, 1.0,  // slip_ratio = 0.0
///     0.5, 1.0, 0.9,  // slip_ratio = 0.1
/// ];
/// let lut = Lut2D::new(slip_angle, slip_ratio, grip).unwrap();
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Lut2D {
    x_axis: Vec<f64>,
    y_axis: Vec<f64>,
    data: Vec<f64>,
}

impl Lut2D {
    /// Creates a new 2D lookup table.
    ///
    /// # Arguments
    ///
    /// * `x_axis` - The first independent variable axis (columns)
    /// * `y_axis` - The second independent variable axis (rows)
    /// * `data` - The dependent variable values in row-major order
    ///
    /// # Errors
    ///
    /// Returns `LutError` if any axis is empty, unsorted, or dimensions don't match.
    pub fn new(x_axis: Vec<f64>, y_axis: Vec<f64>, data: Vec<f64>) -> Result<Self, LutError> {
        validate_axis(&x_axis, "X", LutError::EmptyXAxis)?;
        validate_axis(&y_axis, "Y", LutError::EmptyYAxis)?;

        let expected = x_axis.len() * y_axis.len();
        if data.len() != expected {
            return Err(LutError::DimensionMismatch {
                expected,
                actual: data.len(),
            });
        }

        Ok(Self {
            x_axis,
            y_axis,
            data,
        })
    }

    /// Looks up and interpolates a value at the given (x, y) coordinates.
    ///
    /// Uses bilinear interpolation between adjacent points.
    /// Values outside the axis ranges are clamped to boundary values.
    #[inline]
    #[must_use]
    pub fn lookup(&self, x: f64, y: f64) -> f64 {
        let (xi, tx) = find_interval(&self.x_axis, x);
        let (yi, ty) = find_interval(&self.y_axis, y);

        let x_len = self.x_axis.len();

        // Get the four corner values
        let v00 = self.data[yi * x_len + xi];
        let v10 = self.data[yi * x_len + xi + 1];
        let v01 = self.data[(yi + 1) * x_len + xi];
        let v11 = self.data[(yi + 1) * x_len + xi + 1];

        // Bilinear interpolation
        let v0 = lerp(v00, v10, tx);
        let v1 = lerp(v01, v11, tx);
        lerp(v0, v1, ty)
    }

    /// Returns the X axis values.
    #[must_use]
    pub fn x_axis(&self) -> &[f64] {
        &self.x_axis
    }

    /// Returns the Y axis values.
    #[must_use]
    pub fn y_axis(&self) -> &[f64] {
        &self.y_axis
    }

    /// Returns the data values in row-major order.
    #[must_use]
    pub fn data(&self) -> &[f64] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    fn create_test_lut() -> Lut2D {
        // 3x2 grid:
        //         x=0  x=1  x=2
        // y=0  [  0,  10,  20 ]
        // y=1  [ 100, 110, 120 ]
        Lut2D::new(
            vec![0.0, 1.0, 2.0],
            vec![0.0, 1.0],
            vec![0.0, 10.0, 20.0, 100.0, 110.0, 120.0],
        )
        .expect("valid LUT")
    }

    #[test]
    fn test_exact_match() {
        let lut = create_test_lut();

        assert!((lut.lookup(0.0, 0.0) - 0.0).abs() < 1e-10);
        assert!((lut.lookup(1.0, 0.0) - 10.0).abs() < 1e-10);
        assert!((lut.lookup(2.0, 0.0) - 20.0).abs() < 1e-10);
        assert!((lut.lookup(0.0, 1.0) - 100.0).abs() < 1e-10);
        assert!((lut.lookup(2.0, 1.0) - 120.0).abs() < 1e-10);
    }

    #[test]
    fn test_interpolation_x() {
        let lut = create_test_lut();

        assert!((lut.lookup(0.5, 0.0) - 5.0).abs() < 1e-10);
        assert!((lut.lookup(1.5, 0.0) - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_interpolation_y() {
        let lut = create_test_lut();

        assert!((lut.lookup(0.0, 0.5) - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_bilinear_interpolation() {
        let lut = create_test_lut();

        // Center point (0.5, 0.5): corners 0, 10, 100, 110 -> 55
        let center = lut.lookup(0.5, 0.5);
        assert!((center - 55.0).abs() < 1e-10);
    }

    #[test]
    fn test_out_of_bounds() {
        let lut = create_test_lut();

        assert!((lut.lookup(-1.0, 0.5) - 50.0).abs() < 1e-10);
        assert!((lut.lookup(10.0, 0.5) - 70.0).abs() < 1e-10);
        assert!((lut.lookup(0.5, -1.0) - 5.0).abs() < 1e-10);
        assert!((lut.lookup(0.5, 10.0) - 105.0).abs() < 1e-10);
    }

    #[test]
    fn test_error_empty_axis() {
        let result = Lut2D::new(vec![], vec![0.0], vec![]);
        assert!(matches!(result, Err(LutError::EmptyXAxis)));

        let result = Lut2D::new(vec![0.0], vec![], vec![]);
        assert!(matches!(result, Err(LutError::EmptyYAxis)));
    }

    #[test]
    fn test_error_dimension_mismatch() {
        let result = Lut2D::new(vec![0.0, 1.0], vec![0.0, 1.0], vec![0.0, 1.0, 2.0]);
        assert!(matches!(
            result,
            Err(LutError::DimensionMismatch {
                expected: 4,
                actual: 3
            })
        ));
    }

    #[test]
    fn test_many_lookups() {
        let nx = 50;
        let ny = 50;
        let x_axis: Vec<f64> = (0..nx).map(|i| i as f64).collect();
        let y_axis: Vec<f64> = (0..ny).map(|i| i as f64).collect();
        let data: Vec<f64> = (0..nx * ny).map(|i| i as f64).collect();
        let lut = Lut2D::new(x_axis, y_axis, data).expect("valid LUT");

        for i in 0..10000 {
            let x = (i % 49) as f64 + 0.5;
            let y = ((i / 49) % 49) as f64 + 0.5;
            let _ = lut.lookup(x, y);
        }
    }
}
