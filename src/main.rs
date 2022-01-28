
fn main() {
    let strofa: &str = "Deutch frog";

    let mut xor_sum8: u8 = 0;
    let mut add_sum8: u16 = 0;
    for i in strofa.chars() {
        xor_sum8 = xor_sum8 ^ i as u8;
        add_sum8 = add_sum8 + i as u16;
    }
    println!("{} {}", xor_sum8, add_sum8)
}
