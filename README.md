# Natural Unit

[![On crates.io](https://img.shields.io/crates/v/natural_unit.svg)](https://crates.io/crates/natural_unit)
[![On docs.rs](https://docs.rs/natural_unit/badge.svg)](https://docs.rs/natural_unit/)
[![travis](https://api.travis-ci.org/Axect/Natural_Unit.svg?branch=master)](https://travis-ci.org/Axect/Natural_Unit)  
![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

Physical unit conversion with Fundamental constants

## Unit System List

- [x] SI
- [x] CGS
- [x] Geometrized(With CGS)
- [x] Geometrized with unit solar mass
- [x] Natural Unit(With CGS)
- [x] Natural Unit(MeV)
- [x] Natural Unit(Gaussian)

## How to use?

1. Just declare three conversion factors - M, L, T
2. Get automatically obtained remain conversion factors (momentum, energy and etc.)
3. `convert` value using by declared conversion factor structure.

**Example: CGS -> Geometrized with Solar radius = 1**

```rust
extern crate natural_unit;
use natural_unit::*;

fn main() {
    // Declare mass, length, time conversion factor.
    let cgs_to_geom = ConversionFactor::new(
        CONSTANT_CGS.G / (CONSTANT_CGS.c.powi(2) * CONSTANT_CGS.r_solar),   // Mass conversion factor
        1f64 / CONSTANT_CGS.r_solar,                                        // Length conversion factor
        CONSTANT_CGS.c / CONSTANT_CGS.r_solar                               // Time conversion factor
    );
    
    // What do you want to convert?
    let solar_mass_cgs = CONSTANT_CGS.m_solar;                              // CGS Solar Mass
    
    // Convert! (`convert(f64, Dimension, ConversionFactor)`)
    let new_solar_mass = convert(solar_mass_cgs, Mass, cgs_to_geom);        // Converted Solar Mass
    
    // Invert! (`invert(f64, Dimension, ConversionFactor)`)
    let inverted_solar_mass = invert(new_solar_mass, Mass, cgs_to_geom);    // Inverted Solar Mass
    assert_eq!(solar_mass_cgs, inverted_solar_mass);
}
```
