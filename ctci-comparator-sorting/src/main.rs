use std::cmp::Ordering;
use std::io;

#[derive(Clone)]
#[derive(Eq)]
struct Player {
    name: String,
    score: u16,
}

impl Player {
    fn new() -> Player {
        Player {
            name: String::new(),
            score: 0,
        }
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score.cmp(&other.score) {
            Ordering::Equal => self.name.cmp(&other.name),
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
        }
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.score == other.score
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut n_buf = String::new();
    io::stdin().read_line(&mut n_buf);
    let n = n_buf.trim_end().parse().unwrap();

    let mut players = vec![Player::new(); n];
    for i in 0..n {
        let mut player_buf = String::new();
        io::stdin().read_line(&mut player_buf).unwrap();
        let fields: Vec<_> = player_buf.trim_end()
            .split_whitespace()
            .collect();

        players[i].name = String::from(fields[0]);
        players[i].score = fields[1].parse().unwrap();
    }

    players.sort();
    for player in players {
        println!("{} {}", player.name, player.score);
    }
}
