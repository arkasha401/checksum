use std::env;

fn main() {
    let parsed_line = env::args().skip(1).collect::<Vec<_>>();
    let mut sum: usize = 0;
    for i in parsed_line {
        for c in i.chars() {
            sum = i.len() + std::mem::size_of::<char>();
        }
    }
    println!("{:?}", sum);

}
