#![allow(unused)]

use clap::{arg, command};
use kyber_rust::params::Params;

fn main() {
    let matches = command!()
        .arg(
            arg!([SECURITYLEVEL])
                .help(
                    "The security level to be run.
                      kyber512 ~= AES128
                      kyber768 ~= AES192
                      kyber1024 ~= AES256\n",
                )
                .value_parser(["512", "768", "1024"])
                .required(false)
                .default_value("768"),
        )
        .get_matches();

    let bits: &String = matches.get_one::<String>("SECURITYLEVEL").expect("");
    println!("{}", bits);
    let params;
    match bits as &str {
        "512" => {
            params = Params::sec_level_512();
        }
        "768" => {
            params = Params::sec_level_768();
        }
        "1024" => {
            params = Params::sec_level_1024();
        }
        _ => panic!("Invalid security level"),
    }
}
