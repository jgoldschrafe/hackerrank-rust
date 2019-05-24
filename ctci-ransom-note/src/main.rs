use std::collections::HashMap;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut magazine_words: HashMap<&str, u16> = HashMap::new();
    let mut magazine_buf = String::new();
    io::stdin().read_line(&mut magazine_buf).unwrap();
    for word in magazine_buf.trim_end().split_whitespace().into_iter() {
        *(magazine_words.entry(word).or_insert(0)) += 1;
    }

    let mut note_buf = String::new();
    io::stdin().read_line(&mut note_buf).unwrap();
    for word in note_buf.trim_end().split_whitespace().into_iter() {
        let entry = magazine_words.entry(word).or_insert(0);
        if *entry == 0 {
            println!("No");
            return;
        }
        *entry -= 1;
    }
    println!("Yes");
}
