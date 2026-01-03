//! 3D lookup table implementation.

use alloc::vec::Vec;

use super::{find_interval, lerp, validate_axis, LutError};

/// 3D lookup table for w = f(x, y, z) interpolation.
///
/// Data is stored in a linearized format: `data[zi * (nx * ny) + yi * nx + xi]`
///
/// # Example
///
/// ```
/// use vd_math::lut::Lut3D;
///
/// // Complex aerodynamic coefficient: speed x angle x altitude -> Cd
/// let speed = vec![0.0, 50.0];
/// let angle = vec![0.0, 10.0];
/// let altitude = vec![0.0, 1000.0];
/// let cd = vec![
///     // altitude = 0
///     0.30, 0.35,  // angle = 0
///     0.40, 0.45,  // angle = 10
///     // altitude = 1000
///     0.28, 0.33,  // angle = 0
///     0.38, 0.43,  // angle = 10
/// ];
/// let lut = Lut3D::new(speed, angle, altitude, cd).unwrap();
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Lut3D {
    x_axis: Vec<f64>,
    y_axis: Vec<f64>,
    z_axis: Vec<f64>,
    data: Vec<f64>,
}

impl Lut3D {
    /// Creates a new 3D lookup table.
    ///
    /// # Arguments
    ///
    /// * `x_axis` - The first independent variable axis
    /// * `y_axis` - The second independent variable axis
    /// * `z_axis` - The third independent variable axis
    /// * `data` - The dependent variable values in linearized order
    ///
    /// # Errors
    ///
    /// Returns `LutError` if any axis is empty, unsorted, or dimensions don't match.
    pub fn new(
        x_axis: Vec<f64>,
        y_axis: Vec<f64>,
        z_axis: Vec<f64>,
        data: Vec<f64>,
    ) -> Result<Self, LutError> {
        validate_axis(&x_axis, "X", LutError::EmptyXAxis)?;
        validate_axis(&y_axis, "Y", LutError::EmptyYAxis)?;
        validate_axis(&z_axis, "Z", LutError::EmptyZAxis)?;

        let expected = x_axis.len() * y_axis.len() * z_axis.len();
        if data.len() != expected {
            return Err(LutError::DimensionMismatch {
                expected,
                actual: data.len(),
            });
        }

        Ok(Self {
            x_axis,
            y_axis,
            z_axis,
            data,
        })
    }

