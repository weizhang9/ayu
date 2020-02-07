extern crate rand;

use clap::{clap_app, crate_version};

fn main() {
    println!("{:?}", run());
}

fn run() -> String {
    let args = clap_app!(Ayu =>
    (version: crate_version!())
    (author: "Wei Zhang <vivian_17@msn.cn>")
    (about: "Random password generator")
    (@arg LENGTH: -L --len  +required +takes_value "(REQUIRED) desired password LENGTH")
    (@arg CHARSET: -C --char +takes_value "(OPTIONAL) desired password CHARSET\n\
    Options include:\n\
    1) Alphanumeric + Special characters (default)\n\
    2) Alphanumeric\n\
    3) Alphabetic\n\
    4) Numberic")
    )
    .get_matches_safe().unwrap_or_else(|e| e.exit());

    let password_len = args.value_of("LENGTH").unwrap();
    if args.is_present("CHARSET") {
        let option = args.value_of("CHARSET").unwrap();
        generate_with_option(password_len, option.parse().unwrap())
    } else {
        generate(password_len)
    }
}

fn generate_with_option(len: &str, option: u8) -> String {
    use rand::Rng;
    const ALPHANUMSPEC: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789!()-.?[]_`~;:!@#$%^&*+=";
    const ALPHANUM: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789";
    const ALPHA: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz";
    const NUM: &[u8] = b"0123456789"; 

    let charset: &[u8] = match option {
        1 => ALPHANUMSPEC,
        2 => ALPHANUM,
        3 => ALPHA,
        4 => NUM,
        _ => ALPHANUMSPEC,
    };
    let mut rng = rand::thread_rng();

    let password: String = (0..len.parse().unwrap())
        .map(|_| {
            let idx = rng.gen_range(0, charset.len());
            charset[idx] as char
        })
        .collect();
    password
}

fn generate(len: &str) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789!()-.?[]_`~;:!@#$%^&*+=";

    let mut rng = rand::thread_rng();

    let password: String = (0..len.parse().unwrap())
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
}