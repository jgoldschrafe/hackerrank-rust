use std::io;

#[allow(non_snake_case)]
fn maximumToys(prices: &mut Vec<u32>, k: u32) -> u32 {
    prices.sort();
    let mut num_toys = 0u32;
    let mut spent = 0u32;
    for p in prices.iter() {
        if spent + p > k {
            break
        }
        spent += p;
        num_toys += 1;
    }
    num_toys
}

fn main() {
    let mut params_buf = String::new();
    io::stdin().read_line(&mut params_buf).unwrap();
    let params: Vec<u32> = params_buf.trim_end()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let k = params[1];

    let mut prices_buf = String::new();
    io::stdin().read_line(&mut prices_buf).unwrap();
    let mut prices: Vec<u32> = prices_buf.trim_end()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", maximumToys(&mut prices, k));
}
