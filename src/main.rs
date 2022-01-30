use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
        for arg in args {
            let mut xor_sum: u8 = 0;
            let mut add_sum: u16 = 0;
        for i in arg.chars() {
            add_sum = add_sum + i as u16;   
            xor_sum = xor_sum ^ i as u8;
        }
        println!("{}", add_sum);
        println!("{:>0width$X} {:>0width$X}", add_sum, xor_sum, width = 2);
        if arg.chars().last().unwrap() == '*' {
            if add_sum > 255{
                add_sum -= 256;
            let d = format!("{}{:X}", arg, add_sum);
            println!("{}", d)
            } else {
                let d = format!("{}{:X}", arg, add_sum);
                println!("{}", d)
            }
        } else if arg.chars().last().unwrap() == '^' {
                let d = format!("{}{:X}", arg, xor_sum);
                println!("{}", d) 
            }
        } else {
            eprintln!("Print ^ or * at the end of the line");
            process::exit(1);
            }
        }  
    
}    

