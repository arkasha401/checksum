use std::env;

fn main() {
    let args: String = env::args().skip(1).collect();

    let mut xor_sum: u8 = 0;
    let mut add_sum: u16 = 0;
    for i in args.chars() {
        xor_sum = xor_sum ^ i as u8;
        add_sum = add_sum + i as u16;
    }
    println!("xor sum: {}; add_sum: {}", xor_sum, add_sum);
}
