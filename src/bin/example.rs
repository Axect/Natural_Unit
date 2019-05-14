extern crate natural_unit;
use natural_unit::*;

fn main() {
    let cgs_to_geom = ConversionFactor::new(
        CONSTANT_CGS.G / (CONSTANT_CGS.c.powi(2) * CONSTANT_CGS.r_solar),   // Mass conversion factor
        1f64 / CONSTANT_CGS.r_solar,                                        // Length conversion factor
        CONSTANT_CGS.c / CONSTANT_CGS.r_solar                               // Time conversion factor
    );

    let solar_mass_cgs = CONSTANT_CGS.m_solar;                              // CGS Solar Mass
    let new_solar_mass = convert(solar_mass_cgs, Mass, cgs_to_geom);        // Converted Solar Mass
    println!("{}", new_solar_mass);

    let solar_radius_cgs = CONSTANT_CGS.r_solar;
    println!("{}", convert(solar_radius_cgs, Length, cgs_to_geom));
}