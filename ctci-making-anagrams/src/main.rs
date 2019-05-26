use std::collections::HashMap;
use std::io;

#[allow(non_snake_case)]
fn makeAnagram(a: &str, b: &str) -> i16 {
    let mut differences: HashMap<char, i16> = HashMap::new();
    for c in a.chars() {
        let entry = differences.entry(c).or_insert(0);
        *entry += 1;
    }
    for c in b.chars() {
        let entry = differences.entry(c).or_insert(0);
        *entry -= 1;
    }
    differences.values()
        .map(|n| n.abs())
        .fold(0, |acc, n| acc + n)
}

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a);

    let mut b = String::new();
    io::stdin().read_line(&mut b);

    println!("{}", makeAnagram(&a[..], &b[..]));
}
