use std::io;

#[allow(non_snake_case)]
fn isBalanced(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '{' | '(' | '[' => {
                stack.push(c);
            },
            '}' | ')' | ']' => {
                match stack.pop() {
                    None => {
                        return false;
                    },
                    Some(matched_c) => {
                        if (c == '}' && matched_c == '{') ||
                                (c == ')' && matched_c == '(') ||
                                (c == ']' && matched_c == '[') {
                            continue;
                        } else {
                            return false;
                        }
                    }
                }
            },
            _ => {
                panic!("Don't understand character: {}", c);
            }
        }
    }
    true
}

fn main() {
    let mut num_inputs = String::new();
    io::stdin().read_line(&mut num_inputs).unwrap();
    let num_inputs: u16 = num_inputs.trim_end().parse().unwrap();

    for _ in 0..num_inputs {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim_end();
        println!("{}", match isBalanced(input) {
            false => "NO",
            true => "YES",
        });
    }
}
