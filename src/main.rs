use std::env;

fn main() {
    let args: String = env::args().nth(1).unwrap();

    let mut xor_sum: u8 = 0;
    let mut add_sum: u16 = 0;
    for i in args.chars() {
        xor_sum = xor_sum ^ i as u8;
        add_sum = add_sum + i as u16;
        
    }
    println!("{:>0width$X} {:>0width$X}", xor_sum, add_sum, width = 2);
    


}
