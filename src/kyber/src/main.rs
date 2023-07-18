use clap::{command, arg};
use kyber_rust::params::*;

fn main() {
    let matches = command!()
        .arg(
            arg!([SECURITYLEVEL])
                .help("The security level to be run.
                      kyber512 ~= AES128
                      kyber768 ~= AES192
                      kyber1024 ~= AES256\n")
                .value_parser(["512", "768", "1024"])
                .required(false)
                .default_value("768"),
        )
        .get_matches();

    let bits: &String = matches
        .get_one::<String>("SECURITYLEVEL")
        .expect("");
    println!("{}", bits);
    match &bits as &str {
        "512" => {impl ParamsTemplate for Params {
            const K: usize = K_512;
            const ETA: usize = 5;
            const POLYVECBYTES: usize = Self::K * POLYBYTES;
            const POLYVECCOMPRESSEDBYTES: usize = Self::K * 320;
            const INDCPAPUBLICKEYBYTES: usize = Self::POLYVECBYTES + SYMBYTES;
            const INDCPASECRETKEYBYTES: usize = Self::POLYVECBYTES;
            const INDCPABYTES: usize = Self::POLYVECCOMPRESSEDBYTES + POLYCOMPRESSEDBYTES;
            const PUBLICKEYBYTES: usize = Self::INDCPAPUBLICKEYBYTES;
            const SECRETKEYBYTES: usize = Self::INDCPASECRETKEYBYTES + Self::INDCPAPUBLICKEYBYTES + 2 * SYMBYTES;
            const CIPHERTEXTBYTES: usize = Self::INDCPABYTES;
        }},
        "768" => {impl ParamsTemplate for Params {
            const K: usize = K_768;
            const ETA: usize = 5;
            const POLYVECBYTES: usize = Self::K * POLYBYTES;
            const POLYVECCOMPRESSEDBYTES: usize = Self::K * 320;
            const INDCPAPUBLICKEYBYTES: usize = Self::POLYVECBYTES + SYMBYTES;
            const INDCPASECRETKEYBYTES: usize = Self::POLYVECBYTES;
            const INDCPABYTES: usize = Self::POLYVECCOMPRESSEDBYTES + POLYCOMPRESSEDBYTES;
            const PUBLICKEYBYTES: usize = Self::INDCPAPUBLICKEYBYTES;
            const SECRETKEYBYTES: usize = Self::INDCPASECRETKEYBYTES + Self::INDCPAPUBLICKEYBYTES + 2 * SYMBYTES;
            const CIPHERTEXTBYTES: usize = Self::INDCPABYTES;
        }},
        "1024" => {impl ParamsTemplate for Params {
            const K: usize = K_1024;
            const ETA: usize = 5;
            const POLYVECBYTES: usize = Self::K * POLYBYTES;
            const POLYVECCOMPRESSEDBYTES: usize = Self::K * 320;
            const INDCPAPUBLICKEYBYTES: usize = Self::POLYVECBYTES + SYMBYTES;
            const INDCPASECRETKEYBYTES: usize = Self::POLYVECBYTES;
            const INDCPABYTES: usize = Self::POLYVECCOMPRESSEDBYTES + POLYCOMPRESSEDBYTES;
            const PUBLICKEYBYTES: usize = Self::INDCPAPUBLICKEYBYTES;
            const SECRETKEYBYTES: usize = Self::INDCPASECRETKEYBYTES + Self::INDCPAPUBLICKEYBYTES + 2 * SYMBYTES;
            const CIPHERTEXTBYTES: usize = Self::INDCPABYTES;
        }},
        _ => panic!("Invalid security level"),
    }
}

