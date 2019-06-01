use std::io;

#[allow(non_snake_case)]
fn luckBalance(k: usize, contests: &[[i32;2]]) -> i32 {
    let mut important_contests: Vec<i32> = vec![];
    let mut luck = 0;

    for c in contests {
        if c[1] == 0 {
            luck += c[0];
        } else {
            important_contests.push(c[0]);
        }
    }
    important_contests.sort();
    let num_mandatory = match important_contests.len() > k {
        true => important_contests.len() - k,
        false => 0,
    };

    for i in 0..num_mandatory {
        luck -= important_contests[i];
    }

    for i in num_mandatory..important_contests.len() {
        luck += important_contests[i];
    }

    luck
}

fn main() {
    let mut params = String::new();
    io::stdin().read_line(&mut params).unwrap();
    let params: Vec<usize> = params.split_whitespace()
        .map(|x| x.parse().unwrap())
        .into_iter()
        .collect();
    let (n, k) = (params[0], params[1]);

    let mut contests = vec![[0, 0]; n];
    for i in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let fields: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .into_iter()
            .collect();
        contests[i] = [fields[0], fields[1]];
    }

    println!("{}", luckBalance(k, &contests[..]));
}
