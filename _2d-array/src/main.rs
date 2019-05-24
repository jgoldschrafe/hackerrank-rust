use std::io;
use std::io::BufRead;

#[allow(non_snake_case)]
fn hourglassSum(arr: &Vec<Vec<i8>>) -> i8 {
    let height = arr.len();
    let width = arr[0].len();
    let max_height = height - 2;
    let max_width = width - 2;

    let mut max_sum = i8::min_value();
    for x in 0..max_width {
        for y in 0..max_height {
            let sum =
                arr[y][x] + arr[y][x + 1] + arr[y][x + 2] +
                arr[y + 1][x + 1] + 
                arr[y + 2][x] + arr[y + 2][x + 1] + arr[y + 2][x + 2];
            if sum > max_sum {
                max_sum = sum;
            }
        }
    }
    max_sum
}

fn main() {
    let arr: Vec<Vec<i8>> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .into_iter()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("{}", hourglassSum(&arr));
}
