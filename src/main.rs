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
    (@arg LENGTH: -L --len  +takes_value "[OPTIONAL] desired password LENGTH (default 18 characters)")
    (@arg CHARSET: -C --char +takes_value "[OPTIONAL] desired password CHARSET\n\
    Options include:\n\
    1) Alphanumeric + Special characters (default)\n\
    2) Alphanumeric\n\
    3) Alphabetic\n\
    4) Numberic")
    (@arg UPPERCASE: -U --uppercase "Return password in UPPERCASE, invalid if used with LOWERCASE flag -N")
    (@arg LOWERCASE: -N --lowercase "Return password in LOWERCASE, invalid if used with UPPERCASE flag -U")
    )
    .get_matches_safe().unwrap_or_else(|e| e.exit());

    let allcaps = args.is_present("UPPERCASE");
    let noncaps = args.is_present("LOWERCASE");
    if args.is_present("CHARSET") && args.is_present("LENGTH") {
        let option = args.value_of("CHARSET").unwrap();
        let password_len = args.value_of("LENGTH").unwrap();
        generate_with_option(password_len, option.parse().unwrap(), allcaps, noncaps)
    } else if args.is_present("LENGTH") {
        let password_len = args.value_of("LENGTH").unwrap();
        generate_with_option(password_len, 1, allcaps, noncaps)
    } else if args.is_present("CHARSET") {
        let option = args.value_of("CHARSET").unwrap();
        generate_with_option("18", option.parse().unwrap(), allcaps, noncaps)
    } else {
        generate_with_option("18", 1, allcaps, noncaps)
    }
}

fn generate_with_option(len: &str, option: u8, allcaps: bool, noncaps: bool) -> String {
    use rand::Rng;
    const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const SPECIAL: &[u8] = b"()-.?[]_`~;:!@#$%^&*+=";
    const NUM: &[u8] = b"0123456789";
    // const ALPHANUMSPEC: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    //                         abcdefghijklmnopqrstuvwxyz\
    //                         0123456789!()-.?[]_`~;:!@#$%^&*+=";
    // const ALPHANUM: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    //                     abcdefghijklmnopqrstuvwxyz\
    //                     0123456789";
    // const ALPHA: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    //                     abcdefghijklmnopqrstuvwxyz";
    // const NUM: &[u8] = b"0123456789"; 

    let allchars: &[u8] = &[UPPERCASE, LOWERCASE, SPECIAL, NUM].concat();
    let allchars_caps: &[u8] = &[UPPERCASE, SPECIAL, NUM].concat();
    let allchars_noncaps: &[u8] = &[LOWERCASE, SPECIAL, NUM].concat();
    let alphameric: &[u8] = &[UPPERCASE, LOWERCASE, NUM].concat();
    let alphameric_caps: &[u8] = &[UPPERCASE, NUM].concat();
    let alphameric_noncaps: &[u8] = &[LOWERCASE, NUM].concat();
    let alphabets: &[u8] = &[UPPERCASE, LOWERCASE].concat();

    let charset: &[u8] = match (option, allcaps, noncaps) {
        (1, true, true) => allchars,
        (1, true, false) => allchars_caps,
        (1, false, true) => allchars_noncaps,
        (1, false, false) => allchars,
        (2, true, true) => alphameric,
        (2, true, false) => alphameric_caps,
        (2, false, true) => alphameric_noncaps,
        (2, false, false) => alphameric,
        (3, true, true) => alphabets,
        (3, true, false) => UPPERCASE,
        (3, false, true) => LOWERCASE,
        (3, false, false) => alphabets,
        (4, _, _) => NUM,
        (_, true, false) => allchars_caps,
        (_, false, true) => allchars_noncaps,
        (_, _, _) => allchars,
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
