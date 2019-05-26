use std::io;

#[allow(non_snake_case)]
fn alternatingCharacters(s: &str) -> u16 {
    let mut extra_chars = 0u16;
    let mut last: Option<char> = None;
    for c in s.chars() {
        if let Some(l) = last {
            if c == l {
                extra_chars += 1;
            }
        }

        last = Some(c);
    }
    extra_chars
}

fn main() {
    let mut n_buf = String::new();
    io::stdin().read_line(&mut n_buf).unwrap();
    let n: u8 = n_buf.trim_end().parse().unwrap();

    for _ in 0..n {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let s = buf.trim_end();

        println!("{}", alternatingCharacters(&s[..]));
    }
}
