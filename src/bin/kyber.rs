use clap::{command, arg};

fn main() {
    let matches = command!()
        .arg(
            arg!([SECURITYLEVEL])
                .help("The security level to be run.
                      kyber512 ~= AES-128
                      kyber768 ~= AES-192
                      kyber1024 ~= AES-256\n")
                .value_parser(["512", "768", "1024"])
                .required(false)
                .default_value("768"),
        )
        .get_matches();

    let bits: &String = matches
        .get_one::<String>("SECURITYLEVEL")
        .expect("");
    println!("{}", bits);
}
