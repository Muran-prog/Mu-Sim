//! SI unit types with type-safe arithmetic operations.
//!
//! All types use the newtype pattern with `#[repr(transparent)]` for zero-cost
//! abstractions. Arithmetic operations are only defined where physically meaningful.

#[macro_use]
mod macros;

mod angular;
mod base;
mod derived;
mod motion;
mod ops;

pub use angular::*;
pub use base::*;
pub use derived::*;
pub use motion::*;

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_basic_arithmetic() {
        let m1 = Meters(10.0);
        let m2 = Meters(5.0);

        assert!(approx_eq((m1 + m2).0, 15.0));
        assert!(approx_eq((m1 - m2).0, 5.0));
        assert!(approx_eq((m1 * 2.0).0, 20.0));
        assert!(approx_eq((m1 / 2.0).0, 5.0));
    }

    #[test]
    fn test_velocity_calculation() {
        let distance = Meters(100.0);
        let time = Seconds(10.0);
        let velocity = distance / time;

        assert!(approx_eq(velocity.0, 10.0));
    }

    #[test]
    fn test_velocity_time_distance() {
        let velocity = MetersPerSecond(20.0);
        let time = Seconds(5.0);
        let distance = velocity * time;

        assert!(approx_eq(distance.0, 100.0));
    }

    #[test]
    fn test_acceleration_calculation() {
        let velocity = MetersPerSecond(30.0);
        let time = Seconds(3.0);
        let acceleration = velocity / time;

        assert!(approx_eq(acceleration.0, 10.0));
    }

    #[test]
    fn test_force_calculation() {
        let mass = Kilograms(10.0);
        let acceleration = MetersPerSecondSquared(9.806_65);
        let force = mass * acceleration;

        assert!(approx_eq(force.0, 98.0665));
    }

    #[test]
    fn test_degrees_radians_conversion() {
        let rad = Radians::from_degrees(180.0);
        assert!(approx_eq(rad.0, core::f64::consts::PI));

        let deg = rad.as_degrees();
        assert!(approx_eq(deg, 180.0));

        let rad90 = Radians::from_degrees(90.0);
        assert!(approx_eq(rad90.0, core::f64::consts::FRAC_PI_2));
    }

    #[test]
    fn test_rpm_conversion() {
        let rpm = RPM(60.0);
        let rad_per_sec = rpm.to_rad_per_sec();

        // 60 RPM = 1 revolution per second = 2*PI rad/s
        assert!(approx_eq(rad_per_sec.0, 2.0 * core::f64::consts::PI));

        let rpm_back = RPM::from_rad_per_sec(rad_per_sec);
        assert!(approx_eq(rpm_back.0, 60.0));
    }

    #[test]
    fn test_pressure_conversions() {
        let pressure = Pascals::from_bar(1.0);
        assert!(approx_eq(pressure.0, 100_000.0));
        assert!(approx_eq(pressure.as_bar(), 1.0));

        let pressure_kpa = Pascals::from_kpa(100.0);
        assert!(approx_eq(pressure_kpa.0, 100_000.0));

        let pressure_psi = Pascals::from_psi(14.696);
        assert!((pressure_psi.0 - 101_325.0).abs() < 100.0); // ~1 atm
    }

    #[test]
    fn test_temperature_conversions() {
        let temp = Kelvin::from_celsius(0.0);
        assert!(approx_eq(temp.0, 273.15));
        assert!(approx_eq(temp.as_celsius(), 0.0));

        let temp_f = Kelvin::from_fahrenheit(32.0);
        assert!(approx_eq(temp_f.0, 273.15));

        let boiling = Kelvin::from_celsius(100.0);
        assert!(approx_eq(boiling.as_fahrenheit(), 212.0));
    }

    #[test]
    fn test_velocity_conversions() {
        let velocity = MetersPerSecond::from_kmh(36.0);
        assert!(approx_eq(velocity.0, 10.0));
        assert!(approx_eq(velocity.as_kmh(), 36.0));

        let velocity_mph = MetersPerSecond::from_mph(60.0);
        assert!((velocity_mph.0 - 26.8224).abs() < 0.001);
    }

    #[test]
    fn test_g_force_conversion() {
        let accel = MetersPerSecondSquared::from_g(1.0);
        assert!(approx_eq(accel.0, 9.806_65));
        assert!(approx_eq(accel.as_g(), 1.0));
    }

    #[test]
    fn test_power_from_torque_and_rpm() {
        let torque = NewtonMeters(100.0);
        let angular_velocity = RadiansPerSecond(100.0);
        let power = torque * angular_velocity;

        assert!(approx_eq(power.0, 10_000.0));
    }

    #[test]
    fn test_radians_normalize() {
        let angle = Radians(3.0 * core::f64::consts::PI);
        let normalized = angle.normalize();
        assert!(approx_eq(normalized.0, core::f64::consts::PI));

        let negative = Radians(-core::f64::consts::FRAC_PI_2);
        let normalized_neg = negative.normalize();
        assert!(approx_eq(normalized_neg.0, 3.0 * core::f64::consts::FRAC_PI_2));
    }

    #[test]
    fn test_trig_functions() {
        let angle = Radians::from_degrees(30.0);
        assert!((angle.sin() - 0.5).abs() < 1e-10);

        let angle90 = Radians::from_degrees(90.0);
        assert!((angle90.sin() - 1.0).abs() < 1e-10);
        assert!(angle90.cos().abs() < 1e-10);
    }
}
