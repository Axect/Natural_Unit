pub use self::Dimension::*;

/// Unit System
///
/// * SI
/// * CGS
/// * Geometrized   (c=G=1)
/// * Natural       (c=h=1)
pub enum UnitSystem {
    SI,
    CGS,
    Geometrized,
    Natural,
}

/// Dimension
///
/// * Time
/// * Length
/// * Mass
/// * Velocity
/// * Momentum
/// * Angular velocity
/// * Acceleration
/// * Energy
/// * Energy density
/// * Angular momentum
/// * Force
/// * Power
/// * Pressure
/// * Density
#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    Time,
    Length,
    Mass,
    Velocity,
    Momentum,
    AngularVelocity,
    Acceleration,
    Energy,
    EnergyDensity,
    AngularMomentum,
    Force,
    Power,
    Pressure,
    Density,
}

#[derive(Debug, Copy, Clone)]
pub struct ConversionFactor {
    pub conv_mass: f64,
    pub conv_length: f64,
    pub conv_time: f64,
    pub conv_velocity: f64,
    pub conv_momentum: f64,
    pub conv_angular_velocity: f64,
    pub conv_acceleration: f64,
    pub conv_energy: f64,
    pub conv_energy_density: f64,
    pub conv_angular_momentum: f64,
    pub conv_force: f64,
    pub conv_power: f64,
    pub conv_pressure: f64,
    pub conv_density: f64,
}

impl ConversionFactor {
    pub fn new(conv_mass: f64, conv_length: f64, conv_time: f64) -> Self {
        let conv_velocity = conv_length / conv_time;
        let conv_momentum = conv_mass * conv_velocity;
        let conv_angular_velocity = 1f64 / conv_time;
        let conv_acceleration = conv_velocity / conv_time;
        let conv_energy = conv_mass * conv_velocity.powi(2);
        let conv_energy_density = conv_energy / conv_length.powi(3);
        let conv_angular_momentum = conv_momentum * conv_length;
        let conv_force = conv_mass * conv_acceleration;
        let conv_power = conv_energy / conv_time;
        let conv_pressure = conv_force / conv_length.powi(2);
        let conv_density = conv_mass / conv_length.powi(3);

        ConversionFactor {
            conv_mass,
            conv_length,
            conv_time,
            conv_velocity,
            conv_momentum,
            conv_angular_velocity,
            conv_acceleration,
            conv_energy,
            conv_energy_density,
            conv_angular_momentum,
            conv_force,
            conv_power,
            conv_pressure,
            conv_density,
        }
    }
}

pub fn convert(value: f64, dim: Dimension, conv_factor: ConversionFactor) -> f64 {
    value * match dim {
        Time => { conv_factor.conv_time }
        Length => { conv_factor.conv_length }
        Mass => { conv_factor.conv_mass }
        Velocity => { conv_factor.conv_velocity }
        Momentum => { conv_factor.conv_momentum }
        AngularVelocity => { conv_factor.conv_angular_velocity }
        Acceleration => { conv_factor.conv_acceleration }
        Energy => { conv_factor.conv_energy }
        EnergyDensity => { conv_factor.conv_energy_density }
        AngularMomentum => { conv_factor.conv_angular_momentum }
        Force => { conv_factor.conv_force }
        Power => { conv_factor.conv_power }
        Pressure => { conv_factor.conv_pressure }
        Density => { conv_factor.conv_density }
    }
}

pub fn invert(value: f64, dim: Dimension, conv_factor: ConversionFactor) -> f64 {
    value / match dim {
        Time => { conv_factor.conv_time }
        Length => { conv_factor.conv_length }
        Mass => { conv_factor.conv_mass }
        Velocity => { conv_factor.conv_velocity }
        Momentum => { conv_factor.conv_momentum }
        AngularVelocity => { conv_factor.conv_angular_velocity }
        Acceleration => { conv_factor.conv_acceleration }
        Energy => { conv_factor.conv_energy }
        EnergyDensity => { conv_factor.conv_energy_density }
        AngularMomentum => { conv_factor.conv_angular_momentum }
        Force => { conv_factor.conv_force }
        Power => { conv_factor.conv_power }
        Pressure => { conv_factor.conv_pressure }
        Density => { conv_factor.conv_density }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone)]
pub struct FundamentalConstant {
    pub c: f64,
    pub G: f64,
    pub e: f64,
    pub k_b: f64,
    pub N_A: f64,
    pub h: f64,
    pub hbar: f64,
    pub m_u: f64,
    pub m_e: f64,
    pub eV: f64,
    pub m_solar: f64,
    pub r_solar: f64,
}

/// CGS Unit (cm, g, s)
/// 
/// # Reference
/// [NIST(2018)](http://physics.nist.gov/constants)
pub const CONSTANT_CGS: FundamentalConstant = FundamentalConstant {
    c: 2.99792458e+10,      // Speed of light (cm s^{-1})
    G: 6.67430e-8,          // Gravitational constants (cm^3 g^{-1} s^{-2})
    e: 1.602176634e-19,     // Elementary charge (C)
    k_b: 1.380649e-16,      // Boltzmann constant (erg K^{-1})
    N_A: 6.02214076e+23,    // Avogadro constant (mol^{-1})
    h: 6.62607015e-27,      // Planck constant (erg s)
    hbar: 1.05457182e-27,   // Planck Constant (erg s)
    m_u: 1.66053906660e-24, // Atomic mass unit (g)
    m_e: 9.1093837015e-28,  // Electron mass (g)
    eV: 1.602176634e-12,    // Electron Volt (erg)
    m_solar: 1.98848e+33,   // Solar mass (g)
    r_solar: 6.957e+10      // Solar radius (cm)
};

// =========================================================
// Reference Constants
// =========================================================
/// Default Conversion Factor
///
/// * CGS to SI : `cgs_to_si`
/// * CGS to Geometrized : `cgs_to_geom`
/// * CGS to Natural : `cgs_to_natural`
pub trait DefaultFactors {
    fn cgs_to_si() -> ConversionFactor;
    fn cgs_to_geom() -> ConversionFactor;
    fn cgs_to_natural() -> ConversionFactor;
}

pub struct Reference { }

impl DefaultFactors for Reference {
    fn cgs_to_si() -> ConversionFactor {
        ConversionFactor::new(1E+3, 1E+2, 1f64)
    }

    fn cgs_to_geom() -> ConversionFactor {
        ConversionFactor::new(
            CONSTANT_CGS.G / CONSTANT_CGS.c.powi(2),
            1f64,
            CONSTANT_CGS.c,
        )
    }

    fn cgs_to_natural() -> ConversionFactor {
        ConversionFactor::new(
            CONSTANT_CGS.eV / CONSTANT_CGS.hbar,
            CONSTANT_CGS.eV / (CONSTANT_CGS.hbar * CONSTANT_CGS.c),
            CONSTANT_CGS.c.powi(2) / CONSTANT_CGS.eV,
        )
    }
}

