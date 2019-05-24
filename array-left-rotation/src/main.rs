use std::io;

fn main() {
    let mut params_buf = String::new();
    io::stdin().read_line(&mut params_buf);
    let params: Vec<u32> = params_buf
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (n, d) = (params[0], params[1]);

    let mut nums_buf = String::new();
    io::stdin().read_line(&mut nums_buf);
    let nums: Vec<u32> = nums_buf
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let rotated_nums: Vec<String> = (0..nums.len())
        .map(|i| nums[(i + d as usize) % nums.len()].to_string())
        .collect();
    println!("{}", rotated_nums.join(" "));
}
