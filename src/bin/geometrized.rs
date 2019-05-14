extern crate natural_unit;
use natural_unit::*;

fn main() {
    let cgs_to_geom = ConversionFactor::new(
        CONSTANT_CGS.G / CONSTANT_CGS.c.powi(2),
        1f64,
        CONSTANT_CGS.c,
    );
    println!("{:#?}", cgs_to_geom);
}