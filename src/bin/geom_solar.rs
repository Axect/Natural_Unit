extern crate natural_unit;
use natural_unit::*;

fn main() {
    let m_solar = CONSTANT_CGS.m_solar;
    let c = CONSTANT_CGS.c;
    let G = CONSTANT_CGS.G;
    let cgs_to_geom = ConversionFactor::new(
        1f64 / m_solar,
        c.powi(2) / (G * m_solar),
        c.powi(3) / (G * m_solar),
    );
    println!("{:#?}", cgs_to_geom);

    let r_solar = CONSTANT_CGS.r_solar;
    let new_r = convert(r_solar, Length, cgs_to_geom);
    println!("{}", new_r);
}
