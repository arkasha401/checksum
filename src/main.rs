
fn main() {
    let strofa: &str = "Deutchfrog";
    let mut xor_sum = 0;
    let mut add_sum = 0;
    for i in strofa.chars(){
        xor_sum = xor_sum + i as u32;
        add_sum = add_sum ^ i as u32;
        println!("{} {}", xor_sum, add_sum);
    }

}
