use num::traits::FromPrimitive;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;

#[derive(Debug)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl fmt::Display for RockPaperScissors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RockPaperScissors::Rock => write!(f, "Rock"),
            RockPaperScissors::Paper => write!(f, "Paper"),
            RockPaperScissors::Scissors => write!(f, "Scissors"),
        }
    }
}

impl FromPrimitive for RockPaperScissors {
    fn from_i64(n: i64) -> Option<RockPaperScissors> {
        match n {
            0 => Some(RockPaperScissors::Rock),
            1 => Some(RockPaperScissors::Paper),
            2 => Some(RockPaperScissors::Scissors),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<RockPaperScissors> {
        match n {
            0 => Some(RockPaperScissors::Rock),
            1 => Some(RockPaperScissors::Paper),
            2 => Some(RockPaperScissors::Scissors),
            _ => None,
        }
    }
}

impl Distribution<RockPaperScissors> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RockPaperScissors {
        let n = rng.gen_range(0, 3);
        RockPaperScissors::from_u8(n).unwrap()
    }
}

fn beat_rock_paper_scissors(my_hand: RockPaperScissors, opp_hand: RockPaperScissors) -> bool {
    match (my_hand, opp_hand) {
        (RockPaperScissors::Rock, RockPaperScissors::Scissors) | 
        (RockPaperScissors::Paper, RockPaperScissors::Rock) | 
        (RockPaperScissors::Scissors, RockPaperScissors::Paper) => true,
        (_, _) => false,
    }
}

fn main() {
    let n: u8 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        s.trim().parse().unwrap()
    };
    let my_hand: RockPaperScissors = RockPaperScissors::from_u8(n).unwrap();
    let opp_hand: RockPaperScissors = rand::random();

    println!("My Hand: {}", my_hand);
    println!("Opponent's Hand: {}", opp_hand);
    println!("{}", beat_rock_paper_scissors(my_hand, opp_hand));
}