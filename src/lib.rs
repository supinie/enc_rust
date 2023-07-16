pub mod kem;
mod params;
mod kex;
mod indcpa;

pub fn print_sec() {
    println!("{}", crate::params::K);
}
