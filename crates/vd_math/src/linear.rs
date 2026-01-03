//! Linear algebra types and helpers.
//!
//! This module provides convenient type aliases and helper functions
//! for working with `nalgebra` types in the vehicle dynamics context.

use nalgebra::{Matrix3, UnitQuaternion, Vector3};

/// 3D vector with f64 precision.
pub type Vec3 = Vector3<f64>;

/// 3x3 matrix with f64 precision.
pub type Mat3 = Matrix3<f64>;

/// Unit quaternion for rotations with f64 precision.
pub type Quat = UnitQuaternion<f64>;

/// Creates a new 3D vector from components.
#[inline]
#[must_use]
pub const fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

/// Creates the zero vector.
#[inline]
#[must_use]
pub fn vec3_zero() -> Vec3 {
    Vec3::zeros()
}

/// Creates a unit vector along X axis.
#[inline]
#[must_use]
pub fn vec3_x() -> Vec3 {
    Vec3::x()
}

/// Creates a unit vector along Y axis.
#[inline]
#[must_use]
pub fn vec3_y() -> Vec3 {
    Vec3::y()
}

/// Creates a unit vector along Z axis.
#[inline]
#[must_use]
pub fn vec3_z() -> Vec3 {
    Vec3::z()
}

/// Creates the identity matrix.
#[inline]
#[must_use]
pub fn mat3_identity() -> Mat3 {
    Mat3::identity()
}

/// Creates a rotation matrix from Euler angles (roll, pitch, yaw) in radians.
/// Uses the ZYX convention (yaw-pitch-roll).
#[inline]
#[must_use]
pub fn mat3_from_euler(roll: f64, pitch: f64, yaw: f64) -> Mat3 {
    let quat = Quat::from_euler_angles(roll, pitch, yaw);
    *quat.to_rotation_matrix().matrix()
}

/// Creates an identity quaternion (no rotation).
#[inline]
#[must_use]
pub fn quat_identity() -> Quat {
    Quat::identity()
}

/// Creates a quaternion from Euler angles (roll, pitch, yaw) in radians.
#[inline]
#[must_use]
pub fn quat_from_euler(roll: f64, pitch: f64, yaw: f64) -> Quat {
    Quat::from_euler_angles(roll, pitch, yaw)
}

/// Creates a quaternion from axis-angle representation.
#[inline]
#[must_use]
pub fn quat_from_axis_angle(axis: &Vec3, angle: f64) -> Quat {
    Quat::from_axis_angle(&nalgebra::Unit::new_normalize(*axis), angle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_creation() {
        let v = vec3(1.0, 2.0, 3.0);
        assert!((v.x - 1.0).abs() < 1e-10);
        assert!((v.y - 2.0).abs() < 1e-10);
        assert!((v.z - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec3_magnitude() {
        let v = vec3(3.0, 4.0, 0.0);
        assert!((v.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec3_dot() {
        let a = vec3(1.0, 0.0, 0.0);
        let b = vec3(0.0, 1.0, 0.0);
        assert!((a.dot(&b)).abs() < 1e-10);
    }

    #[test]
    fn test_vec3_cross() {
        let x = vec3_x();
        let y = vec3_y();
        let z = x.cross(&y);
        assert!((z.x).abs() < 1e-10);
        assert!((z.y).abs() < 1e-10);
        assert!((z.z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_mat3_identity() {
        let m = mat3_identity();
        let v = vec3(1.0, 2.0, 3.0);
        let result = m * v;
        assert!((result - v).magnitude() < 1e-10);
    }

    #[test]
    fn test_quat_identity() {
        let q = quat_identity();
        let v = vec3(1.0, 2.0, 3.0);
        let result = q * v;
        assert!((result - v).magnitude() < 1e-10);
    }

    #[test]
    fn test_quat_rotation() {
        use core::f64::consts::FRAC_PI_2;
        
        // 90 degree rotation around Z axis
        let q = quat_from_axis_angle(&vec3_z(), FRAC_PI_2);
        let v = vec3_x();
        let result = q * v;
        
        // X unit vector should become Y unit vector
        assert!((result.x).abs() < 1e-10);
        assert!((result.y - 1.0).abs() < 1e-10);
        assert!((result.z).abs() < 1e-10);
    }
}
