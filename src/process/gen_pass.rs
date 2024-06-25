use crate::opts::GenPassOpts;
use rand::seq::SliceRandom;

const UPPER_CASE: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER_CASE: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(opts: &GenPassOpts) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if opts.uppercase {
        chars.extend_from_slice(UPPER_CASE);
        password.push(*UPPER_CASE.choose(&mut rng).unwrap());
    }
    if opts.lowecase {
        chars.extend_from_slice(LOWER_CASE);
        password.push(*LOWER_CASE.choose(&mut rng).unwrap());
    }
    if opts.number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).unwrap());
    }
    if opts.symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).unwrap());
    }
    for _ in 0..(opts.length - password.len() as u8) {
        let c = chars.choose(&mut rng).unwrap();
        password.push(*c);
    }

    password.shuffle(&mut rng);

    print!("{}", String::from_utf8(password)?);
    Ok(())
}
