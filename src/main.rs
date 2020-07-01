use std::env;
use bs58;
use hex;
use std::process::exit;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("usage: bs58_to_hex <bs58>");
        exit(1);
    }
    let bs58_str = &args[1];
    let x = bs58::decode(bs58_str).into_vec().unwrap();
    println!("{}", hex::encode(x));
}
