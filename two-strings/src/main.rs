use std::collections::HashMap;
use std::io;

fn main() {
    let mut cases_buf = String::new();
    io::stdin().read_line(&mut cases_buf).unwrap();
    let cases: u8 = cases_buf.trim_end().parse().unwrap();

    'cases: for _ in 0..cases {
        let mut s1 = String::new();
        io::stdin().read_line(&mut s1).unwrap();

        let mut s2 = String::new();
        io::stdin().read_line(&mut s2).unwrap();

        let mut s1_chars: HashMap<char, u16> = HashMap::new();
        for c in s1.trim_end().chars() {
            let entry = s1_chars.entry(c).or_insert(0);
            *entry += 1;
        }

        for c in s2.trim_end().chars() {
            if s1_chars.contains_key(&c) {
                println!("{}", "YES");
                continue 'cases;
            }
        }
        println!("{}", "NO");
    }
}
