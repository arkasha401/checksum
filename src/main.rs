use std::env;
use std::process;

fn main() {
    let args: String = env::args().nth(1).unwrap();

    let mut xor_sum: u16 = 0;
    let mut add_sum: u16 = 8;
    for i in args.chars() {
        add_sum = add_sum + i as u16;   
        xor_sum = xor_sum + i as u16;
    }
    println!("{:>0width$X} {:>0width$X}", add_sum, xor_sum, width = 2);
    if args.chars().last().unwrap().to_string() == "*"{
        println!("add")
    } else if args.chars().last().unwrap().to_string() == "^" {
        println!("xor")
    } else {
        process::exit(0x0100)
    }
}    
