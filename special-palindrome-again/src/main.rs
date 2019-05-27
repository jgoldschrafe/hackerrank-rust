use std::io;

struct IterationState {
    pos: usize,
    num_before_pivot: u64,
    num_after_pivot: u64,
    pivot_pos: Option<usize>,
    wanted_char: char,
}

impl IterationState {
    fn new(begin_pos: usize, wanted_char: char) -> IterationState {
        IterationState {
            num_before_pivot: 0,
            num_after_pivot: 0,
            pivot_pos: None,
            pos: begin_pos,
            wanted_char,
        }
    }
}

#[allow(non_snake_case)]
fn substrCount(n: usize, s: &str) -> u64 {
    let chars: Vec<_> = s.chars().collect();
    let mut num_found = 0u64;

    let mut state = IterationState::new(0, chars[0]);

    while state.pos < n {
        if state.wanted_char == chars[state.pos] {
            match state.pivot_pos {
                None => {
                    state.num_before_pivot += 1;
                    num_found += state.num_before_pivot;
                },
                Some(pivot_pos) => {
                    state.num_after_pivot += 1;
                    if state.num_after_pivot <= state.num_before_pivot {
                        num_found += 1;
                    } else {
                        state = IterationState::new(pivot_pos, chars[pivot_pos]);
                    }
                }
            }
        } else {
            match state.pivot_pos {
                None => {
                    state.pivot_pos = Some(state.pos);
                },
                Some(pivot_pos) => {
                    state = IterationState::new(pivot_pos, chars[pivot_pos]);
                }
            }
        }

        // Increment the position unless we just set it
        if state.num_before_pivot + state.num_after_pivot > 0 {
            state.pos += 1;
        }

        // Drain remaining pivots
        if let Some(pivot_pos) = state.pivot_pos {
            if state.pos == n {
                state = IterationState::new(pivot_pos, chars[pivot_pos]);
            }
        }
    }

    num_found
}

fn main() {
    let mut len_buf = String::new();
    io::stdin().read_line(&mut len_buf).unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim_end();

    println!("{}", substrCount(s.len(), s));
}