    /// Looks up and interpolates a value at the given (x, y, z) coordinates.
    ///
    /// Uses trilinear interpolation between adjacent points (8 corners of a cube).
    /// Values outside the axis ranges are clamped to boundary values.
    #[inline]
    #[must_use]
    #[allow(clippy::similar_names)]
    pub fn lookup(&self, x: f64, y: f64, z: f64) -> f64 {
        let (xi, tx) = find_interval(&self.x_axis, x);
        let (yi, ty) = find_interval(&self.y_axis, y);
        let (zi, tz) = find_interval(&self.z_axis, z);

        let nx = self.x_axis.len();
        let nxy = nx * self.y_axis.len();

        // Get the eight corner values of the cube
        let idx = |ix: usize, iy: usize, iz: usize| iz * nxy + iy * nx + ix;

        let c000 = self.data[idx(xi, yi, zi)];
        let c100 = self.data[idx(xi + 1, yi, zi)];
        let c010 = self.data[idx(xi, yi + 1, zi)];
        let c110 = self.data[idx(xi + 1, yi + 1, zi)];
        let c001 = self.data[idx(xi, yi, zi + 1)];
        let c101 = self.data[idx(xi + 1, yi, zi + 1)];
        let c011 = self.data[idx(xi, yi + 1, zi + 1)];
        let c111 = self.data[idx(xi + 1, yi + 1, zi + 1)];

        // Trilinear interpolation: X -> Y -> Z
        let c00 = lerp(c000, c100, tx);
        let c10 = lerp(c010, c110, tx);
        let c01 = lerp(c001, c101, tx);
        let c11 = lerp(c011, c111, tx);

        let c0 = lerp(c00, c10, ty);
        let c1 = lerp(c01, c11, ty);

        lerp(c0, c1, tz)
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

    /// Returns the Z axis values.
    #[must_use]
    pub fn z_axis(&self) -> &[f64] {
        &self.z_axis
    }

    /// Returns the data values.
    #[must_use]
    pub fn data(&self) -> &[f64] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    fn create_test_lut() -> Lut3D {
        // 2x2x2 cube
        Lut3D::new(
            vec![0.0, 1.0],
            vec![0.0, 1.0],
            vec![0.0, 1.0],
            vec![
                // z=0
                0.0, 1.0, // y=0
                10.0, 11.0, // y=1
                // z=1
                100.0, 101.0, // y=0
                110.0, 111.0, // y=1
            ],
        )
        .expect("valid LUT")
    }

    #[test]
    fn test_exact_match_corners() {
        let lut = create_test_lut();

        assert!((lut.lookup(0.0, 0.0, 0.0) - 0.0).abs() < 1e-10);
        assert!((lut.lookup(1.0, 0.0, 0.0) - 1.0).abs() < 1e-10);
        assert!((lut.lookup(0.0, 1.0, 0.0) - 10.0).abs() < 1e-10);
        assert!((lut.lookup(1.0, 1.0, 0.0) - 11.0).abs() < 1e-10);
        assert!((lut.lookup(0.0, 0.0, 1.0) - 100.0).abs() < 1e-10);
        assert!((lut.lookup(1.0, 1.0, 1.0) - 111.0).abs() < 1e-10);
    }

    #[test]
    fn test_trilinear_center() {
        let lut = create_test_lut();

        // Center: (0+1+10+11+100+101+110+111)/8 = 55.5
        let center = lut.lookup(0.5, 0.5, 0.5);
        assert!((center - 55.5).abs() < 1e-10);
    }

    #[test]
    fn test_interpolation_single_axis() {
        let lut = create_test_lut();

        assert!((lut.lookup(0.0, 0.0, 0.5) - 50.0).abs() < 1e-10);
        assert!((lut.lookup(0.5, 0.0, 0.0) - 0.5).abs() < 1e-10);
        assert!((lut.lookup(0.0, 0.5, 0.0) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_out_of_bounds() {
        let lut = create_test_lut();

        assert!((lut.lookup(-1.0, -1.0, -1.0) - 0.0).abs() < 1e-10);
        assert!((lut.lookup(10.0, 10.0, 10.0) - 111.0).abs() < 1e-10);
    }

    #[test]
    fn test_error_empty_axis() {
        let result = Lut3D::new(vec![], vec![0.0], vec![0.0], vec![]);
        assert!(matches!(result, Err(LutError::EmptyXAxis)));

        let result = Lut3D::new(vec![0.0], vec![], vec![0.0], vec![]);
        assert!(matches!(result, Err(LutError::EmptyYAxis)));

        let result = Lut3D::new(vec![0.0], vec![0.0], vec![], vec![]);
        assert!(matches!(result, Err(LutError::EmptyZAxis)));
    }

    #[test]
    fn test_error_dimension_mismatch() {
        let result = Lut3D::new(vec![0.0, 1.0], vec![0.0, 1.0], vec![0.0, 1.0], vec![0.0; 4]);
        assert!(matches!(
            result,
            Err(LutError::DimensionMismatch {
                expected: 8,
                actual: 4
            })
        ));
    }

    #[test]
    fn test_many_lookups() {
        let n = 10;
        let axis: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let data: Vec<f64> = (0..n * n * n).map(|i| i as f64).collect();
        let lut = Lut3D::new(axis.clone(), axis.clone(), axis, data).expect("valid LUT");

        for i in 0..10000 {
            let x = (i % 9) as f64 + 0.5;
            let y = ((i / 9) % 9) as f64 + 0.5;
            let z = ((i / 81) % 9) as f64 + 0.5;
            let _ = lut.lookup(x, y, z);
        }
    }
}
