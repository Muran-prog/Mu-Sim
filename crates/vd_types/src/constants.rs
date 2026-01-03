//! Physical constants used in vehicle dynamics calculations.
//!
//! All constants are defined with SI units and high precision values
//! from recognized scientific sources.

use crate::units::{Kelvin, MetersPerSecondSquared, Pascals};

/// Standard acceleration due to gravity (m/s^2).
///
/// This is the standard value defined by ISO 80000-3:2006.
pub const G_FORCE: MetersPerSecondSquared = MetersPerSecondSquared(9.806_65);

/// Standard atmospheric pressure at sea level (Pa).
///
/// Defined as exactly 101325 Pa by ISO 2533:1975.
pub const ATMOSPHERIC_PRESSURE: Pascals = Pascals(101_325.0);

/// Standard air density at sea level and 15C (kg/m^3).
///
/// Based on ISA (International Standard Atmosphere) conditions:
/// - Temperature: 288.15 K (15C)
/// - Pressure: 101325 Pa
/// - Humidity: 0%
pub const AIR_DENSITY_STD: f64 = 1.225;

/// Standard temperature for ISA conditions (K).
///
/// 288.15 K = 15 C
pub const TEMPERATURE_STD: Kelvin = Kelvin(288.15);

/// Specific gas constant for dry air (J/(kg*K)).
pub const GAS_CONSTANT_AIR: f64 = 287.058;

/// Ratio of specific heats for air (dimensionless).
///
/// gamma = Cp/Cv for diatomic gas
pub const GAMMA_AIR: f64 = 1.4;

/// Dynamic viscosity of air at 15C (Pa*s).
pub const AIR_VISCOSITY_STD: f64 = 1.81e-5;

/// Kinematic viscosity of air at 15C (m^2/s).
pub const AIR_KINEMATIC_VISCOSITY_STD: f64 = 1.48e-5;

/// Speed of sound in air at 15C (m/s).
pub const SPEED_OF_SOUND_STD: f64 = 340.3;

/// Water density at 4C (kg/m^3).
pub const WATER_DENSITY: f64 = 1000.0;

/// Absolute zero temperature (K).
pub const ABSOLUTE_ZERO: Kelvin = Kelvin(0.0);

/// Universal gas constant (J/(mol*K)).
pub const UNIVERSAL_GAS_CONSTANT: f64 = 8.314_462_618;

/// Boltzmann constant (J/K).
pub const BOLTZMANN_CONSTANT: f64 = 1.380_649e-23;

/// Pi constant (for convenience).
pub const PI: f64 = core::f64::consts::PI;

/// 2 * Pi (full rotation in radians).
pub const TWO_PI: f64 = 2.0 * core::f64::consts::PI;

/// Pi / 2 (quarter rotation in radians).
pub const HALF_PI: f64 = core::f64::consts::FRAC_PI_2;

/// Conversion factor: degrees to radians.
pub const DEG_TO_RAD: f64 = core::f64::consts::PI / 180.0;

/// Conversion factor: radians to degrees.
pub const RAD_TO_DEG: f64 = 180.0 / core::f64::consts::PI;

/// Conversion factor: km/h to m/s.
pub const KMH_TO_MS: f64 = 1.0 / 3.6;

/// Conversion factor: m/s to km/h.
pub const MS_TO_KMH: f64 = 3.6;

/// Conversion factor: mph to m/s.
pub const MPH_TO_MS: f64 = 0.447_04;

/// Conversion factor: m/s to mph.
pub const MS_TO_MPH: f64 = 1.0 / 0.447_04;

/// Conversion factor: bar to Pascal.
pub const BAR_TO_PA: f64 = 100_000.0;

/// Conversion factor: Pascal to bar.
pub const PA_TO_BAR: f64 = 1.0 / 100_000.0;

/// Conversion factor: PSI to Pascal.
pub const PSI_TO_PA: f64 = 6_894.757;

/// Conversion factor: Pascal to PSI.
pub const PA_TO_PSI: f64 = 1.0 / 6_894.757;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_g_force_value() {
        assert!((G_FORCE.0 - 9.80665).abs() < 1e-10);
    }

    #[test]
    fn test_atmospheric_pressure() {
        assert!((ATMOSPHERIC_PRESSURE.0 - 101325.0).abs() < 1e-10);
    }

    #[test]
    fn test_conversion_factors() {
        // 1 bar = 100000 Pa
        assert!((BAR_TO_PA - 100_000.0).abs() < 1e-10);

        // Round trip
        let bar = 2.5;
        let pa = bar * BAR_TO_PA;
        let bar_back = pa * PA_TO_BAR;
        assert!((bar - bar_back).abs() < 1e-10);
    }

    #[test]
    fn test_temperature_constant() {
        assert!((TEMPERATURE_STD.0 - 288.15).abs() < 1e-10);
    }
}
