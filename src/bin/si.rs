extern crate natural_unit;
use natural_unit::*;

fn main() {
    let cgs_to_si = ConversionFactor::new(1E+3, 1E+2, 1f64);
    println!("{:#?}", cgs_to_si);
}