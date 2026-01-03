//! Core interpolation utilities.

use super::LutError;

/// Validates that an axis is non-empty and strictly ascending.
pub(super) fn validate_axis(
    axis: &[f64],
    name: &'static str,
    empty_err: LutError,
) -> Result<(), LutError> {
    if axis.is_empty() {
        return Err(empty_err);
    }
    for i in 1..axis.len() {
        if axis[i] <= axis[i - 1] {
            return Err(LutError::UnsortedAxis {
                axis: name,
                index: i,
            });
        }
    }
    Ok(())
}

/// Binary search to find the interval containing a value.
/// Returns the lower index and interpolation factor t in [0, 1].
/// Clamps to boundaries if x is outside the axis range.
#[inline]
pub(super) fn find_interval(axis: &[f64], x: f64) -> (usize, f64) {
    let n = axis.len();

    // Handle boundary cases (clamping)
    if x <= axis[0] {
        return (0, 0.0);
    }
    if x >= axis[n - 1] {
        return (n.saturating_sub(2), 1.0);
    }

    // Binary search for the interval
    let mut lo = 0;
    let mut hi = n - 1;

    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        if axis[mid] <= x {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    // Calculate interpolation factor
    let x0 = axis[lo];
    let x1 = axis[hi];
    let t = (x - x0) / (x1 - x0);

    (lo, t)
}

/// Linear interpolation between two values.
#[inline]
pub(super) fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}
