pub mod kem;
mod params;
mod kex;
mod indcpa;

pub fn print_sec() {
    println!("{}", crate::params::K);
    if cfg!(features = "kyber512") {
        println!("feature 512");
    } else {
        println!("not feature 512");
    }
}
