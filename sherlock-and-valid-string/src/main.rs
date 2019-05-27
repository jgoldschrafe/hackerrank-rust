use std::collections::HashMap;
use std::io;

fn isValid(s: &str) -> bool {
    let mut occurrences: HashMap<char, i16> = HashMap::new();
    for c in s.chars() {
        let entry = occurrences.entry(c).or_insert(0);
        *entry += 1;
    }

    let mut occurrence_freq: HashMap<i16, i16> = HashMap::new();
    for o in occurrences.values() {
        let entry = occurrence_freq.entry(*o).or_insert(0);
        *entry += 1;
    }

    let mut freqs: Vec<_> = occurrence_freq
        .keys()
        .map(|k| (*k, occurrence_freq[k]) )
        .collect();
    freqs.sort();
    match freqs.len() {
        1 => true,
        2 => (freqs[0].0 == 1 && freqs[0].1 == 1) ||
                (freqs[1].0 - freqs[0].0 == 1 && freqs[1].1 == 1),
        _ => false,
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let s = buf.trim_end();

    println!("{}", match isValid(s) {
        true => "YES",
        false => "NO",
    });
}
