use clap::{command, arg};
use kyber512;
use kyber768;
use kyber1024;

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
        "512" => kyber512::test_512(),
        "768" => kyber768::test_768(),
        "1024" => kyber1024::test_1024(),
        _ => panic!("Invalid security level"),
    }
}

