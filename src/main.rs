use std::env;
use std::process;

fn main() {
    let args: String = env::args().nth(1).unwrap();

    let mut xor_sum: u8 = 0;
    let mut add_sum: u16 = 0;
    for i in args.chars() {
        add_sum = add_sum + i as u16;   
        xor_sum = xor_sum + i as u8;
    }
    println!("{:>0width$X} {:>0width$X}", add_sum, xor_sum, width = 2);
    if args.chars().last().unwrap() == '*' {
        println!("add");
        let d = format!("{}{:x}", args, add_sum);
        println!("{}", d)
    } else if args.chars().last().unwrap() == '^' {
        println!("xor");
        let d = format!("{}{:x}", args, add_sum);
        println!("{}", d)
    } else {
        eprintln!("Print ^ or * at the end of the line");
        process::exit(1);
    }
}    
