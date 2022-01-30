use std::env;
use std::process;

fn main() {
    let args: String = env::args().nth(1).unwrap();

    let mut xor_sum: u16 = 0;
    let mut add_sum: u16 = 0;
    if args.chars().last().unwrap().to_string() == "*" {
        for i in args.chars() {
            add_sum = add_sum + i as u16
        }
        println!("add")
    } else if args.chars().last().unwrap().to_string() == "^" {
        for i in args.chars() {
            xor_sum = xor_sum + i as u16
        }
        println!("xor")

    } else {
        process::exit(0x0100);
    }    


}