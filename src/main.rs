use std::env;
use std::process;

fn main() {
    let args: String = env::args().nth(1).unwrap();

    let mut xor_sum: u8 = 0;
    let mut add_sum: u16 = 0;
    if args.chars().last().unwrap().to_string() == "*" {
        println!("add");
    } else if args.chars().last().unwrap().to_string() == "^" {
        println!("xor")

    } else {
        process::exit(0x0100);
    }    


}
