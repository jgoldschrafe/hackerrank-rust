use std::io;

#[allow(non_snake_case)]
fn countSwaps(a: &mut Vec<u32>) -> u32 {
    let mut swaps = 0;
    for i in 0..(a.len() - 1) {
        for j in (i + 1)..a.len() {
            if a[i] > a[j] {
                a.swap(i, j);
                swaps += 1;
            }
        }
    }
    swaps
}

fn main() {
    let mut _buf = String::new();
    io::stdin().read_line(&mut _buf).unwrap();

    let mut nums_buf = String::new();
    io::stdin().read_line(&mut nums_buf).unwrap();
    let mut nums: Vec<u32> = nums_buf
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let swaps = countSwaps(&mut nums);

    println!("Array is sorted in {} swaps.", swaps);
    println!("First Element: {}", nums.first().unwrap());
    println!("Last Element: {}", nums.last().unwrap());
}
