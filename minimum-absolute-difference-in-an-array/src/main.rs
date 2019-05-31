use std::io;

#[allow(non_snake_case)]
fn minimumAbsoluteDifference(n: usize, arr: &mut [i32]) -> i32 {
    arr.sort();
    arr.windows(2).fold(std::i32::MAX, |min, els| {
        let diff = (els[0] - els[1]).abs();
        match diff < min {
            true => diff,
            false => min,
        }
    })
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim_end().parse().unwrap();

    let mut arr = String::new();
    io::stdin().read_line(&mut arr).unwrap();
    let mut arr: Vec<_> = arr.split_whitespace()
        .map(|x| x.parse().unwrap())
        .into_iter()
        .collect();

    println!("{}", minimumAbsoluteDifference(n, &mut arr));
}
